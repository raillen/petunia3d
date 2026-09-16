//! Component Gallery executável (diretiva §38).
//!
//! ```bash
//! cargo run -p petunia_ui --example component_gallery
//! ```
//!
//! Mostra, num único frame: botões, icon buttons, pílulas de workspace,
//! segmentados, campos, `Vector3Field`, sliders, property rows, seções, abas,
//! clusters de toolbar, menus, layouts do adapter Taffy, temas
//! (dark / high contrast), presets de densidade e de escala.
//!
//! O que ainda **não** existe como componente reutilizável aparece na própria
//! galeria como linha `pendente · <nome>` com o contrato que falta — ver
//! `crates/ui/src/gallery.rs`.
//!
//! Este é um host de desenvolvimento. O app real continua com `WgpuApp`/`GlApp`
//! (`crates/app/src/lib.rs`); aqui só interessa a superfície de UI.

use petunia_ui::gallery::{self, GalleryState};

struct Gallery {
    state: petunia_core::AppState,
    ui: GalleryState,
}

impl Gallery {
    fn new() -> Self {
        Self {
            state: petunia_core::AppState::new("en"),
            ui: GalleryState::default(),
        }
    }
}

impl eframe::App for Gallery {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        gallery::draw(ui, &mut self.state, &mut self.ui);
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1180.0, 820.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Petunia3D · Component Gallery",
        options,
        Box::new(|cc| {
            // Mesma fundação visual do app: tema + fontes de ícone.
            petunia_ui::apply_theme_to_egui(&petunia_config::Theme::load(), &cc.egui_ctx);
            petunia_ui::icon_registry::IconRegistry::ensure_fonts(&cc.egui_ctx);
            Ok(Box::new(Gallery::new()))
        }),
    )
}
