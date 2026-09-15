#!/usr/bin/env python3
"""Small retry fixes for the modeling convergence staging batch."""

from pathlib import Path

# Tool parameters no longer belong to the Inspector search results after the
# viewport-local Tool Properties convergence.
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

# Adding the serde-defaulted modifier stack appends an empty sequence to each
# serialized Asset. The fixture intentionally snapshots the exact container
# byte length, so keep that assertion explicit instead of relying on Insta's
# inline-snapshot rewrite mode in CI.
path = Path("crates/project/tests/save_tree.rs")
text = path.read_text()
text = text.replace('insta::assert_debug_snapshot!(bytes.len(), @"1179");', 'insta::assert_debug_snapshot!(bytes.len(), @"1181");')
path.write_text(text)
