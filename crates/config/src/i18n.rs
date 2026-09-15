//! i18n via TOML — melhor encaixe para Rust:
//! - `serde + toml` é idiomático, tipado e com comentários (#),
//! - menos verboso que JSON, sem as armadilhas do YAML (spec gigante),
//! - um arquivo por idioma em `locales/*.toml`, fácil de traduzir.
//!
//! Formato: tabelas viram prefixo com ponto. Ex.:
//! ```toml
//! [ui]
//! title = "Simple3D"
//! [tools]
//! select = "Selecionar"
//! ```
//! vira chaves `ui.title`, `tools.select` via `t()`.

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct I18n {
    pub lang: String,
    map: HashMap<String, String>,
    fallback: HashMap<String, String>,
}

impl I18n {
    pub fn available() -> Vec<String> {
        let mut out = vec!["en".to_string()];
        if let Ok(entries) = fs::read_dir(locales_dir()) {
            for e in entries.flatten() {
                let p = e.path();
                if p.extension().map(|x| x == "toml").unwrap_or(false)
                    && let Some(stem) = p.file_stem().and_then(|s| s.to_str())
                    && stem != "en"
                    && !out.contains(&stem.to_string())
                {
                    out.push(stem.to_string());
                }
            }
        }
        out.sort();
        out
    }

    pub fn load(lang: &str) -> Self {
        let fallback = load_file("en");
        let map = if lang == "en" {
            fallback.clone()
        } else {
            let m = load_file(lang);
            if m.is_empty() { fallback.clone() } else { m }
        };
        Self {
            lang: lang.to_string(),
            map,
            fallback,
        }
    }

    pub fn set_lang(&mut self, lang: &str) {
        *self = Self::load(lang);
    }

    /// Traduz `chave`. Cai para inglês e depois para a própria chave.
    /// Retorna `String` (não `&str`) de propósito: evita borrow persistente
    /// de `app` dentro dos closures do egui.
    pub fn t(&self, key: &str) -> String {
        self.map
            .get(key)
            .or_else(|| self.fallback.get(key))
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }
}

fn locales_dir() -> PathBuf {
    // 1. ./assets/locales (dev, executando da raiz)
    // 2. ao lado do executável (instalado)
    for cand in ["assets/locales", "locales"] {
        let p = PathBuf::from(cand);
        if p.exists() {
            return p;
        }
    }
    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        for cand in ["assets/locales", "locales", "../../assets/locales"] {
            let p = dir.join(cand);
            if p.exists() {
                return p;
            }
        }
    }
    PathBuf::from("assets/locales")
}

fn load_file(lang: &str) -> HashMap<String, String> {
    let path = locales_dir().join(format!("{lang}.toml"));
    let text = fs::read_to_string(&path).unwrap_or_default();
    if text.is_empty() {
        return HashMap::new();
    }
    let v: toml::Value = toml::from_str(&text).unwrap_or(toml::Value::Table(Default::default()));
    let mut map = HashMap::new();
    flatten(&v, String::new(), &mut map);
    map
}

fn flatten(v: &toml::Value, prefix: String, out: &mut HashMap<String, String>) {
    match v {
        toml::Value::Table(t) => {
            for (k, vv) in t {
                let key = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{prefix}.{k}")
                };
                flatten(vv, key, out);
            }
        }
        toml::Value::String(s) => {
            out.insert(prefix, s.clone());
        }
        other => {
            out.insert(prefix, other.to_string());
        }
    }
}
