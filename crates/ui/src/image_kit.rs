//! Image loading kit behind Petunia adapters (`egui_extras`, P0-01).
//!
//! Workspace code never touches `egui_extras` directly: image loaders
//! (including the SVG loader required by the vector-first icon policy) are
//! installed once through [`install_image_loaders`], and layout helpers live
//! behind the functions below. `egui_extras` types never reach Core or
//! Application APIs.

/// Installs the Petunia image loader set on `ctx`.
///
/// Includes the `.svg` loader (`egui_extras` `svg` feature): Petunia-owned
/// domain icons ship as SVG and resolve through here. Safe to call more than
/// once; already-installed loaders are skipped by `egui_extras`.
pub fn install_image_loaders(ctx: &egui::Context) {
    egui_extras::install_image_loaders(ctx);
}

/// Whether an SVG image loader is installed on `ctx`.
pub fn has_svg_loader(ctx: &egui::Context) -> bool {
    ctx.is_loader_installed(egui_extras::loaders::svg_loader::SvgLoader::ID)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn installers_are_idempotent_and_provide_svg() {
        let ctx = egui::Context::default();
        install_image_loaders(&ctx);
        install_image_loaders(&ctx);
        assert!(has_svg_loader(&ctx));
    }

    #[test]
    fn image_loader_present_after_install() {
        let ctx = egui::Context::default();
        install_image_loaders(&ctx);
        assert!(ctx.is_loader_installed(egui_extras::loaders::image_loader::ImageCrateLoader::ID));
    }
}
