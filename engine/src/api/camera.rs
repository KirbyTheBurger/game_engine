use mlua::{Lua, UserData, userdata_impl};

use crate::{Shared, api::components::Transform, graphics::camera::CamInstances};

pub fn camera_mod(
    lua: &mut Lua,
    cam_instances: Shared<CamInstances>
) -> mlua::Result<()> {
    let camera_table = lua.create_table()?;

    let new = lua.create_function(move |_, pos: Transform| {
        let instance_table = cam_instances.clone();
        let id = instance_table.get().next_id();

        let instance = LuauCamera {
            pos,
            id,
            instances: instance_table.clone(),
        };
        cam_instances.get().cameras.insert(id, pos);

        Ok(instance)
    })?;
    camera_table.set("create", new)?;

    lua.globals().set("Camera", camera_table)?;

    Ok(())
}

#[derive(Clone, UserData)]
pub struct LuauCamera {
    pos: Transform,
    #[lua(skip)]
    id: i32,
    #[lua(skip)]
    instances: Shared<CamInstances>,
}

#[userdata_impl]
impl LuauCamera {
    #[lua(infallible)]
    #[allow(non_snake_case)]
    fn setPrimary(&self) {
        self.instances.get().primary = Some(self.id);
    }
}
