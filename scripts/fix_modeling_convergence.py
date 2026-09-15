#!/usr/bin/env python3
"""Small retry fix for the modeling convergence staging batch."""

from pathlib import Path

path = Path("crates/ui/src/properties_panel.rs")
text = path.read_text()
legacy = '''    let tool_label = state.t(&format!("tools.{}", state.active_tool));
    if tool_label.to_lowercase().contains(query)
        || state
            .t("inspector.tool_active")
            .to_lowercase()
            .contains(query)
    {
        any = true;
        draw_modify_tool_panel(ui, state, true);
    }
'''
if legacy in text:
    text = text.replace(legacy, "", 1)
path.write_text(text)
