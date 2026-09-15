#!/usr/bin/env python3
"""Execute the Wave 2 staging script with normalized multiline anchors.

The GitHub contents transport preserved a few anchor newlines as double-escaped
sequences. Normalize only the staging source before compiling it; generated Rust
and TOML files are still authored by the canonical Wave 2 script.
"""
from pathlib import Path

source_path = Path(__file__).with_name("apply_wave2_tool_properties.py")
source = source_path.read_text()
source = source.replace("\\\\n", "\\n")
namespace = {"__file__": str(source_path), "__name__": "__main__"}
exec(compile(source, str(source_path), "exec"), namespace)
