//! Editable tool properties driving the same transaction as viewport transforms.
use egui::{Key, Modifiers};
use glam::Vec3;
use petunia_core::{AppState, ModalConstraint, ModalKind};

const SESSION_ID: &str = "tool.property_fields";

#[derive(Clone)]
struct FieldSession {
    kind: ModalKind,
    values: [String; 3],
    active: bool,
    error: Option<String>,
}

impl FieldSession {
    fn new(kind: ModalKind, state: &AppState) -> Self {
        let values = match kind {
            ModalKind::Scale => [1.0; 3],
            ModalKind::Extrude => [state.extrude_dist, 0.0, 0.0],
            ModalKind::Inset => [state.inset_factor, 0.0, 0.0],
            ModalKind::Bevel => [state.bevel_amount, 0.0, 0.0],
            ModalKind::PushPull => [state.push_dist, 0.0, 0.0],
            _ => [0.0; 3],
        };
        Self {
            kind,
            values: values.map(|value| value.to_string()),
            active: false,
            error: None,
        }
    }

    fn parsed(&self) -> Result<Vec3, String> {
        let count = if transform(self.kind) { 3 } else { 1 };
        let mut values = [0.0; 3];
        for (index, slot) in values.iter_mut().enumerate().take(count) {
            *slot = self.values[index]
                .trim()
                .replace(',', ".")
                .parse::<f32>()
                .ok()
                .filter(|value| value.is_finite())
                .ok_or_else(|| "Informe um número finito em cada campo.".to_string())?;
        }
        Ok(Vec3::from_array(values))
    }

    fn preview(&mut self, state: &mut AppState) -> Result<(), String> {
        if !self.active {
            state.pending_modal = None;
            state
                .begin_modal(self.kind)
                .map_err(|error| error.to_string())?;
            self.active = true;
        }
        let values = self.parsed()?;
        if transform(self.kind) {
            state.update_modal_components(values)
        } else {
            state
                .set_modal_constraint(ModalConstraint::Free)
                .map_err(|error| error.to_string())?;
            state.update_modal(Vec3::ZERO, values.x)
        }
        .map_err(|error| error.to_string())
    }
}

fn transform(kind: ModalKind) -> bool {
    matches!(kind, ModalKind::Move | ModalKind::Rotate | ModalKind::Scale)
}

/// Viewport input yields while the property editor owns the active transaction.
pub fn owns_modal(ctx: &egui::Context) -> bool {
    ctx.data_mut(|data| data.get_temp::<FieldSession>(egui::Id::new(SESSION_ID)))
        .is_some_and(|session| session.active)
}

fn selected_kind(state: &AppState) -> Option<ModalKind> {
    state
        .modal
        .as_ref()
        .map(|modal| modal.kind)
        .or(state.pending_modal)
        .or(match state.active_tool.as_str() {
            "transform" => Some(state.gizmo_mode),
            "extrude" => Some(ModalKind::Extrude),
            "inset" => Some(ModalKind::Inset),
            "bevel" => Some(ModalKind::Bevel),
            "pushpull" => Some(ModalKind::PushPull),
            _ => None,
        })
}

fn field_id(kind: ModalKind, index: usize) -> egui::Id {
    egui::Id::new((SESSION_ID, kind.label(), index))
}

