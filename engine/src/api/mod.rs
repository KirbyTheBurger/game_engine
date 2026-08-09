use mlua::Lua;

use crate::{api::{camera::camera_mod, components::register_component_classes, input::input_mod, sprite::sprite_mod}, input::InputState};

pub mod input;
pub mod camera;
pub mod components;
mod sprite;

pub fn init(
    input_state: InputState,
    device: wgpu::Device,
    queue: wgpu::Queue,
    layout: wgpu::BindGroupLayout,
) -> mlua::Result<Lua>
{
    let mut lua = Lua::new();

    input_mod(input_state, &mut lua)?;
    camera_mod(&mut lua)?;
    sprite_mod(&mut lua, device, queue, layout)?;
    register_component_classes(&mut lua)?;

    let script = std::fs::read_to_string("main.luau")?;
    lua.load(&script).exec().unwrap();

    Ok(lua)
}
