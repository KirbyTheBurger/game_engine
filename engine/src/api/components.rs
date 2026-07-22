use mlua::{AnyUserData, FromLua, Lua, UserData, userdata_impl};

#[derive(Debug, Clone, Copy, UserData, FromLua)]
pub struct Transform {
    #[lua(get, set)]
    pub x: f32,
    #[lua(get, set)]
    pub y: f32,
}

#[userdata_impl]
impl Transform {
    #[lua(infallible, meta)]
    fn __call(_this: AnyUserData, x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub(super) fn register_component_classes(lua: &mut Lua) -> mlua::Result<()> {
    lua.globals().set("Transform", lua.create_proxy::<Transform>()?)?;

    Ok(())
}