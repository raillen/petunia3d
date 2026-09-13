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

impl Binding {
    pub fn to_shortcut_string(&self) -> String {
        let mut parts = Vec::new();
        if self.mods.ctrl {
            parts.push("Ctrl");
        }
        if self.mods.shift {
            parts.push("Shift");
        }
        if self.mods.alt {
            parts.push("Alt");
        }
        parts.push(self.key.as_str());
        parts.join("+")
    }
}

/// Mapa ação -> binding, ex. `"model.extrude"`.
#[derive(Debug, Default, Clone)]
pub struct Keybinds {
    map: HashMap<String, Binding>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KeymapProfileInfo {
    pub id: String,
    pub name: String,
    pub description: String,
}

impl Keybinds {
    /// Retorna a lista de todos os 8 perfis canônicos disponíveis.
    pub fn available_profiles() -> Vec<KeymapProfileInfo> {
        vec![
            KeymapProfileInfo {
                id: "petunia-default".into(),
                name: "Petunia Padrão".into(),
                description: "Atalhos canônicos com acesso direto a ferramentas e navegação ágil"
                    .into(),
            },
            KeymapProfileInfo {
                id: "petunia-simple".into(),
                name: "Petunia Simples".into(),
                description:
                    "Atalhos minimalistas focados em modelagem rápida sem combinações complexas"
                        .into(),
            },
            KeymapProfileInfo {
                id: "petunia-notebook".into(),
                name: "Petunia Notebook".into(),
                description: "Otimizado para laptops sem teclado numérico dedicado".into(),
            },
            KeymapProfileInfo {
                id: "blender".into(),
                name: "Blender (Oficial)".into(),
                description:
                    "Mapeamento 100% fiel ao padrão do Blender (G/R/S, E, I, Ctrl+B, Shift+A, Tab)"
                        .into(),
            },
            KeymapProfileInfo {
                id: "blender-notebook".into(),
                name: "Blender Notebook".into(),
                description: "Padrão Blender adaptado para laptops sem teclado numérico".into(),
            },
            KeymapProfileInfo {
                id: "maya".into(),
                name: "Autodesk Maya".into(),
                description:
                    "Padrão Maya (Q/W/E/R para Seleção, Mover, Rotacionar e Escalar, F para Frame)"
                        .into(),
            },
            KeymapProfileInfo {
                id: "3ds-max".into(),
                name: "Autodesk 3ds Max".into(),
                description:
                    "Padrão 3ds Max (Q/W/E/R, Z para Zoom Extents, 1/2/4 para sub-objetos)".into(),
            },
            KeymapProfileInfo {
                id: "cinema-4d".into(),
                name: "Maxon Cinema 4D".into(),
                description: "Padrão Cinema 4D (E para Mover, R para Rotacionar, T para Escalar)"
                    .into(),
            },
        ]
    }

