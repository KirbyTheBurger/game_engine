use std::{fs, sync::Mutex};

use glam::{Vec2, Vec3};
use mlua::{FromLua, Lua, UserData, userdata_impl};

use crate::{Shared, api::components::Transform, graphics::instance::{Instance, TextureReg}};

pub fn sprite_mod(
    lua: &mut Lua,
    instances: Shared<Vec<Shared<Instance>>>,
    texture_reg: Shared<TextureReg>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    layout: wgpu::BindGroupLayout
) -> mlua::Result<()> {
    let texture_table = lua.create_table()?;
    let next_texture_id = Mutex::new(0);

    let new_texture = lua.create_function(move |_, path: String| {
        let id = *next_texture_id.lock().unwrap();
        let bytes = fs::read(path)?;
        texture_reg.get().load(&device, &queue, &layout, id, bytes.as_slice()).unwrap();
        *next_texture_id.lock().unwrap() += 1;

        Ok(Texture(id))
    })?;

    texture_table.set("new", new_texture)?;

    let sprite_table = lua.create_table()?;

    let new_sprite = lua.create_function(
        move |_, (x, y, texture): (f32, f32, Texture)| {
            let instance = Shared::new(Instance {
                position: Vec3 { x, y, z: 0.0 },
                rotation: 0.0,
                scale: Vec2 { x: 1.0, y: 1.0 },
                texture_id: texture.0,
            });

            instances.get().push(instance.clone());

            let sprite = Sprite {
                texture_id: texture.0,
                instance: instance,
                position: Transform { x, y },
                rotation: 0.0,
                scale: Transform { x: 0.0, y: 0.0 },
                zindex: 0.0,
            };

            Ok(sprite)
        }
    )?;

    sprite_table.set("new", new_sprite)?;

    lua.globals().set("Texture", texture_table)?;
    lua.globals().set("Sprite", sprite_table)?;

    Ok(())
}

#[derive(UserData)]
struct Sprite {
    #[lua(skip)]
    texture_id: i32,
    #[lua(skip)]
    instance: Shared<Instance>,
    #[lua(skip)]
    position: Transform,
    #[lua(skip)]
    rotation: f32,
    #[lua(skip)]
    scale: Transform,
    #[lua(skip)]
    zindex: f32,
}

#[userdata_impl]
impl Sprite {
    
}

#[derive(Clone, UserData, FromLua)]
struct Texture(#[lua(skip)] i32);
