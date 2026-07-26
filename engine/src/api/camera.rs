use mlua::{Lua, UserData, userdata_impl};

use crate::{Shared, api::components::Transform, graphics::camera::CamInstances};

pub fn camera_mod(
    lua: &mut Lua,
    cam_instances: Shared<CamInstances>
) -> mlua::Result<()> {
    let camera_table = lua.create_table()?;

    let new = lua.create_function(move |_, position: Transform| {
        let instance_table = cam_instances.clone();
        let id = instance_table.get().next_id();

        let instance = LuauCamera {
            position,
            id,
            instances: instance_table.clone(),
        };
        cam_instances.get().cameras.insert(id, position);

        Ok(instance)
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
    #[lua(skip)]
    instances: Shared<CamInstances>,
}

#[userdata_impl]
impl LuauCamera {
    #[lua(infallible, name = "setPrimary")]
    fn set_primary(&self) {
        self.instances.get().primary = Some(self.id);
    }

    #[lua(setter, name = "position", infallible)]
    fn set_pos(&mut self, pos: Transform) {
        self.position = pos;
        self.instances.get().cameras.insert(self.id, pos);
    }
}
