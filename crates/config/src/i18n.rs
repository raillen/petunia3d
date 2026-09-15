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

    /// Traduz um [`TextId`] tipado (Wave 7).
    pub fn t_id(&self, id: TextId) -> String {
        self.t(id.key())
    }

    /// Constrói de um texto TOML (testes e pseudo-locale, sem disco).
    pub fn parse(text: &str) -> HashMap<String, String> {
        let v: toml::Value = toml::from_str(text).unwrap_or(toml::Value::Table(Default::default()));
        let mut map = HashMap::new();
        flatten(&v, String::new(), &mut map);
        map
    }

    /// Pseudo-locale de teste (Wave 7 — §12.4): expande ~40% e acentua para
    /// expor clipping, larguras fixas e overflow de shelf/dropdowns.
    /// Uso exclusivo em testes; nunca embarca.
    pub fn pseudo_from(base: &Self) -> Self {
        let map = base
            .map
            .iter()
            .map(|(k, v)| (k.clone(), pseudo_transform(v)))
            .collect();
        let fallback = base
            .fallback
            .iter()
            .map(|(k, v)| (k.clone(), pseudo_transform(v)))
            .collect();
        Self {
            lang: "pseudo".to_string(),
            map,
            fallback,
        }
    }

    /// Chaves do mapa principal (auditoria de paridade).
    pub fn keys(&self) -> Vec<String> {
        let mut keys: Vec<String> = self.map.keys().cloned().collect();
        keys.sort();
        keys
    }
}

/// Expansão pseudo-locale: prefixo/sufixo visíveis + alongamento + acentos.
fn pseudo_transform(text: &str) -> String {
    const ACCENTS: &[char] = &['é', 'ñ', 'ü', 'ß', 'ç', 'ø'];
    let mut out = String::with_capacity(text.len() * 2 + 4);
    out.push('⟦');
    for (i, ch) in text.chars().enumerate() {
        out.push(ch);
        if ch.is_ascii_alphabetic() && i % 2 == 0 {
            out.push(ch);
        }
        if ch == ' ' {
            out.push(ACCENTS[i % ACCENTS.len()]);
        }
    }
    out.push('⟧');
    out
}

/// Identificador de tradução tipado (Wave 7 — §12.2).
///
/// Evita chaves stringly-typed espalhadas: o catálogo vive em [`text_id`] e o
/// teste de paridade garante resolução em todos os locales embutidos.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextId(pub &'static str);

impl TextId {
    pub const fn new(key: &'static str) -> Self {
        Self(key)
    }

    pub fn key(self) -> &'static str {
        self.0
    }
}

/// Catálogo de `TextId` das superfícies migradas (Waves 2–7).
///
/// Chaves pré-existentes em formato string continuam válidas (compatível);
/// superfícies novas usam estas constantes.
pub mod text_id {
    use super::TextId;

    pub const UI_OUTLINER: TextId = TextId::new("ui.outliner");
    pub const UI_PROPERTIES: TextId = TextId::new("ui.properties");
    pub const UI_COLLAPSE: TextId = TextId::new("ui.collapse");
    pub const UI_EXPAND: TextId = TextId::new("ui.expand");
    pub const UI_DOCK_SPLIT_HINT: TextId = TextId::new("ui.dock_split_hint");
    pub const UI_MORE: TextId = TextId::new("ui.more");
    pub const UI_TOOLS_MENU: TextId = TextId::new("ui.tools_menu");
    pub const UI_DUPLICATE: TextId = TextId::new("ui.duplicate");
    pub const UI_REFS: TextId = TextId::new("ui.refs");
    pub const UI_ASSETS: TextId = TextId::new("ui.assets");
    pub const UI_CLOSE: TextId = TextId::new("ui.close");
    pub const UI_FLOATING_INSPECTOR: TextId = TextId::new("ui.floating_inspector");
    pub const UI_REDOCK: TextId = TextId::new("ui.redock");
    pub const UI_AT_3D_CURSOR: TextId = TextId::new("ui.at_3d_cursor");

    pub const UV_TITLE: TextId = TextId::new("uv.title");
    pub const UV_SELECTED: TextId = TextId::new("uv.selected");
    pub const UV_FACES: TextId = TextId::new("uv.faces");
    pub const UV_PREVIEW_3D: TextId = TextId::new("uv.preview_3d");
    pub const UV_HINT: TextId = TextId::new("uv.hint");

