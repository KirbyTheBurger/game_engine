use engine::graphics::state::UPDATE_FNS;
use mlua::Lua;

pub fn runtime_mod(lua: &mut Lua) -> mlua::Result<()> {
    let runtime_table = lua.create_table()?;

    let update = lua.create_function(|_, f: mlua::Function| {
        UPDATE_FNS.get().push(f);
        Ok(())
    })?;

    runtime_table.set("OnUpdate", update)?;
    lua.globals().set("Runtime", runtime_table)?;

    Ok(())
}