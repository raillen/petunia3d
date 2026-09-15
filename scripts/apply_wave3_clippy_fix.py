#!/usr/bin/env python3
"""Small follow-up patch for Clippy findings in the Wave 3 staging output."""
from pathlib import Path

path = Path(__file__).resolve().parents[1] / "crates/ui/src/primitive_card.rs"
text = path.read_text()

replacements = {
    "    let Some(session) = state.session.primitive_session.clone() else {\n        return None;\n    };":
        "    let session = state.session.primitive_session.clone()?;",
    "(viewport_rect.width() - 24.0).max(96.0).min(300.0)":
        "(viewport_rect.width() - 24.0).clamp(96.0, 300.0)",
    "(viewport_rect.height() - 24.0).max(96.0).min(520.0)":
        "(viewport_rect.height() - 24.0).clamp(96.0, 520.0)",
    "ui.available_width().min(110.0).max(40.0)":
        "ui.available_width().clamp(40.0, 110.0)",
    "ui.available_width().min(160.0).max(64.0)":
        "ui.available_width().clamp(64.0, 160.0)",
}

for old, new in replacements.items():
    if old not in text:
        raise RuntimeError(f"expected Wave 3 generated anchor missing: {old}")
    text = text.replace(old, new)

path.write_text(text)
print("Wave 3 Clippy follow-up applied")
