//! Habilita/desliga ferramentas sem quebrar (lido por module-model).

use std::collections::HashMap;
use std::fs;

/// Lê `[tools] id = true/false`. Desconhecido = ligado.
pub fn load_tools_config() -> HashMap<String, bool> {
    let mut map = HashMap::new();
    for path in [
        "assets/tools.toml",
        "tools.toml",
        "../tools.toml",
        "../../tools.toml",
    ] {
        if let Ok(text) = fs::read_to_string(path) {
            if let Ok(v) = toml::from_str::<toml::Value>(&text) {
                if let Some(t) = v.get("tools").and_then(|t| t.as_table()) {
                    for (k, val) in t {
                        if let Some(b) = val.as_bool() {
                            map.insert(k.clone(), b);
                        }
                    }
                }
            }
            break;
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let p = dir.join("assets/tools.toml");
            if let Ok(text) = fs::read_to_string(&p) {
                if let Ok(v) = toml::from_str::<toml::Value>(&text) {
                    if let Some(t) = v.get("tools").and_then(|t| t.as_table()) {
                        for (k, val) in t {
                            if let Some(b) = val.as_bool() {
                                map.insert(k.clone(), b);
                            }
                        }
                    }
                }
            }
        }
    }
    map
}
