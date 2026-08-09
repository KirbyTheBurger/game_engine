use mlua::Lua;

use engine::input::{INPUT_STATE, KEY_NAMES, parse_key};

pub fn input_mod(lua: &mut Lua) -> mlua::Result<()> {
    let input = lua.create_table()?;
    let key_code_table = lua.create_table()?;

    for key in KEY_NAMES {
        key_code_table.set(*key, *key)?;
    }

    input.set("KeyCode", key_code_table)?;

    let is_pressed = lua.create_function(move |_, key: String| {
        Ok(INPUT_STATE.get().is_down(parse_key(&key).unwrap()))
    })?;
    input.set("isPressed", is_pressed)?;

    lua.globals().set("Input", input)?;

    Ok(())
}
