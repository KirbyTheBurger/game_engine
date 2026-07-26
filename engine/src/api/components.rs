use derive_more::{Add, Sub};
use mlua::{AnyUserData, FromLua, Lua, UserData, userdata_impl};

#[derive(Debug, Clone, Copy, UserData, FromLua, Add, Sub)]
pub struct Transform {
    #[lua(get)]
    pub x: f32,
    #[lua(get)]
    pub y: f32,
}

#[userdata_impl]
impl Transform {
    #[lua(infallible, meta, name = "__call")]
    fn new(_self: AnyUserData, x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[lua(infallible, meta, name = "__add")]
    fn add(&self, rhs: &Transform) -> Transform {
        *self + *rhs
    }

    #[lua(infallible, meta, name = "__sub")]
    fn sub(&self, rhs: &Transform) -> Transform {
        *self - *rhs
    }
}

pub(super) fn register_component_classes(lua: &mut Lua) -> mlua::Result<()> {
    lua.globals().set("Transform", lua.create_proxy::<Transform>()?)?;

    Ok(())
}