    /// Carrega um perfil específico pelo ID buscando em assets/keymaps/{id}.toml.
    pub fn load_profile(profile_id: &str) -> Self {
        let mut kb = Self::defaults();
        let candidate_paths = [
            format!("assets/keymaps/{profile_id}.toml"),
            format!("keymaps/{profile_id}.toml"),
            format!("assets/keybinds/{profile_id}.toml"),
            format!("keybinds/{profile_id}.toml"),
        ];

        for path in candidate_paths {
            if let Ok(text) = fs::read_to_string(&path) {
                if let Ok(v) = toml::from_str::<toml::Value>(&text) {
                    if let Some(t) = v.as_table() {
                        for (section, inner) in t {
                            if section == "profile" {
                                continue;
                            }
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

    pub fn load() -> Self {
        Self::load_profile("petunia-default")
    }

    /// Procura ação pelo (key, mods). Retorna ex. `"model.extrude"`.
    pub fn find(&self, key: KeyCode, mods: Mods2) -> Option<&str> {
        self.map
            .iter()
            .find(|(_, b)| b.key == key && b.mods == mods)
            .map(|(a, _)| a.as_str())
    }

    /// Retorna a representação textual do atalho para uma ação (ex: `"model.extrude"` -> `"E"`).
    pub fn shortcut_for(&self, action: &str) -> Option<String> {
        self.map.get(action).map(|b| b.to_shortcut_string())
    }

    /// Retorna a lista de todas as ações e seus atalhos formatados como string, ordenados.
    pub fn all_bindings(&self) -> Vec<(String, String)> {
        let mut list: Vec<(String, String)> = self
            .map
            .iter()
            .map(|(action, binding)| (action.clone(), binding.to_shortcut_string()))
            .collect();
        list.sort_by(|a, b| a.0.cmp(&b.0));
        list
    }

    /// Detecta conflitos de atalho (duas ações diferentes usando a mesma combinação de teclas).
    pub fn detect_conflicts(&self) -> Vec<(String, String, String)> {
        let mut conflicts = Vec::new();
        let items: Vec<(&String, &Binding)> = self.map.iter().collect();

        for i in 0..items.len() {
            for j in (i + 1)..items.len() {
                let (act_a, bind_a) = items[i];
                let (act_b, bind_b) = items[j];
                if bind_a == bind_b {
                    conflicts.push((act_a.clone(), act_b.clone(), bind_a.to_shortcut_string()));
                }
            }
        }
        conflicts
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
            ("model.select_object", "0"),
            ("model.transform", "G"),
            ("model.rotate", "R"),
            ("model.scale", "S"),
            ("model.frame_selection", "F"),
            ("model.extrude", "E"),
            ("model.extrude_individual", "Alt+E"),
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

    impl KeyCode {
        pub fn as_str(&self) -> &'static str {
            match self {
                KeyCode::KeyA => "A",
                KeyCode::KeyB => "B",
                KeyCode::KeyC => "C",
                KeyCode::KeyD => "D",
                KeyCode::KeyE => "E",
                KeyCode::KeyF => "F",
                KeyCode::KeyG => "G",
                KeyCode::KeyH => "H",
                KeyCode::KeyI => "I",
                KeyCode::KeyJ => "J",
                KeyCode::KeyK => "K",
                KeyCode::KeyL => "L",
                KeyCode::KeyM => "M",
                KeyCode::KeyN => "N",
                KeyCode::KeyO => "O",
                KeyCode::KeyP => "P",
                KeyCode::KeyQ => "Q",
                KeyCode::KeyR => "R",
                KeyCode::KeyS => "S",
                KeyCode::KeyT => "T",
                KeyCode::KeyU => "U",
                KeyCode::KeyV => "V",
                KeyCode::KeyW => "W",
                KeyCode::KeyX => "X",
                KeyCode::KeyY => "Y",
                KeyCode::KeyZ => "Z",
                KeyCode::Digit0 => "0",
                KeyCode::Digit1 => "1",
                KeyCode::Digit2 => "2",
                KeyCode::Digit3 => "3",
                KeyCode::Digit4 => "4",
                KeyCode::Digit5 => "5",
                KeyCode::Digit6 => "6",
                KeyCode::Digit7 => "7",
                KeyCode::Digit8 => "8",
                KeyCode::Digit9 => "9",
                KeyCode::F1 => "F1",
                KeyCode::F2 => "F2",
                KeyCode::F3 => "F3",
                KeyCode::F4 => "F4",
                KeyCode::F5 => "F5",
                KeyCode::F6 => "F6",
                KeyCode::F7 => "F7",
                KeyCode::F8 => "F8",
                KeyCode::F9 => "F9",
                KeyCode::F10 => "F10",
                KeyCode::F11 => "F11",
                KeyCode::F12 => "F12",
                KeyCode::Tab => "Tab",
                KeyCode::Space => "Space",
                KeyCode::Delete => "Delete",
                KeyCode::Backspace => "Backspace",
                KeyCode::Home => "Home",
                KeyCode::End => "End",
                KeyCode::Escape => "Escape",
                KeyCode::Enter => "Enter",
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    #[allow(missing_docs)]
    pub struct Mods {
        pub ctrl: bool,
        pub shift: bool,
        pub alt: bool,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_8_keymap_profiles_load_validly() {
        let profiles = Keybinds::available_profiles();
        assert_eq!(profiles.len(), 8);

        for p in profiles {
            let kb = Keybinds::load_profile(&p.id);
            let bindings = kb.all_bindings();
            assert!(
                !bindings.is_empty(),
                "Profile {} must contain bindings",
                p.id
            );
        }
    }

    #[test]
    fn test_conflict_detection_logic() {
        let mut kb = Keybinds::default();
        kb.map
            .insert("model.extrude".into(), parse_binding("E").unwrap());
        kb.map
            .insert("model.push_pull".into(), parse_binding("E").unwrap());

        let conflicts = kb.detect_conflicts();
        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0].2, "E");
    }
}
