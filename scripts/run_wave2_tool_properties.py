#!/usr/bin/env python3
"""Execute Wave 2 staging and apply CI-discovered borrow-boundary fixes."""
from pathlib import Path

source_path = Path(__file__).with_name("apply_wave2_tool_properties.py")
source = source_path.read_text().replace("\\\\n", "\\n")
namespace = {"__file__": str(source_path), "__name__": "__main__"}
exec(compile(source, str(source_path), "exec"), namespace)

# Rust evaluates method receiver arguments in a way that keeps the mutable
# borrow from Slider::new(&mut state.field, ...) alive while `.text(state.t())`
# tries to borrow state immutably. Resolve labels before borrowing the field.
root = Path(__file__).resolve().parents[1]
path = root / "crates/ui/src/modeling_tool_properties.rs"
text = path.read_text()

text = text.replace(
    '''    ui.separator();
    ui.label(state.t("actions.merge_by_distance"));
    if ui
        .add(
            egui::Slider::new(&mut state.merge_dist, 0.0005..=0.1)
                .text(state.t("actions.merge_distance"))
                .logarithmic(true),
        )''',
    '''    ui.separator();
    ui.label(state.t("actions.merge_by_distance"));
    let merge_distance_label = state.t("actions.merge_distance");
    if ui
        .add(
            egui::Slider::new(&mut state.merge_dist, 0.0005..=0.1)
                .text(merge_distance_label)
                .logarithmic(true),
        )''',
)
text = text.replace(
    '''    if ui
        .add(
            egui::Slider::new(&mut state.revolve_angle, 5.0..=360.0)
                .text(state.t("actions.angle")),
        )''',
    '''    let angle_label = state.t("actions.angle");
    if ui
        .add(egui::Slider::new(&mut state.revolve_angle, 5.0..=360.0).text(angle_label))''',
)
text = text.replace(
    '''    if ui
        .add(
            egui::Slider::new(&mut state.profile.depth, 0.05..=8.0)
                .text(state.t("profile.depth")),
        )''',
    '''    let profile_depth_label = state.t("profile.depth");
    if ui
        .add(egui::Slider::new(&mut state.profile.depth, 0.05..=8.0).text(profile_depth_label))''',
)
text = text.replace(
    '''    if ui
        .add(
            egui::Slider::new(&mut state.profile.revolve_segments, 3..=48)
                .text(state.t("profile.segments")),
        )''',
    '''    let profile_segments_label = state.t("profile.segments");
    if ui
        .add(
            egui::Slider::new(&mut state.profile.revolve_segments, 3..=48)
                .text(profile_segments_label),
        )''',
)
path.write_text(text)
