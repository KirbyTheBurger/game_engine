use mlua::Lua;

use crate::{Shared, api::{camera::camera_mod, components::register_component_classes, input::input_mod, sprite::sprite_mod}, graphics::{camera::CamInstances, instance::{Instance, TextureReg}}, input::InputState};

pub mod input;
pub mod camera;
pub mod components;
mod sprite;

pub fn init(
    input_state: InputState,
    cam_instances: Shared<CamInstances>,
    instances: Shared<Vec<Shared<Instance>>>,
    texture_reg: Shared<TextureReg>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    layout: wgpu::BindGroupLayout,
) -> mlua::Result<Lua>
{
    let mut lua = Lua::new();

    input_mod(input_state, &mut lua)?;
    camera_mod(&mut lua, cam_instances)?;
    sprite_mod(&mut lua, instances, texture_reg, device, queue, layout)?;
    register_component_classes(&mut lua)?;

    let script = std::fs::read_to_string("main.luau")?;
    lua.load(&script).exec().unwrap();

    Ok(lua)
}
