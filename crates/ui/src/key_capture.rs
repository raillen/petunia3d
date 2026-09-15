//! Native key-combination capture (`keymap-capture` feature, P1-19).
//!
//! `egui_hotkey` 0.2 pins egui 0.19, which would drag a whole duplicate egui
//! family into the tree for a single capture widget. This adapter implements
//! the same contract natively on egui 0.36 input events: the Petunia keymap
//! stays authoritative, the widget only captures and reports. Revisit if
//! upstream releases an egui 0.36-compatible version.

/// A captured key combination: logical key plus modifiers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CapturedCombo {
    /// Logical key pressed.
    pub key: egui::Key,
    /// Shift held.
    pub shift: bool,
    /// Control held.
    pub ctrl: bool,
    /// Alt held.
    pub alt: bool,
}

impl std::fmt::Display for CapturedCombo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.ctrl {
            write!(f, "Ctrl+")?;
        }
        if self.shift {
            write!(f, "Shift+")?;
        }
        if self.alt {
            write!(f, "Alt+")?;
        }
        write!(f, "{:?}", self.key)
    }
}

/// Key-capture button. Shows `current` (e.g. `"Ctrl+Z"`); click arms it
/// ("press keys…"); the next non-repeat key press is returned as
/// [`CapturedCombo`]. `Escape` disarms without capturing.
pub fn key_capture(ui: &mut egui::Ui, id: egui::Id, current: &str) -> Option<CapturedCombo> {
    let armed_id = id.with("armed");
    let armed = ui.data_mut(|data| data.get_temp::<bool>(armed_id).unwrap_or(false));
    let mut captured = None;
    if armed {
        let events: Vec<egui::Event> = ui.input(|i| i.events.clone());
        for event in events {
            if let egui::Event::Key {
                key,
                pressed: true,
                repeat: false,
                modifiers,
                ..
            } = event
            {
                if key == egui::Key::Escape {
                    break;
                }
                captured = Some(CapturedCombo {
                    key,
                    shift: modifiers.shift,
                    ctrl: modifiers.ctrl,
                    alt: modifiers.alt,
                });
                break;
            }
        }
        if captured.is_some() || ui.input(|i| i.key_pressed(egui::Key::Escape)) {
            ui.data_mut(|data| data.insert_temp(armed_id, false));
        }
        let _ = ui.button("press keys… (Esc cancels)");
    } else if ui.button(current).clicked() {
        ui.data_mut(|data| data.insert_temp(armed_id, true));
    }
    captured
}

