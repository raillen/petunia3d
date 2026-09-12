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

/// Grid estilo Blender no plano XZ (semi-eixo 20).
pub fn grid_lines() -> Vec<ColoredLine> {
    let mut v = Vec::new();
    let n = 20i32;
    let minor = [0.22, 0.22, 0.24];
    let axis_x = [0.75, 0.25, 0.30];
    let axis_z = [0.25, 0.45, 0.75];
    for i in -n..=n {
        let f = i as f32;
        let cx = if i == 0 { axis_x } else { minor };
        let cz = if i == 0 { axis_z } else { minor };
        v.push(([f, 0.0, -(n as f32)], [f, 0.0, n as f32], cx));
        v.push(([-(n as f32), 0.0, f], [n as f32, 0.0, f], cz));
    }
    v
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
