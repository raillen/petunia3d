//! Regiões canônicas do shell UI (Wave 2 — §4.1).
//!
//! Fonte única da verdade para retângulos seguros de layout. Cada componente
//! registra seu próprio retângulo ao desenhar; overlays (shelf, gizmo, HUD) e
//! hit-testing leem daqui em vez de derivar da tela global.
//!
//! Ordem canônica de alocação (§4.2): Header → Viewport Toolbar → Status Bar →
//! Bottom Pane → Left Tools → Right Dock → Central Content/Viewport → overlays.
//! Painéis `top`/`bottom` não competem entre si; a ordem entre laterais e a
//! toolbar do viewport define que a toolbar vive só na área central (padrão
//! Blender: header do editor dentro da área do editor).

/// Slot de região do shell. Um escritor por slot (§3.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegionSlot {
    Header,
    ViewportToolbar,
    StatusBar,
    LeftTools,
    AssetBrowser,
    BottomDock,
    RightDock,
    RightOutliner,
    RightInspector,
    UvEditor,
    Viewport,
    Shelf,
}

/// Retângulos do shell no frame corrente. `None` = pane oculto neste frame.
#[derive(Debug, Clone, Default)]
pub struct UiRegions {
    pub header: Option<egui::Rect>,
    pub viewport_toolbar: Option<egui::Rect>,
    pub status_bar: Option<egui::Rect>,
    pub left_tools: Option<egui::Rect>,
    pub asset_browser: Option<egui::Rect>,
    pub bottom_dock: Option<egui::Rect>,
    pub right_dock: Option<egui::Rect>,
    pub right_outliner: Option<egui::Rect>,
    pub right_inspector: Option<egui::Rect>,
    pub uv_editor: Option<egui::Rect>,
    pub viewport: Option<egui::Rect>,
    pub shelf: Option<egui::Rect>,
}

impl UiRegions {
    fn slot_mut(&mut self, slot: RegionSlot) -> &mut Option<egui::Rect> {
        match slot {
            RegionSlot::Header => &mut self.header,
            RegionSlot::ViewportToolbar => &mut self.viewport_toolbar,
            RegionSlot::StatusBar => &mut self.status_bar,
            RegionSlot::LeftTools => &mut self.left_tools,
            RegionSlot::AssetBrowser => &mut self.asset_browser,
            RegionSlot::BottomDock => &mut self.bottom_dock,
            RegionSlot::RightDock => &mut self.right_dock,
            RegionSlot::RightOutliner => &mut self.right_outliner,
            RegionSlot::RightInspector => &mut self.right_inspector,
            RegionSlot::UvEditor => &mut self.uv_editor,
            RegionSlot::Viewport => &mut self.viewport,
            RegionSlot::Shelf => &mut self.shelf,
        }
    }

    /// Painéis principais (exclui overlays contidos como shelf).
    pub fn panes(&self) -> Vec<(&'static str, egui::Rect)> {
        [
            ("header", self.header),
            ("viewport_toolbar", self.viewport_toolbar),
            ("status_bar", self.status_bar),
            ("left_tools", self.left_tools),
            ("asset_browser", self.asset_browser),
            ("bottom_dock", self.bottom_dock),
            ("right_dock", self.right_dock),
            ("uv_editor", self.uv_editor),
            ("viewport", self.viewport),
        ]
        .into_iter()
        .filter_map(|(name, r)| r.map(|rect| (name, rect)))
        .collect()
    }

    /// Nomes dos painéis que invadem a barra de status (deve ser vazio).
    ///
    /// Exige área positiva de interseção: `Rect::intersects` do egui usa `<=` e
    /// acusa até painéis perfeitamente adjacentes (borda compartilhada).
    pub fn status_overlaps(&self) -> Vec<&'static str> {
        let Some(status) = self.status_bar else {
            return Vec::new();
        };
        self.panes()
            .into_iter()
            .filter(|(name, _)| *name != "status_bar")
            .filter(|(_, r)| overlaps_area(*r, status))
            .map(|(name, _)| name)
            .collect()
    }

    /// A shelf deve permanecer dentro da viewport segura.
    pub fn shelf_within_viewport(&self) -> bool {
        match (self.shelf, self.viewport) {
            (Some(shelf), Some(viewport)) => viewport.contains_rect(shelf),
            // Sem shelf visível: invariante trivialmente satisfeito.
            (None, _) => true,
            // Shelf sem viewport conhecida: não verificável.
            (Some(_), None) => false,
        }
    }

    /// Outliner e Inspector devem ter retângulos independentes e disjuntos.
    pub fn dock_sections_disjoint(&self) -> bool {
        match (self.right_outliner, self.right_inspector) {
            (Some(a), Some(b)) => !overlaps_area(a, b),
            _ => true,
        }
    }
}

/// Interseção com área positiva (tolerância de meio pixel p/ poeira float).
fn overlaps_area(a: egui::Rect, b: egui::Rect) -> bool {
    let inter = a.intersect(b);
    inter.width() > 0.5 && inter.height() > 0.5
}

/// Alturas do dock direito (Wave 2 — §4.4).
pub const DOCK_MIN_OUTLINER: f32 = 150.0;
pub const DOCK_MIN_INSPECTOR: f32 = 200.0;
pub const DOCK_SEPARATOR_H: f32 = 10.0;
pub const DOCK_HEADER_H: f32 = 28.0;