/// Returns true when the selected tool has editable numeric properties.
/// This section must be drawn outside the disabled legacy tool controls.
pub fn draw(ui: &mut egui::Ui, state: &mut AppState) -> bool {
    let ctx = ui.ctx().clone();
    let id = egui::Id::new(SESSION_ID);
    let Some(mut kind) = selected_kind(state) else {
        ctx.data_mut(|data| data.remove::<FieldSession>(id));
        return false;
    };
    let mut session = ctx
        .data_mut(|data| data.get_temp::<FieldSession>(id))
        .filter(|session| session.kind == kind)
        .unwrap_or_else(|| FieldSession::new(kind, state));
    if session.active && state.modal.is_none() {
        session = FieldSession::new(kind, state);
    }
    let blocked = state.mesh_preview.is_some()
        || state.paint_stroke.is_some()
        || (state.modal.is_some() && !session.active);
    ui.heading("Propriedades da ferramenta");
    if transform(kind) {
        ui.add_enabled_ui(!session.active && !blocked, |ui| {
            ui.horizontal(|ui| {
                for (label, candidate) in [
                    ("Mover", ModalKind::Move),
                    ("Rotacionar", ModalKind::Rotate),
                    ("Escalar", ModalKind::Scale),
                ] {
                    if ui.selectable_label(kind == candidate, label).clicked() {
                        kind = candidate;
                        state.gizmo_mode = kind;
                        state.pending_modal = None;
                        session = FieldSession::new(kind, state);
                        state.mark_dirty();
                    }
                }
            });
        });
    }
    if blocked {
        if let Some(modal) = &state.modal {
            session.values = if transform(kind) {
                modal.components.to_array()
            } else {
                [modal.value, 0.0, 0.0]
            }
            .map(|value| format!("{value:.4}"));
        }
        ui.label("Confirme ou cancele a operação no viewport para editar os campos.");
    }
    let (label, unit) = match kind {
        ModalKind::Move => ("Deslocamento", "m"),
        ModalKind::Rotate => ("Rotação XYZ", "°"),
        ModalKind::Scale => ("Escala por eixo", "×"),
        ModalKind::Extrude => ("Distância de extrusão", "m"),
        ModalKind::Inset => ("Fator de inset", "0–0,95"),
        ModalKind::Bevel => ("Largura do chanfro", "m"),
        ModalKind::PushPull => ("Distância de empurrar/puxar", "m"),
    };
    ui.label(label);
    let mut changed = false;
    let mut focused = false;
    let mut apply = false;
    let mut cancel = false;
    ui.add_enabled_ui(!blocked, |ui| {
        egui::Grid::new((SESSION_ID, "grid"))
            .num_columns(3)
            .show(ui, |ui| {
                for index in 0..if transform(kind) { 3 } else { 1 } {
                    ui.label(if transform(kind) {
                        ["X", "Y", "Z"][index]
                    } else {
                        "Valor"
                    });
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut session.values[index])
                            .id(field_id(kind, index))
                            .desired_width(100.0)
                            .char_limit(64),
                    );
                    changed |= response.changed();
                    focused |= response.has_focus() || response.lost_focus();
                    ui.label(unit);
                    ui.end_row();
                }
            });
        if changed {
            session.error = session.preview(state).err();
            state.mark_dirty();
        }
        ui.horizontal(|ui| {
            apply = ui
                .add_enabled(session.parsed().is_ok(), egui::Button::new("Aplicar"))
                .clicked();
            cancel = ui
                .add_enabled(
                    session.active || state.pending_modal.is_some(),
                    egui::Button::new("Cancelar"),
                )
                .clicked();
        });
        if focused || session.active {
            apply |= ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Enter));
            cancel |= ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Escape));
        }
    });
    if cancel {
        state.cancel_modal();
        session = FieldSession::new(kind, state);
    } else if apply {
        match session.preview(state) {
            Ok(()) => {
                if let Ok(values) = session.parsed() {
                    match kind {
                        ModalKind::Extrude => state.extrude_dist = values.x,
                        ModalKind::Inset => state.inset_factor = values.x,
                        ModalKind::Bevel => state.bevel_amount = values.x,
                        ModalKind::PushPull => state.push_dist = values.x,
                        _ => {}
                    }
                }
                state.commit_modal();
                session = FieldSession::new(kind, state);
            }
            Err(error) => session.error = Some(error),
        }
    }
    if let Some(error) = &session.error {
        ui.colored_label(egui::Color32::LIGHT_RED, error);
    }
    ui.small("A edição mostra uma prévia. Enter aplica; Esc cancela.");
    if kind == ModalKind::Rotate {
        ui.small("Ângulos em graus, ordem Euler XYZ, em torno do centro da seleção.");
    }
    if kind == ModalKind::Bevel {
        ui.small("Uma aresta convexa com cantos simples; um segmento.");
    }
    ctx.data_mut(|data| data.insert_temp(id, session));
    true
}