    pub const ANIMATE_HUMANOID: TextId = TextId::new("animate.humanoid");
    pub const ANIMATE_AUTO_RIG: TextId = TextId::new("animate.auto_rig");
    pub const ANIMATE_PLAY: TextId = TextId::new("animate.play");
    pub const ANIMATE_PAUSE: TextId = TextId::new("animate.pause");
    pub const ANIMATE_FIRST_FRAME: TextId = TextId::new("animate.first_frame");
    pub const ANIMATE_LAST_FRAME: TextId = TextId::new("animate.last_frame");
    pub const ANIMATE_FRAME: TextId = TextId::new("animate.frame");
    pub const ANIMATE_TIP_FIRST: TextId = TextId::new("animate.tip_first");
    pub const ANIMATE_TIP_PREV: TextId = TextId::new("animate.tip_prev");
    pub const ANIMATE_TIP_PLAY: TextId = TextId::new("animate.tip_play");
    pub const ANIMATE_TIP_NEXT: TextId = TextId::new("animate.tip_next");
    pub const ANIMATE_TIP_LAST: TextId = TextId::new("animate.tip_last");

    pub const PAINT_RADIUS: TextId = TextId::new("paint.radius");
    pub const PAINT_COLOR: TextId = TextId::new("paint.color");

    pub const SETTINGS_INTERFACE: TextId = TextId::new("settings.interface");
    pub const SETTINGS_IMPORT_EXPORT: TextId = TextId::new("settings.import_export");
    pub const SETTINGS_SHOW_SHELF: TextId = TextId::new("settings.show_shelf");
    pub const SETTINGS_RESET_WORKSPACE: TextId = TextId::new("settings.reset_workspace");
    pub const SETTINGS_RESET_ALL: TextId = TextId::new("settings.reset_all_layouts");
    pub const SETTINGS_EXPORT_GLB: TextId = TextId::new("settings.export_glb");
    pub const SETTINGS_EXPORT_GLB_HINT: TextId = TextId::new("settings.export_glb_hint");

    pub const REFS_VISIBLE: TextId = TextId::new("refs.visible");
    pub const REFS_LOCK: TextId = TextId::new("refs.lock");
    pub const REFS_CLICK_TO_LOAD: TextId = TextId::new("refs.click_to_load");
    pub const REFS_NO_IMAGE: TextId = TextId::new("refs.no_image");
    pub const REFS_REPLACE: TextId = TextId::new("refs.replace");
    pub const REFS_REMOVE: TextId = TextId::new("refs.remove");
    pub const REFS_ALIGN_VIEW: TextId = TextId::new("refs.align_view");
    pub const REFS_RESET_DEFAULT: TextId = TextId::new("refs.reset_default");
    pub const REFS_FINE_TUNE: TextId = TextId::new("refs.fine_tune");
    pub const REFS_LOADED: TextId = TextId::new("refs.loaded");
    pub const REFS_OPACITY: TextId = TextId::new("refs.opacity");
    pub const REFS_SIZE: TextId = TextId::new("refs.size");
    pub const REFS_OFFSET: TextId = TextId::new("refs.offset");
    pub const REFS_ROTATION: TextId = TextId::new("refs.rotation");