/// Alturas (outliner, inspector) do dock para uma altura total dada.
///
/// Pura e total: nunca panica, nunca retorna negativo; em painéis minúsculos
/// divide o espaço em vez de respeitar mínimos impossíveis.
pub fn split_heights(
    total_h: f32,
    split: f32,
    outliner_collapsed: bool,
    inspector_collapsed: bool,
) -> (f32, f32) {
    let sep = DOCK_SEPARATOR_H;
    match (outliner_collapsed, inspector_collapsed) {
        (true, true) => (DOCK_HEADER_H, DOCK_HEADER_H),
        (true, false) => (
            DOCK_HEADER_H,
            (total_h - DOCK_HEADER_H - sep).max(DOCK_HEADER_H),
        ),
        (false, true) => (
            (total_h - DOCK_HEADER_H - sep).max(DOCK_HEADER_H),
            DOCK_HEADER_H,
        ),
        (false, false) => {
            let room = (total_h - sep).max(0.0);
            if room <= DOCK_HEADER_H * 2.0 {
                (room * 0.5, room * 0.5)
            } else {
                let lo = DOCK_MIN_OUTLINER.min(room * 0.5).max(DOCK_HEADER_H);
                let hi = (room - DOCK_MIN_INSPECTOR).max(lo);
                let out = (room * split.clamp(0.25, 0.75)).clamp(lo, hi);
                (out, room - out)
            }
        }
    }
}

const REGIONS_KEY: &str = "petunia_ui_regions";

/// Reseta as regiões no início do frame (evita slots obsoletos de painéis ocultos).
pub fn reset(ctx: &egui::Context) {
    ctx.data_mut(|d| d.insert_temp(egui::Id::new(REGIONS_KEY), UiRegions::default()));
}

/// Registra o retângulo de um slot (chamado pelo dono do componente).
pub fn record(ctx: &egui::Context, slot: RegionSlot, rect: egui::Rect) {
    ctx.data_mut(|d| {
        let mut regions = d
            .get_temp::<UiRegions>(egui::Id::new(REGIONS_KEY))
            .unwrap_or_default();
        *regions.slot_mut(slot) = Some(rect);
        d.insert_temp(egui::Id::new(REGIONS_KEY), regions);
    });
}

/// Lê as regiões do frame corrente (para overlays e testes).
pub fn load(ctx: &egui::Context) -> Option<UiRegions> {
    ctx.data(|d| d.get_temp::<UiRegions>(egui::Id::new(REGIONS_KEY)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rect(x0: f32, y0: f32, x1: f32, y1: f32) -> egui::Rect {
        egui::Rect::from_min_max(egui::pos2(x0, y0), egui::pos2(x1, y1))
    }

    #[test]
    fn no_overlap_by_default() {
        let regions = UiRegions::default();
        assert!(regions.status_overlaps().is_empty());
        assert!(regions.shelf_within_viewport());
        assert!(regions.dock_sections_disjoint());
    }

    #[test]
    fn detects_status_overlap() {
        let regions = UiRegions {
            status_bar: Some(rect(0.0, 1000.0, 1920.0, 1080.0)),
            viewport: Some(rect(0.0, 100.0, 1500.0, 1050.0)),
            ..Default::default()
        };
        assert_eq!(regions.status_overlaps(), vec!["viewport"]);
    }

    #[test]
    fn edge_touching_panels_do_not_overlap() {
        let regions = UiRegions {
            status_bar: Some(rect(0.0, 1000.0, 1920.0, 1080.0)),
            viewport: Some(rect(0.0, 100.0, 1500.0, 1000.0)),
            ..Default::default()
        };
        assert!(regions.status_overlaps().is_empty());
    }

    #[test]
    fn shelf_outside_viewport_fails() {
        let regions = UiRegions {
            viewport: Some(rect(0.0, 0.0, 800.0, 600.0)),
            shelf: Some(rect(0.0, 700.0, 800.0, 740.0)),
            ..Default::default()
        };
        assert!(!regions.shelf_within_viewport());
    }

    #[test]
    fn disjoint_sections_pass() {
        let regions = UiRegions {
            right_outliner: Some(rect(1500.0, 0.0, 1920.0, 400.0)),
            right_inspector: Some(rect(1500.0, 408.0, 1920.0, 1000.0)),
            ..Default::default()
        };
        assert!(regions.dock_sections_disjoint());
    }

    #[test]
    fn intersecting_sections_fail() {
        let regions = UiRegions {
            right_outliner: Some(rect(1500.0, 0.0, 1920.0, 500.0)),
            right_inspector: Some(rect(1500.0, 400.0, 1920.0, 1000.0)),
            ..Default::default()
        };
        assert!(!regions.dock_sections_disjoint());
    }

    #[test]
    fn split_open_uses_fraction() {
        let (out, insp) = split_heights(600.0, 0.42, false, false);
        assert!((out - 247.8).abs() < 0.5);
        assert!((out + insp - (600.0 - DOCK_SEPARATOR_H)).abs() < 0.01);
    }

    #[test]
    fn split_collapsed_gives_header() {
        let (out, insp) = split_heights(600.0, 0.42, true, false);
        assert_eq!(out, DOCK_HEADER_H);
        assert!(insp > DOCK_MIN_INSPECTOR);
    }

    #[test]
    fn split_tiny_panel_never_negative_or_nan() {
        for total in [0.0, 40.0, 100.0, 200.0] {
            let (out, insp) = split_heights(total, 0.42, false, false);
            assert!(out >= 0.0 && insp >= 0.0);
            assert!((out + insp - (total - DOCK_SEPARATOR_H).max(0.0)).abs() < 0.01);
        }
    }

    #[test]
    fn split_clamps_fraction() {
        let (out_narrow, _) = split_heights(600.0, 0.0, false, false);
        let (out_wide, _) = split_heights(600.0, 1.0, false, false);
        assert!(out_narrow >= DOCK_HEADER_H && out_wide <= 600.0 - DOCK_SEPARATOR_H);
        assert!(out_narrow < out_wide);
    }
}
