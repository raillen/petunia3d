//! Matemática de cena compartilhada pelos backends GL e wgpu:
//! grid, quads de referência e constantes de luz. Uma fonte só.

/// Direção da luz + ambiente (lambert simples).
pub const LIGHT_DIR: [f32; 3] = [0.5, 0.9, 0.6];
pub const LIGHT_AMBIENT: f32 = 0.45;
pub const LIGHT_DIFFUSE: f32 = 0.65;

/// Cor de seleção (laranja Blender-like).
pub const SELECT_COLOR: [f32; 3] = [1.0, 0.55, 0.15];
/// Cor de aresta selecionada (overlay).
pub const SELECT_EDGE_COLOR: [f32; 3] = [1.0, 0.35, 0.1];

/// Segmento de linha com cor: (a, b, cor).
pub type ColoredLine = ([f32; 3], [f32; 3], [f32; 3]);

/// Linhas dos eixos mundiais cartesianos (X=vermelho, Y=verde, Z=azul).
pub fn world_axes_lines(extent: f32) -> Vec<ColoredLine> {
    let axis_x = [0.88, 0.24, 0.26];
    let axis_y = [0.38, 0.79, 0.20];
    let axis_z = [0.19, 0.51, 0.96];
    vec![
        ([-extent, 0.0, 0.0], [extent, 0.0, 0.0], axis_x),
        ([0.0, -extent, 0.0], [0.0, extent, 0.0], axis_y),
        ([0.0, 0.0, -extent], [0.0, 0.0, extent], axis_z),
    ]
}

/// Grid estilo Blender no plano XZ com suporte a tamanho, subdivisões, opacidade e guia isométrico.
pub fn grid_lines_custom(
    size: f32,
    spacing: f32,
    opacity: f32,
    show_iso: bool,
    iso_angle_deg: f32,
) -> Vec<ColoredLine> {
    let mut v = Vec::new();
    let extent = size.max(1.0);
    let step = spacing.clamp(0.1, extent);
    let op = opacity.clamp(0.05, 1.0);
    let minor = [0.22 * op * 2.5, 0.22 * op * 2.5, 0.24 * op * 2.5];

    let steps = (extent / step).ceil() as i32;
    for i in -steps..=steps {
        let f = i as f32 * step;
        if f.abs() > extent + 1e-4 {
            continue;
        }
        v.push(([f, 0.0, -extent], [f, 0.0, extent], minor));
        v.push(([-extent, 0.0, f], [extent, 0.0, f], minor));
    }

    if show_iso {
        let iso_color = [0.18 * op * 2.5, 0.42 * op * 2.5, 0.52 * op * 2.5];
        let angle_rad = iso_angle_deg.to_radians();
        let tan_a = angle_rad.tan().abs().max(0.1);
        let iso_step = step * 2.0;
        let iso_steps = (extent * 2.0 / iso_step).ceil() as i32;
        for i in -iso_steps..=iso_steps {
            let offset = i as f32 * iso_step;
            let z0 = -extent * tan_a + offset;
            let z1 = extent * tan_a + offset;
            if (z0 >= -extent && z0 <= extent) || (z1 >= -extent && z1 <= extent) {
                let cz0 = z0.clamp(-extent, extent);
                let cx0 = (cz0 - offset) / tan_a;
                let cz1 = z1.clamp(-extent, extent);
                let cx1 = (cz1 - offset) / tan_a;
                v.push(([cx0, 0.0, cz0], [cx1, 0.0, cz1], iso_color));
            }

            let z0_neg = extent * tan_a + offset;
            let z1_neg = -extent * tan_a + offset;
            if (z0_neg >= -extent && z0_neg <= extent) || (z1_neg >= -extent && z1_neg <= extent) {
                let cz0 = z0_neg.clamp(-extent, extent);
                let cx0 = -(cz0 - offset) / tan_a;
                let cz1 = z1_neg.clamp(-extent, extent);
                let cx1 = -(cz1 - offset) / tan_a;
                v.push(([cx0, 0.0, cz0], [cx1, 0.0, cz1], iso_color));
            }
        }
    }

    v
}

/// Grid estilo Blender no plano XZ (semi-eixo 20).
pub fn grid_lines() -> Vec<ColoredLine> {
    grid_lines_custom(20.0, 1.0, 0.4, false, 30.0)
}

/// Eixo do plano de referência.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefPlane {
    Front,
    Back,
    Left,
    Right,
    Side,
    Top,
    Bottom,
}

/// Quad da imagem de referência (4 cantos) no mundo com rotação opcional em graus.
/// `aspect = width / height`.
pub fn ref_quad(plane: RefPlane, offset: f32, size: f32, aspect: f32) -> [[f32; 3]; 4] {
    ref_quad_with_rot(plane, offset, size, aspect, 0.0)
}

/// Quad da imagem de referência com rotação no plano em graus.
pub fn ref_quad_with_rot(
    plane: RefPlane,
    offset: f32,
    size: f32,
    aspect: f32,
    rotation_deg: f32,
) -> [[f32; 3]; 4] {
    let h = size * 0.5;
    let w = h * aspect;
    let rad = rotation_deg.to_radians();
    let cos_r = rad.cos();
    let sin_r = rad.sin();
    let rot2 = |x: f32, y: f32| (x * cos_r - y * sin_r, x * sin_r + y * cos_r);

    let (p0, p1, p2, p3) = (rot2(-w, -h), rot2(w, -h), rot2(w, h), rot2(-w, h));

    match plane {
        RefPlane::Front => [
            [p0.0, p0.1, offset],
            [p1.0, p1.1, offset],
            [p2.0, p2.1, offset],
            [p3.0, p3.1, offset],
        ],
        RefPlane::Back => [
            [-p0.0, p0.1, -offset],
            [-p1.0, p1.1, -offset],
            [-p2.0, p2.1, -offset],
            [-p3.0, p3.1, -offset],
        ],
        RefPlane::Right | RefPlane::Side => [
            [offset, p0.1, -p0.0],
            [offset, p1.1, -p1.0],
            [offset, p2.1, -p2.0],
            [offset, p3.1, -p3.0],
        ],
        RefPlane::Left => [
            [-offset, p0.1, p0.0],
            [-offset, p1.1, p1.0],
            [-offset, p2.1, p2.0],
            [-offset, p3.1, p3.0],
        ],
        RefPlane::Top => [
            [p0.0, offset, -p0.1],
            [p1.0, offset, -p1.1],
            [p2.0, offset, -p2.1],
            [p3.0, offset, -p3.1],
        ],
        RefPlane::Bottom => [
            [p0.0, -offset, p0.1],
            [p1.0, -offset, p1.1],
            [p2.0, -offset, p2.1],
            [p3.0, -offset, p3.1],
        ],
    }
}

/// UVs padrão do quad (origem em cima, como RGBA de `image`).
pub const QUAD_UVS_TOP_LEFT: [[f32; 2]; 4] = [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]];

/// UVs com V flipado (origem GL embaixo).
pub const QUAD_UVS_GL: [[f32; 2]; 4] = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
