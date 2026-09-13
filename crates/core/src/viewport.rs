//! Contrato comum entre retângulo da UI, picking e os viewports das duas GPUs.

/// Retângulo no espaço de coordenadas lógicas de tela da UI.
/// Totalmente desacoplado de bibliotecas gráficas concretas (egui, Qt, Slint, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LogicalRect {
    pub min: [f32; 2],
    pub max: [f32; 2],
}

impl LogicalRect {
    pub const fn from_min_max(min: [f32; 2], max: [f32; 2]) -> Self {
        Self { min, max }
    }

    pub fn left(&self) -> f32 {
        self.min[0]
    }

    pub fn top(&self) -> f32 {
        self.min[1]
    }

    pub fn right(&self) -> f32 {
        self.max[0]
    }

    pub fn bottom(&self) -> f32 {
        self.max[1]
    }

    pub fn width(&self) -> f32 {
        self.max[0] - self.min[0]
    }

    pub fn height(&self) -> f32 {
        self.max[1] - self.min[1]
    }

    pub fn center(&self) -> [f32; 2] {
        [
            (self.min[0] + self.max[0]) * 0.5,
            (self.min[1] + self.max[1]) * 0.5,
        ]
    }

    pub fn is_finite(&self) -> bool {
        self.min[0].is_finite()
            && self.min[1].is_finite()
            && self.max[0].is_finite()
            && self.max[1].is_finite()
    }

    pub fn is_positive(&self) -> bool {
        self.width() > 0.0 && self.height() > 0.0
    }

    pub fn contains(&self, p: [f32; 2]) -> bool {
        p[0] >= self.min[0] && p[0] <= self.max[0] && p[1] >= self.min[1] && p[1] <= self.max[1]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalViewport {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl PhysicalViewport {
    pub fn from_logical(
        rect: Option<LogicalRect>,
        pixels_per_point: f32,
        width: u32,
        height: u32,
    ) -> Option<Self> {
        if width == 0 || height == 0 || !pixels_per_point.is_finite() || pixels_per_point <= 0.0 {
            return None;
        }
        let Some(rect) = rect else {
            return Some(Self {
                x: 0,
                y: 0,
                width,
                height,
            });
        };
        if !rect.is_finite() || !rect.is_positive() {
            return None;
        }
        let x = (rect.left() * pixels_per_point)
            .round()
            .clamp(0.0, width as f32) as u32;
        let y = (rect.top() * pixels_per_point)
            .round()
            .clamp(0.0, height as f32) as u32;
        let right = (rect.right() * pixels_per_point)
            .round()
            .clamp(0.0, width as f32) as u32;
        let bottom = (rect.bottom() * pixels_per_point)
            .round()
            .clamp(0.0, height as f32) as u32;
        (right > x && bottom > y).then_some(Self {
            x,
            y,
            width: right.saturating_sub(x),
            height: bottom.saturating_sub(y),
        })
    }

    /// OpenGL começa no canto inferior esquerdo; egui e wgpu usam o superior.
    pub fn gl_y(self, framebuffer_height: u32) -> u32 {
        framebuffer_height.saturating_sub(self.y + self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panels_and_hidpi_map_to_same_pixels_for_both_backends() {
        let rect = LogicalRect::from_min_max([60.0, 24.0], [440.0, 710.0]);
        let viewport = PhysicalViewport::from_logical(Some(rect), 2.0, 1356, 1478).unwrap();
        assert_eq!(
            viewport,
            PhysicalViewport {
                x: 120,
                y: 48,
                width: 760,
                height: 1372
            }
        );
        assert_eq!(viewport.gl_y(1478), 58);
        let center = [rect.center()[0] * 2.0, rect.center()[1] * 2.0];
        assert_eq!(center[0], viewport.x as f32 + viewport.width as f32 * 0.5);
        assert_eq!(center[1], viewport.y as f32 + viewport.height as f32 * 0.5);
    }

    #[test]
    fn resize_clamps_gpu_rect_and_rejects_empty_viewport() {
        let rect = LogicalRect::from_min_max([60.0, 24.0], [440.0, 710.0]);
        assert_eq!(
            PhysicalViewport::from_logical(Some(rect), 1.0, 300, 400),
            Some(PhysicalViewport {
                x: 60,
                y: 24,
                width: 240,
                height: 376
            })
        );
        assert!(PhysicalViewport::from_logical(Some(rect), 1.0, 40, 400).is_none());
        assert!(PhysicalViewport::from_logical(Some(rect), f32::NAN, 400, 400).is_none());
    }

    #[test]
    fn projected_corners_and_interior_agree_with_egui_at_fractional_dpi() {
        let rect = LogicalRect::from_min_max([73.25, 31.75], [615.5, 469.25]);
        // Viewport deslocado e assimétrico detecta inversão Y escondida quando
        // a cena ocupa a janela toda. Tolera apenas arredondamento subpixel.
        for scale in [1.0, 1.5, 2.0] {
            let viewport = PhysicalViewport::from_logical(Some(rect), scale, 1600, 1200).unwrap();
            for (nx, ny) in [
                (-1.0, -1.0),
                (-1.0, 1.0),
                (1.0, -1.0),
                (1.0, 1.0),
                (0.0, 0.0),
                (0.6, -0.4),
            ] {
                let expected_x = (rect.center()[0] + nx * rect.width() * 0.5) * scale;
                let expected_y = (rect.center()[1] - ny * rect.height() * 0.5) * scale;
                let gpu_x = viewport.x as f32 + (nx + 1.0) * viewport.width as f32 * 0.5;
                let wgpu_y = viewport.y as f32 + (1.0 - ny) * viewport.height as f32 * 0.5;
                let gl_y = viewport.gl_y(1200) as f32 + (ny + 1.0) * viewport.height as f32 * 0.5;
                assert!((gpu_x - expected_x).abs() <= 0.5);
                assert!((wgpu_y - expected_y).abs() <= 0.5);
                assert!((1200.0 - gl_y - expected_y).abs() <= 0.5);
            }
        }
    }
}
