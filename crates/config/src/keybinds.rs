//! Keybinds configuráveis (TOML) — spec §17.
//!
//! ```toml
//! [global]
//! save_project = "Ctrl+S"
//! undo = "Ctrl+Z"
//! [model]
//! extrude = "E"
//! ```
//! Ações desconhecidas no TOML são ignoradas (não quebra).

use std::collections::HashMap;
use std::fs;

use self::winit_keys::KeyCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Mods2 {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Binding {
    pub key: KeyCode,
    pub mods: Mods2,
}

/// Nomes de teclas físicas (layout-independente).
fn parse_key(name: &str) -> Option<KeyCode> {
    let n = name.trim();
    if n.len() == 1 {
        let c = n.chars().next()?.to_ascii_uppercase();
        return match c {
            'A'..='Z' => Some(match c {
                'A' => KeyCode::KeyA,
                'B' => KeyCode::KeyB,
                'C' => KeyCode::KeyC,
                'D' => KeyCode::KeyD,
                'E' => KeyCode::KeyE,
                'F' => KeyCode::KeyF,
                'G' => KeyCode::KeyG,
                'H' => KeyCode::KeyH,
                'I' => KeyCode::KeyI,
                'J' => KeyCode::KeyJ,
                'K' => KeyCode::KeyK,
                'L' => KeyCode::KeyL,
                'M' => KeyCode::KeyM,
                'N' => KeyCode::KeyN,
                'O' => KeyCode::KeyO,
                'P' => KeyCode::KeyP,
                'Q' => KeyCode::KeyQ,
                'R' => KeyCode::KeyR,
                'S' => KeyCode::KeyS,
                'T' => KeyCode::KeyT,
                'U' => KeyCode::KeyU,
                'V' => KeyCode::KeyV,
                'W' => KeyCode::KeyW,
                'X' => KeyCode::KeyX,
                'Y' => KeyCode::KeyY,
                _ => KeyCode::KeyZ,
            }),
            '0'..='9' => Some(match c {
                '0' => KeyCode::Digit0,
                '1' => KeyCode::Digit1,
                '2' => KeyCode::Digit2,
                '3' => KeyCode::Digit3,
                '4' => KeyCode::Digit4,
                '5' => KeyCode::Digit5,
                '6' => KeyCode::Digit6,
                '7' => KeyCode::Digit7,
                '8' => KeyCode::Digit8,
                _ => KeyCode::Digit9,
            }),
            _ => None,
        };
    }
    Some(match n {
        "F1" => KeyCode::F1,
        "F2" => KeyCode::F2,
        "F3" => KeyCode::F3,
        "F4" => KeyCode::F4,
        "F5" => KeyCode::F5,
        "F6" => KeyCode::F6,
        "F7" => KeyCode::F7,
        "F8" => KeyCode::F8,
        "F9" => KeyCode::F9,
        "F10" => KeyCode::F10,
        "F11" => KeyCode::F11,
        "F12" => KeyCode::F12,
        "Tab" => KeyCode::Tab,
        "Space" => KeyCode::Space,
        "Delete" | "Del" => KeyCode::Delete,
        "Backspace" => KeyCode::Backspace,
        "Home" => KeyCode::Home,
        "End" => KeyCode::End,
        "Escape" | "Esc" => KeyCode::Escape,
        "Enter" => KeyCode::Enter,
        _ => return None,
    })
}

/// `"Ctrl+Shift+Z"` -> Binding.
pub fn parse_binding(s: &str) -> Option<Binding> {
    let mut mods = Mods2::default();
    let mut key = None;
    for part in s.split('+') {
        match part.trim() {
            "Ctrl" | "Control" => mods.ctrl = true,
            "Shift" => mods.shift = true,
            "Alt" => mods.alt = true,
            other => key = parse_key(other),
        }
    }
    key.map(|key| Binding { key, mods })
}

/// Mapa ação -> binding, ex. `"model.extrude"`.
#[derive(Debug, Default, Clone)]
pub struct Keybinds {
    map: HashMap<String, Binding>,
}

impl Keybinds {
    pub fn load() -> Self {
        let mut kb = Self::defaults();
        for path in candidate_paths() {
            if let Ok(text) = fs::read_to_string(&path) {
                if let Ok(v) = toml::from_str::<toml::Value>(&text) {
                    if let Some(t) = v.as_table() {
                        for (section, inner) in t {
                            if let Some(m) = inner.as_table() {
                                for (action, val) in m {
                                    if let Some(s) = val.as_str() {
                                        if let Some(b) = parse_binding(s) {
                                            kb.map.insert(format!("{section}.{action}"), b);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                break;
            }
        }
        kb
    }

    /// Procura ação pelo (key, mods). Retorna ex. `"model.extrude"`.
    pub fn find(&self, key: KeyCode, mods: Mods2) -> Option<&str> {
        self.map
            .iter()
            .find(|(_, b)| b.key == key && b.mods == mods)
            .map(|(a, _)| a.as_str())
    }

    /// Defaults "Petunia" (iguais aos atalhos documentados na UI).
    pub fn defaults() -> Self {
        let pairs = [
            ("global.undo", "Ctrl+Z"),
            ("global.redo", "Ctrl+Shift+Z"),
            ("global.save_project", "Ctrl+S"),
            ("global.toggle_wireframe", "Z"),
            ("global.help", "H"),
            ("global.reset_camera", "Home"),
            ("global.toggle_projection", "O"),
            ("global.cycle_mode", "Tab"),
            ("model.select_vertex", "1"),
            ("model.select_face", "3"),
            ("model.select_edge", "2"),
            ("model.select_object", "4"),
            ("model.transform", "G"),
            ("model.rotate", "R"),
            ("model.scale", "S"),
            ("model.frame_selection", "F"),
            ("model.extrude", "E"),
            ("model.push_pull", "P"),
            ("model.inset", "I"),
            ("model.bevel", "Ctrl+B"),
            ("model.subdivide", "W"),
            ("model.merge", "M"),
            ("model.mirror", "Ctrl+M"),
            ("model.primitives", "A"),
            ("model.draw_profile", "Shift+P"),
            ("model.slice", "Shift+K"),
            ("model.knife", "K"),
            ("model.loop_cut", "Ctrl+R"),
            ("model.connect", "Ctrl+J"),
            ("model.dissolve", "X"),
            ("model.invert_selection", "Ctrl+I"),
            ("model.select_linked", "L"),
            ("model.delete", "Delete"),
            ("paint.paint", "B"),
        ];
        let mut kb = Self::default();
        for (a, s) in pairs {
            if let Some(b) = parse_binding(s) {
                kb.map.insert(a.to_string(), b);
            }
        }
        kb
    }
}

fn candidate_paths() -> Vec<String> {
    let mut v = vec![
        "assets/keybinds/petunia.toml".to_string(),
        "keybinds/petunia.toml".to_string(),
    ];
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            v.push(
                dir.join("assets/keybinds/petunia.toml")
                    .to_string_lossy()
                    .to_string(),
            );
        }
    }
    v
}

// Os tipos winit são convertidos em `app`; aqui ficam tipos próprios para
// o crate `config` não depender de `winit`.
pub mod winit_keys {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(missing_docs)]
    pub enum KeyCode {
        KeyA,
        KeyB,
        KeyC,
        KeyD,
        KeyE,
        KeyF,
        KeyG,
        KeyH,
        KeyI,
        KeyJ,
        KeyK,
        KeyL,
        KeyM,
        KeyN,
        KeyO,
        KeyP,
        KeyQ,
        KeyR,
        KeyS,
        KeyT,
        KeyU,
        KeyV,
        KeyW,
        KeyX,
        KeyY,
        KeyZ,
        Digit0,
        Digit1,
        Digit2,
        Digit3,
        Digit4,
        Digit5,
        Digit6,
        Digit7,
        Digit8,
        Digit9,
        F1,
        F2,
        F3,
        F4,
        F5,
        F6,
        F7,
        F8,
        F9,
        F10,
        F11,
        F12,
        Tab,
        Space,
        Delete,
        Backspace,
        Home,
        End,
        Escape,
        Enter,
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    #[allow(missing_docs)]
    pub struct Mods {
        pub ctrl: bool,
        pub shift: bool,
        pub alt: bool,
    }
}
