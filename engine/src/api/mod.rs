use mlua::Lua;

use crate::{Shared, api::{camera::camera_mod, components::register_component_classes, input::input_mod}, graphics::camera::CamInstances, input::InputState};

pub mod input;
pub mod camera;
pub mod components;

pub fn init(
    input_state: InputState,
    instances: Shared<CamInstances>
) -> mlua::Result<Lua>
{
    let mut lua = Lua::new();

    input_mod(input_state, &mut lua)?;
    camera_mod(&mut lua, instances)?;
    register_component_classes(&mut lua)?;

    let script = std::fs::read_to_string("test.luau")?;
    lua.load(&script).exec().unwrap();

    Ok(lua)
}
