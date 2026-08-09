use std::sync::Arc;

use engine::graphics::state::State;
use winit::{event_loop::EventLoop, application::ApplicationHandler, event::{KeyEvent, WindowEvent}, keyboard::PhysicalKey, window::Window};


fn main() {
    env_logger::init();

    let event_loop = EventLoop::with_user_event().build().unwrap();
    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}

pub struct App {
    pub state: Option<State>,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: None,
        }
    }
}

impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = Window::default_attributes();
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        self.state = Some(pollster::block_on(State::new(window)).unwrap());

        let state = self.state.as_mut().unwrap();
        api::init(
            &mut state.lua,
            state.device.clone(),
            state.queue.clone(),
            state.texture_layout.clone(),
        ).unwrap();
    }

    fn user_event(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop, event: State) {
        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    )
    {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                state.update();
                match state.render() {
                    Ok(_) => {},
                    Err(e) => {
                        log::error!("{e}");
                        event_loop.exit();
                    }
                }
            },
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => state.handle_key(code, key_state.is_pressed()),
            _ => {},
        }
    }
}
