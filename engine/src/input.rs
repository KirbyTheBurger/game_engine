use std::collections::HashSet;

use winit::keyboard::KeyCode;

use crate::Shared;

pub static INPUT_STATE: Shared<InputState> = Shared::new();

#[derive(Clone)]
pub struct InputState(HashSet<KeyCode>);

impl InputState {
    pub fn new() -> Self {
        Self(HashSet::new())
    }
    
    pub fn set_pressed(&mut self, key: KeyCode, pressed: bool) {
        if pressed {
            self.0.insert(key);
        } else {
            self.0.remove(&key);
        }
    }

    pub fn is_down(&self, key: KeyCode) -> bool {
        self.0.contains(&key)
    }
}

macro_rules! define_key_map {
    ($($short:literal => $winit:ident),* $(,)?) => {
        pub fn parse_key(s: &str) -> Option<KeyCode> {
            Some(match s {
                $($short => KeyCode::$winit,)*
                _ => return None,
            })
        }

        // pub fn key_to_str(k: KeyCode) -> Option<&'static str> {
        //     Some(match k {
        //         $(KeyCode::$winit => $short,)*
        //         _ => return None,
        //     })
        // }

        pub const KEY_NAMES: &[&str] = &[$($short),*];
    };
}

define_key_map! {
    "A" => KeyA, "B" => KeyB, "C" => KeyC, "D" => KeyD, "E" => KeyE, "F" => KeyF, "G" => KeyG,
    "H" => KeyH, "I" => KeyI, "J" => KeyJ, "K" => KeyK, "L" => KeyL, "M" => KeyM, "N" => KeyN,
    "O" => KeyO, "P" => KeyP, "Q" => KeyQ, "R" => KeyR, "S" => KeyS, "T" => KeyT, "U" => KeyU,
    "V" => KeyV, "W" => KeyW, "X" => KeyX, "Y" => KeyY, "Z" => KeyZ,
}
