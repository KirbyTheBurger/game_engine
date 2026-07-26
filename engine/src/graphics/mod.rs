use winit::event_loop::EventLoop;

use crate::graphics::app::App;

mod app;
mod state;
mod vertex;
mod texture;
pub mod instance;
pub mod camera;

pub fn init() -> anyhow::Result<()> {
    env_logger::init();

    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}