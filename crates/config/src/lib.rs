//! Petunia3D — configuração externa (TOML): idioma/i18n, keybinds,
//! ferramentas habilitadas e tema. Nada disso é hardcoded nas features.

pub mod i18n;
pub mod keybinds;
pub mod theme;
pub mod tools;

pub use i18n::I18n;
pub use keybinds::Keybinds;
pub use theme::Theme;
pub use tools::load_tools_config;
