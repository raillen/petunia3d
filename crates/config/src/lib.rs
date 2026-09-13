//! Petunia3D — configuração externa (TOML): idioma/i18n, keybinds,
//! ferramentas habilitadas e tema. Nada disso é hardcoded nas features.

pub mod i18n;
pub mod keybinds;
pub mod theme;
pub mod tools;

pub use i18n::I18n;
pub use keybinds::{Binding, ConflictKind, KeyConflict, Keybinds, KeymapProfileInfo};
pub use theme::{ColorRgba, Theme, ThemeManifest, ThemeRegistry, ThemeToken};
pub use tools::load_tools_config;