    /// Todos os ids do catálogo (cobertura de tradução).
    pub const ALL: &[TextId] = &[
        UI_OUTLINER,
        UI_PROPERTIES,
        UI_COLLAPSE,
        UI_EXPAND,
        UI_DOCK_SPLIT_HINT,
        UI_MORE,
        UI_TOOLS_MENU,
        UI_DUPLICATE,
        UI_REFS,
        UI_ASSETS,
        UI_CLOSE,
        UI_FLOATING_INSPECTOR,
        UI_REDOCK,
        UI_AT_3D_CURSOR,
        UV_TITLE,
        UV_SELECTED,
        UV_FACES,
        UV_PREVIEW_3D,
        UV_HINT,
        ANIMATE_HUMANOID,
        ANIMATE_AUTO_RIG,
        ANIMATE_PLAY,
        ANIMATE_PAUSE,
        ANIMATE_FIRST_FRAME,
        ANIMATE_LAST_FRAME,
        ANIMATE_FRAME,
        ANIMATE_TIP_FIRST,
        ANIMATE_TIP_PREV,
        ANIMATE_TIP_PLAY,
        ANIMATE_TIP_NEXT,
        ANIMATE_TIP_LAST,
        PAINT_RADIUS,
        PAINT_COLOR,
        SETTINGS_INTERFACE,
        SETTINGS_IMPORT_EXPORT,
        SETTINGS_SHOW_SHELF,
        SETTINGS_RESET_WORKSPACE,
        SETTINGS_RESET_ALL,
        SETTINGS_EXPORT_GLB,
        SETTINGS_EXPORT_GLB_HINT,
        REFS_VISIBLE,
        REFS_LOCK,
        REFS_CLICK_TO_LOAD,
        REFS_NO_IMAGE,
        REFS_REPLACE,
        REFS_REMOVE,
        REFS_ALIGN_VIEW,
        REFS_RESET_DEFAULT,
        REFS_FINE_TUNE,
        REFS_LOADED,
        REFS_OPACITY,
        REFS_SIZE,
        REFS_OFFSET,
        REFS_ROTATION,
    ];
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

/// Locales embutidos como reserva (Wave 7): binários instalados sem o diretório
/// de locales e testes executados fora da raiz nunca voltam a exibir chaves.
const EN_EMBEDDED: &str = include_str!("../../../assets/locales/en.toml");
const PT_BR_EMBEDDED: &str = include_str!("../../../assets/locales/pt-BR.toml");

fn embedded(lang: &str) -> &'static str {
    match lang {
        "pt-BR" => PT_BR_EMBEDDED,
        _ => EN_EMBEDDED,
    }
}

fn load_file(lang: &str) -> HashMap<String, String> {
    let path = locales_dir().join(format!("{lang}.toml"));
    let text = fs::read_to_string(&path).unwrap_or_default();
    if text.is_empty() {
        return I18n::parse(embedded(lang));
    }
    I18n::parse(&text)
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

#[cfg(test)]
mod tests {
    use super::text_id::ALL;
    use super::{I18n, pseudo_transform};

    const EN_TOML: &str = include_str!("../../../assets/locales/en.toml");
    const PT_TOML: &str = include_str!("../../../assets/locales/pt-BR.toml");

    fn keys_of(toml_text: &str) -> Vec<String> {
        let mut keys: Vec<String> = I18n::parse(toml_text).keys().cloned().collect();
        keys.sort();
        keys
    }

    #[test]
    fn locale_parity_en_ptbr() {
        // Paridade exata: CI falha com chave faltante ou órfã (Wave 7 — §12.3).
        let en = keys_of(EN_TOML);
        let pt = keys_of(PT_TOML);
        let missing: Vec<&String> = en.iter().filter(|k| !pt.contains(k)).collect();
        let orphan: Vec<&String> = pt.iter().filter(|k| !en.contains(k)).collect();
        assert!(missing.is_empty(), "pt-BR sem tradução: {missing:?}");
        assert!(orphan.is_empty(), "pt-BR com chaves órfãs: {orphan:?}");
        assert!(!en.is_empty());
    }

    #[test]
    fn text_id_catalog_resolves_in_both_locales() {
        let en = I18n::parse(EN_TOML);
        let pt = I18n::parse(PT_TOML);
        assert_eq!(en.len(), pt.len());
        for id in ALL {
            assert!(en.contains_key(id.key()), "en sem {}", id.key());
            assert!(pt.contains_key(id.key()), "pt-BR sem {}", id.key());
            assert!(!en[id.key()].is_empty() && !pt[id.key()].is_empty());
        }
    }

    #[test]
    fn pseudo_expands_and_marks() {
        let out = pseudo_transform("Settings");
        assert!(out.starts_with('⟦') && out.ends_with('⟧'));
        assert!(out.len() > "Settings".len());
        // Texto vazio não quebra.
        assert_eq!(pseudo_transform(""), "⟦⟧");
    }

    #[test]
    fn pseudo_locale_covers_catalog() {
        let base = I18n {
            lang: "en".to_string(),
            map: I18n::parse(EN_TOML),
            fallback: I18n::parse(EN_TOML),
        };
        let pseudo = I18n::pseudo_from(&base);
        assert_eq!(pseudo.lang, "pseudo");
        for id in ALL {
            let s = pseudo.t_id(*id);
            assert!(s.starts_with('⟦'), "{} sem marca pseudo", id.key());
        }
    }
}
