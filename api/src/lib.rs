use mlua::{AnyUserData, Lua, MaybeSend, MaybeSync, ObjectLike, Table, UserData, Value};

use crate::{camera::camera_mod, components::register_component_classes, input::input_mod, sprite::sprite_mod};

mod sprite;
mod camera;
mod components;
mod input;

pub fn init(
    lua: &mut Lua,
    device: wgpu::Device,
    queue: wgpu::Queue,
    layout: wgpu::BindGroupLayout,
) -> mlua::Result<()>
{
    input_mod(lua)?;
    camera_mod(lua)?;
    sprite_mod(lua, device, queue, layout)?;
    register_component_classes(lua)?;

    let script = std::fs::read_to_string("main.luau")?;
    lua.load(&script).exec().unwrap();

    Ok(())
}

fn wrap_instance<T: UserData + MaybeSend + MaybeSync + 'static>(
    lua: &Lua,
    inner: T,
    class_table: &Table,
) -> mlua::Result<Table> {
    let userdata = lua.create_userdata(inner)?;
    let wrapper = lua.create_table()?;
    wrapper.set("_inner", userdata)?;

    let meta = lua.create_table()?;
    let class_table = class_table.clone();
    meta.set("__index", lua.create_function(
        move |lua, (t, key): (Table, String)| {
            if let Ok(Value::Function(f)) = class_table.get::<Value>(key.clone()) {
                return Ok(Value::Function(f));
            }

            let inner: AnyUserData = t.get("_inner")?;
            let native_val: Value = inner.get(key)?;

            if let Value::Function(native_fn) = native_val {
                let bound = lua.create_function(move |_, (_self_table, args): (Table, mlua::Variadic<Value>)| {
                    native_fn.call::<Value>((inner.clone(), args))
                })?;
                return Ok(Value::Function(bound));
            }

            Ok(native_val)
        }
    )?)?;
    wrapper.set_metatable(Some(meta)).unwrap();

    Ok(wrapper)
}
