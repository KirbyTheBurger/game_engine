use mlua::{Lua, UserData, userdata_impl};

use crate::{api::components::Transform, graphics::camera::CAM_INSTANCES};

pub fn camera_mod(
    lua: &mut Lua,
) -> mlua::Result<()> {
    let camera_table = lua.create_table()?;

    let new = lua.create_function(move |_, position: Transform| {
        let mut cam_instances = CAM_INSTANCES.get();
        let id = cam_instances.next_id();

        let cam = LuauCamera {
            position,
            id,
        };
        cam_instances.cameras.insert(id, position);

        Ok(cam)
    })?;
    camera_table.set("new", new)?;

    lua.globals().set("Camera", camera_table)?;

    Ok(())
}

#[derive(Clone, UserData)]
pub struct LuauCamera {
    #[lua(get)]
    position: Transform,
    #[lua(skip)]
    id: i32,
}

#[userdata_impl]
impl LuauCamera {
    #[lua(infallible, name = "setPrimary")]
    fn set_primary(&self) {
        CAM_INSTANCES.get().primary = Some(self.id);
    }

    #[lua(setter, name = "position", infallible)]
    fn set_pos(&mut self, pos: Transform) {
        self.position = pos;
        CAM_INSTANCES.get().cameras.insert(self.id, pos);
    }
}