/// Applies a captured combo to the Petunia keymap for `action`.
///
/// Maps the logical `egui::Key` onto the physical keycodes the keymap owns
/// (letters, digits, F-keys, Tab/Space/Delete/Backspace/Home/End/Escape/Enter).
/// Returns `false` (and changes nothing) for keys outside that subset.
pub fn apply_captured(
    state: &mut petunia_core::AppState,
    action: &str,
    combo: CapturedCombo,
) -> bool {
    use petunia_config::keybinds::winit_keys::KeyCode;
    let key = match combo.key {
        egui::Key::A => KeyCode::KeyA,
        egui::Key::B => KeyCode::KeyB,
        egui::Key::C => KeyCode::KeyC,
        egui::Key::D => KeyCode::KeyD,
        egui::Key::E => KeyCode::KeyE,
        egui::Key::F => KeyCode::KeyF,
        egui::Key::G => KeyCode::KeyG,
        egui::Key::H => KeyCode::KeyH,
        egui::Key::I => KeyCode::KeyI,
        egui::Key::J => KeyCode::KeyJ,
        egui::Key::K => KeyCode::KeyK,
        egui::Key::L => KeyCode::KeyL,
        egui::Key::M => KeyCode::KeyM,
        egui::Key::N => KeyCode::KeyN,
        egui::Key::O => KeyCode::KeyO,
        egui::Key::P => KeyCode::KeyP,
        egui::Key::Q => KeyCode::KeyQ,
        egui::Key::R => KeyCode::KeyR,
        egui::Key::S => KeyCode::KeyS,
        egui::Key::T => KeyCode::KeyT,
        egui::Key::U => KeyCode::KeyU,
        egui::Key::V => KeyCode::KeyV,
        egui::Key::W => KeyCode::KeyW,
        egui::Key::X => KeyCode::KeyX,
        egui::Key::Y => KeyCode::KeyY,
        egui::Key::Z => KeyCode::KeyZ,
        egui::Key::Num0 => KeyCode::Digit0,
        egui::Key::Num1 => KeyCode::Digit1,
        egui::Key::Num2 => KeyCode::Digit2,
        egui::Key::Num3 => KeyCode::Digit3,
        egui::Key::Num4 => KeyCode::Digit4,
        egui::Key::Num5 => KeyCode::Digit5,
        egui::Key::Num6 => KeyCode::Digit6,
        egui::Key::Num7 => KeyCode::Digit7,
        egui::Key::Num8 => KeyCode::Digit8,
        egui::Key::Num9 => KeyCode::Digit9,
        egui::Key::F1 => KeyCode::F1,
        egui::Key::F2 => KeyCode::F2,
        egui::Key::F3 => KeyCode::F3,
        egui::Key::F4 => KeyCode::F4,
        egui::Key::F5 => KeyCode::F5,
        egui::Key::F6 => KeyCode::F6,
        egui::Key::F7 => KeyCode::F7,
        egui::Key::F8 => KeyCode::F8,
        egui::Key::F9 => KeyCode::F9,
        egui::Key::F10 => KeyCode::F10,
        egui::Key::F11 => KeyCode::F11,
        egui::Key::F12 => KeyCode::F12,
        egui::Key::Tab => KeyCode::Tab,
        egui::Key::Space => KeyCode::Space,
        egui::Key::Delete => KeyCode::Delete,
        egui::Key::Backspace => KeyCode::Backspace,
        egui::Key::Home => KeyCode::Home,
        egui::Key::End => KeyCode::End,
        egui::Key::Escape => KeyCode::Escape,
        egui::Key::Enter => KeyCode::Enter,
        _ => return false,
    };
    state.ui.keybinds.set_binding(
        action,
        petunia_config::keybinds::Binding {
            key,
            mods: petunia_config::keybinds::Mods2 {
                ctrl: combo.ctrl,
                shift: combo.shift,
                alt: combo.alt,
            },
        },
    );
    state.mark_dirty();
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_renders_disarmed_without_panic() {
        let ctx = egui::Context::default();
        ctx.run_ui(egui::RawInput::default(), |ui| {
            let id = egui::Id::new("capture-test");
            assert_eq!(key_capture(ui, id, "Ctrl+Z"), None);
        })
        .textures_delta
        .clear();
    }

    #[test]
    fn armed_capture_reads_key_event() {
        let ctx = egui::Context::default();
        let id = egui::Id::new("capture-armed");
        ctx.data_mut(|data| data.insert_temp(id.with("armed"), true));
        let mut input = egui::RawInput::default();
        input.events.push(egui::Event::Key {
            key: egui::Key::S,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers {
                ctrl: true,
                ..Default::default()
            },
        });
        let mut found = None;
        ctx.run_ui(input, |ui| {
            found = key_capture(ui, id, "…");
        })
        .textures_delta
        .clear();
        assert_eq!(
            found,
            Some(CapturedCombo {
                key: egui::Key::S,
                shift: false,
                ctrl: true,
                alt: false,
            })
        );
        // Disarmed afterwards.
        assert_eq!(
            ctx.data_mut(|data| data.get_temp::<bool>(id.with("armed"))),
            Some(false)
        );
    }

    #[test]
    fn combo_display_orders_modifiers() {
        let combo = CapturedCombo {
            key: egui::Key::Z,
            shift: true,
            ctrl: true,
            alt: false,
        };
        assert_eq!(combo.to_string(), "Ctrl+Shift+Z");
    }

    #[test]
    fn apply_writes_binding_and_rejects_unsupported() {
        let mut state = petunia_core::AppState::new("en");
        let combo = CapturedCombo {
            key: egui::Key::G,
            shift: false,
            ctrl: true,
            alt: false,
        };
        assert!(apply_captured(&mut state, "model.select_linked", combo));
        assert_eq!(
            state
                .ui
                .keybinds
                .shortcut_for("model.select_linked")
                .as_deref(),
            Some("Ctrl+G")
        );
        let arrow = CapturedCombo {
            key: egui::Key::ArrowLeft,
            shift: false,
            ctrl: false,
            alt: false,
        };
        assert!(!apply_captured(&mut state, "model.select_linked", arrow));
    }
}
