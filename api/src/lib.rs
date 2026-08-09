use mlua::{AnyUserData, Lua, MaybeSend, MaybeSync, ObjectLike, Table, UserData, Value};

use crate::{camera::camera_mod, components::register_component_classes, input::input_mod, runtime::runtime_mod, sprite::sprite_mod};

mod sprite;
mod camera;
mod components;
mod input;
mod runtime;

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
    runtime_mod(lua)?;
    register_component_classes(lua)?;

    let script = std::fs::read_to_string("main.luau")?;
    lua.load(&script).exec().unwrap();

    Ok(())
}

fn userdata_from_value<T: UserData + Clone + 'static>(value: mlua::Value, type_name: &str) -> mlua::Result<T> {
    match value {
        mlua::Value::UserData(ud) => Ok(ud.borrow::<T>()?.clone()),
        mlua::Value::Table(t) => {
            let inner: mlua::AnyUserData = t.get("_inner")?;
            Ok(inner.borrow::<T>()?.clone())
        }
        other => Err(mlua::Error::FromLuaConversionError {
            from: other.type_name(),
            to: type_name.to_string(),
            message: Some(format!("expected {} userdata or wrapper table", type_name)),
        }),
    }
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

    let class_table_idx = class_table.clone();
    meta.set("__index", lua.create_function(move |lua, (t, key): (Table, String)| {
        let inner: AnyUserData = t.get("_inner")?;

        if let Ok(Value::Function(f)) = class_table_idx.get::<Value>(key.clone()) {
            let inner = inner.clone();
            let bound = lua.create_function(move |_, (_self_table, args): (Table, mlua::Variadic<Value>)| {
                f.call::<Value>((inner.clone(), args))
            })?;
            return Ok(Value::Function(bound));
        }

        let val: Value = inner.get(key)?;
        if let Value::Function(f) = val {
            let inner = inner.clone();
            let bound = lua.create_function(move |_, (_self_table, args): (Table, mlua::Variadic<Value>)| {
                f.call::<Value>((inner.clone(), args))
            })?;
            return Ok(Value::Function(bound));
        }

        Ok(val)
    })?)?;

    meta.set("__newindex", lua.create_function(move |_, (t, key, val): (Table, String, Value)| {
        let inner: AnyUserData = t.get("_inner")?;
        inner.set(key, val)
    })?)?;

    wrapper.set_metatable(Some(meta)).unwrap();

    Ok(wrapper)
}
