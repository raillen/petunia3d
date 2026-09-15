//! Renderer OpenGL puro (glow, desktop GL 3.3+) — fallback automático
//! quando o wgpu não encontra GPU (ex. Intel antiga sem Vulkan funcional
//! e com EGL problemático). Mesma cena do renderer wgpu: malha sombreada,
//! arestas, grid estilo Blender e quads de referência texturizados.
//!
//! Uso: `GlWindow::create(...)` uma vez; `GlRenderer::new(gl)` uma vez;
//! `draw(&mut state, w, h)` por frame.

#![allow(unsafe_op_in_unsafe_fn)]

pub mod bootstrap;

pub use bootstrap::GlWindow;

use std::sync::Arc;

use egui_glow::glow::{
    self, HasContext, NativeProgram, NativeTexture, NativeUniformLocation, NativeVertexArray,
    PixelUnpackData,
};

use petunia_core::RefAxis;
use petunia_render::Shading;

const MESH_VS: &str = "#version 330 core
layout(location = 0) in vec3 aPos;
layout(location = 1) in vec3 aNrm;
layout(location = 2) in vec3 aCol;
uniform mat4 vp;
out vec3 vN; out vec3 vC;
void main() { gl_Position = vp * vec4(aPos, 1.0); vN = aNrm; vC = aCol; }
";
const MESH_FS_TMPL: &str = "#version 330 core
in vec3 vN; in vec3 vC; out vec4 o;
uniform float opacity;
void main() {
    if (length(vN) < 0.1) {
        o = vec4(vC, opacity);
        return;
    }
    vec3 L = normalize(vec3(LIGHT_X, LIGHT_Y, LIGHT_Z));
    float d = max(dot(normalize(vN), L), 0.0);
    o = vec4(vC * (LIGHT_AMB + LIGHT_DIF * d), opacity);
}
";
const LINE_VS: &str = "#version 330 core
layout(location = 0) in vec3 aPos;
layout(location = 1) in vec3 aCol;
uniform mat4 vp;
out vec3 vC;
void main() { gl_Position = vp * vec4(aPos, 1.0); vC = aCol; }
";
const LINE_FS: &str = "#version 330 core
in vec3 vC; out vec4 o;
void main() { o = vec4(vC, 1.0); }
";
const REF_VS: &str = "#version 330 core
layout(location = 0) in vec3 aPos;
layout(location = 1) in vec2 aUv;
uniform mat4 vp;
out vec2 vUv;
void main() { gl_Position = vp * vec4(aPos, 1.0); vUv = aUv; }
";
const REF_FS: &str = "#version 330 core
in vec2 vUv; out vec4 o;
uniform sampler2D tex;
uniform float opacity;
void main() { vec4 c = texture(tex, vUv); o = vec4(c.rgb, c.a * opacity); }
";
const TEX_VS: &str = "#version 330 core
layout(location = 0) in vec3 aPos;
layout(location = 1) in vec3 aNrm;
layout(location = 2) in vec3 aCol;
layout(location = 3) in vec2 aUv;
uniform mat4 vp;
out vec3 vN; out vec3 vC; out vec2 vUv;
void main() { gl_Position = vp * vec4(aPos, 1.0); vN = aNrm; vC = aCol; vUv = aUv; }
";
const TEX_FS_TMPL: &str = "#version 330 core
in vec3 vN; in vec3 vC; in vec2 vUv; out vec4 o;
uniform sampler2D tex;
uniform float opacity;
void main() {
    vec3 t = texture(tex, vUv).rgb;
    if (length(vN) < 0.1) {
        o = vec4(t * vC, opacity);
        return;
    }
    vec3 L = normalize(vec3(LIGHT_X, LIGHT_Y, LIGHT_Z));
    float d = max(dot(normalize(vN), L), 0.0);
    o = vec4(t * vC * (LIGHT_AMB + LIGHT_DIF * d) * 2.0, opacity);
}
";

/// Preenche o template GLSL com as constantes de luz compartilhadas.
fn fill_light(tmpl: &str) -> String {
    use petunia_render::scene as S;
    tmpl.replace("LIGHT_X", &S::LIGHT_DIR[0].to_string())
        .replace("LIGHT_Y", &S::LIGHT_DIR[1].to_string())
        .replace("LIGHT_Z", &S::LIGHT_DIR[2].to_string())
        .replace("LIGHT_AMB", &S::LIGHT_AMBIENT.to_string())
        .replace("LIGHT_DIF", &S::LIGHT_DIFFUSE.to_string())
}

struct RefTex {
    w: u32,
    h: u32,
    len: usize,
    tex: NativeTexture,
    /// Hash FNV-1a do último upload (evita re-upload por frame).
    hash: u64,
}

fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

pub struct GlRenderer {
    gl: Arc<glow::Context>,
    /// VAO padrão bound uma vez (core profile exige VAO).
    #[allow(dead_code)]
    vao: NativeVertexArray,
    mesh_prog: NativeProgram,
    mesh_vp: Option<NativeUniformLocation>,
    mesh_opacity: Option<NativeUniformLocation>,
    line_prog: NativeProgram,
    line_vp: Option<NativeUniformLocation>,
    ref_prog: NativeProgram,
    ref_vp: Option<NativeUniformLocation>,
    ref_tex_u: Option<NativeUniformLocation>,
    ref_opacity_u: Option<NativeUniformLocation>,
    tex_prog: NativeProgram,
    tex_vp: Option<NativeUniformLocation>,
    tex_u: Option<NativeUniformLocation>,
    tex_opacity: Option<NativeUniformLocation>,
    ref_tex: Vec<RefTex>,
    /// Texturas dos canvas dos assets (UUID -> slot).
    asset_tex: std::collections::HashMap<uuid::Uuid, RefTex>,
}

impl GlRenderer {
    pub fn new(gl: Arc<glow::Context>) -> Result<Self, String> {
        // SAFETY: the caller supplies the current context on the rendering thread.
        unsafe {
            let mut programs = Vec::new();
            let initialized = (|| {
                let (mesh_prog, mesh_locs) =
                    compile(&gl, MESH_VS, &fill_light(MESH_FS_TMPL), &["vp", "opacity"])?;
                programs.push(mesh_prog);
                let (line_prog, line_locs) = compile(&gl, LINE_VS, LINE_FS, &["vp"])?;
                programs.push(line_prog);
                let (ref_prog, ref_locs) = compile(&gl, REF_VS, REF_FS, &["vp", "tex", "opacity"])?;
                programs.push(ref_prog);
                let (tex_prog, tex_locs) = compile(
                    &gl,
                    TEX_VS,
                    &fill_light(TEX_FS_TMPL),
                    &["vp", "tex", "opacity"],
                )?;
                programs.push(tex_prog);
                let vao = gl
                    .create_vertex_array()
                    .map_err(|error| format!("OpenGL vertex array: {error}"))?;
                gl.bind_vertex_array(Some(vao));
                Ok(Self {
                    gl: Arc::clone(&gl),
                    vao,
                    mesh_prog,
                    mesh_vp: locs_first(&mesh_locs, 0),
                    mesh_opacity: locs_first(&mesh_locs, 1),
                    line_prog,
                    line_vp: locs_first(&line_locs, 0),
                    ref_prog,
                    ref_vp: locs_first(&ref_locs, 0),
                    ref_tex_u: locs_first(&ref_locs, 1),
                    ref_opacity_u: locs_first(&ref_locs, 2),
                    tex_prog,
                    tex_vp: locs_first(&tex_locs, 0),
                    tex_u: locs_first(&tex_locs, 1),
                    tex_opacity: locs_first(&tex_locs, 2),
                    ref_tex: Vec::new(),
                    asset_tex: std::collections::HashMap::new(),
                })
            })();
            if initialized.is_err() {
                for program in programs {
                    gl.delete_program(program);
                }
            }
            initialized
        }
    }

    /// Desenha a cena inteira (viewport + clear + refs + malha + arestas).
    pub fn draw(&mut self, state: &petunia_core::AppState, width: u32, height: u32) {
        let Some(viewport) = petunia_core::viewport::PhysicalViewport::from_logical(
            state.ui.viewport_rect,
            state.ui.viewport_pixels_per_point,
            width,
            height,
        ) else {
            return;
        };
        let vp = state.camera.view_proj().to_cols_array();
        let gl = Arc::clone(&self.gl);
        unsafe {
            // Rebind OBRIGATÓRIO: o egui troca o VAO no paint do frame anterior.
            gl.bind_vertex_array(Some(self.vao));
            gl.disable(glow::SCISSOR_TEST);
            gl.depth_mask(true);
            gl.viewport(
                viewport.x as i32,
                viewport.gl_y(height) as i32,
                viewport.width as i32,
                viewport.height as i32,
            );
            let bg = petunia_config::ThemeRegistry::global()
                .get_theme(&state.ui.active_theme_id)
                .map(|t| {
                    t.colors
                        .get_token_color(petunia_config::ThemeToken::BgCanvas)
                        .to_rgba_f32()
                })
                .unwrap_or([0.117, 0.117, 0.133, 1.0]);
            gl.clear_color(bg[0], bg[1], bg[2], 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            gl.scissor(
                viewport.x as i32,
                viewport.gl_y(height) as i32,
                viewport.width as i32,
                viewport.height as i32,
            );
            gl.enable(glow::SCISSOR_TEST);
            gl.enable(glow::DEPTH_TEST);
            gl.depth_func(glow::LESS);

            // --- referências normais (atrás, sem escrever depth) ---
            gl.depth_mask(false);
            self.draw_refs(state, &vp, false);
            gl.depth_mask(true);

            // prune texturas de assets removidos (evita leak de VRAM)
            {
                let live: std::collections::HashSet<_> =
                    state.project.assets.iter().map(|a| a.id).collect();
                self.asset_tex.retain(|id, slot| {
                    let keep = live.contains(id);
                    if !keep {
                        gl.delete_texture(slot.tex);
                    }
                    keep
                });
            }

            // --- malha sólida / wireframe ---
            self.draw_mesh(state, &vp);

            // --- arestas por cima ---
            gl.depth_mask(false);
            self.set_line_vp(&vp);
            self.draw_edges(state);
            gl.depth_mask(true);

            // --- referências X-ray (overlay por cima da malha) ---
            gl.depth_mask(false);
            self.draw_refs(state, &vp, true);
            gl.depth_mask(true);

            // estado limpo p/ o egui (que configura o seu próprio)
            gl.use_program(None);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.disable(glow::BLEND);
            gl.disable(glow::SCISSOR_TEST);
            gl.disable(glow::DEPTH_TEST);
        }
    }

    unsafe fn draw_mesh(&mut self, state: &petunia_core::AppState, vp: &[f32; 16]) {
        let gl = Arc::clone(&self.gl);
        if state.shading == Shading::Wireframe {
            return; // wireframe sai só nas arestas
        }
        let smooth = state.shading == Shading::Smooth;
        let unlit = state.shading == Shading::Unlit;
        let opacity: f32 = if state.show_xray { 0.45 } else { 1.0 };
        if state.show_xray {
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
            gl.depth_mask(false);
        } else {
            gl.disable(glow::BLEND);
            gl.depth_mask(true);
        }
        for obj in &state.project.assets {
            if !obj.visible {
                continue;
            }
            let (mat_profile, mat_color, has_emission, emission_color) =
                if let Some(mat) = obj.material(&state.project.project) {
                    (
                        mat.profile,
                        [mat.base_color[0], mat.base_color[1], mat.base_color[2]],
                        mat.emission_strength > 0.0,
                        [
                            mat.emission_color[0] * mat.emission_strength,
                            mat.emission_color[1] * mat.emission_strength,
                            mat.emission_color[2] * mat.emission_strength,
                        ],
                    )
                } else {
                    (
                        petunia_project::ShaderProfile::Pbr,
                        obj.base_color,
                        false,
                        [0.0, 0.0, 0.0],
                    )
                };

            let tex_canvas = obj.texture.as_ref().or_else(|| {
                obj.material(&state.project.project)
                    .and_then(|m| m.albedo_texture.as_ref())
            });
            let use_tex = state.textured && tex_canvas.is_some();
            let mut data: Vec<f32> = Vec::new();
            let obj_unlit = unlit
                || mat_profile == petunia_project::ShaderProfile::Unlit
                || mat_profile == petunia_project::ShaderProfile::Emissive;
            let triangles = if obj_unlit {
                obj.mesh.to_triangles_unlit()
            } else {
                obj.mesh.to_triangles_smooth(smooth)
            };
            for (pos, n, mut col, uv) in triangles {
                if (col[0] - 0.72).abs() < 0.02
                    && (col[1] - 0.73).abs() < 0.02
                    && (col[2] - 0.78).abs() < 0.02
                {
                    col = mat_color;
                }
                if has_emission {
                    col = [
                        (col[0] + emission_color[0]).min(1.0),
                        (col[1] + emission_color[1]).min(1.0),
                        (col[2] + emission_color[2]).min(1.0),
                    ];
                }
                data.extend_from_slice(&pos);
                data.extend_from_slice(&n);
                data.extend_from_slice(&col);
                if use_tex {
                    data.extend_from_slice(&uv);
                }
            }
            if data.is_empty() {
                continue;
            }
            let stride: usize = if use_tex { 11 } else { 9 };
            if use_tex {
                if let Some(canvas) = tex_canvas {
                    let slot = match self.asset_tex_slot(
                        &gl,
                        obj.id,
                        canvas.w,
                        canvas.h,
                        &canvas.pixels,
                    ) {
                        Ok(texture) => texture,
                        Err(error) => {
                            eprintln!("petunia3d: {error}");
                            continue;
                        }
                    };
                    gl.use_program(Some(self.tex_prog));
                    gl.uniform_matrix_4_f32_slice(self.tex_vp.as_ref(), false, vp);
                    gl.uniform_1_f32(self.tex_opacity.as_ref(), opacity);
                    gl.uniform_1_i32(self.tex_u.as_ref(), 0);
                    gl.active_texture(glow::TEXTURE0);
                    gl.bind_texture(glow::TEXTURE_2D, Some(slot));
                } else {
                    continue;
                }
            } else {
                gl.use_program(Some(self.mesh_prog));
                gl.uniform_matrix_4_f32_slice(self.mesh_vp.as_ref(), false, vp);
                gl.uniform_1_f32(self.mesh_opacity.as_ref(), opacity);
            }
            let vbo = match gl.create_buffer() {
                Ok(buffer) => buffer,
                Err(error) => {
                    eprintln!("petunia3d: OpenGL draw buffer: {error}");
                    continue;
                }
            };
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&data),
                glow::DYNAMIC_DRAW,
            );
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, (stride * 4) as i32, 0);
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, (stride * 4) as i32, 3 * 4);
            gl.enable_vertex_attrib_array(2);
            gl.vertex_attrib_pointer_f32(2, 3, glow::FLOAT, false, (stride * 4) as i32, 6 * 4);
            if use_tex {
                gl.enable_vertex_attrib_array(3);
                gl.vertex_attrib_pointer_f32(3, 2, glow::FLOAT, false, (stride * 4) as i32, 9 * 4);
            } else {
                gl.disable_vertex_attrib_array(3);
            }
            gl.draw_arrays(glow::TRIANGLES, 0, (data.len() / stride) as i32);
            gl.delete_buffer(vbo);
        }
        gl.disable(glow::BLEND);
        gl.depth_mask(true);
        gl.bind_texture(glow::TEXTURE_2D, None);
    }

    /// Slot de textura do canvas (recria se mudou); retorna a textura GL.
    unsafe fn asset_tex_slot(
        &mut self,
        gl: &glow::Context,
        id: uuid::Uuid,
        w: u32,
        h: u32,
        pixels: &[u8],
    ) -> Result<NativeTexture, String> {
        let need = match self.asset_tex.get(&id) {
            Some(s) => s.w != w || s.h != h || s.len != pixels.len(),
            None => true,
        };
        if need {
            let tex = gl
                .create_texture()
                .map_err(|error| format!("OpenGL asset texture: {error}"))?;
            if let Some(old) = self.asset_tex.remove(&id) {
                gl.delete_texture(old.tex);
            }
            gl.bind_texture(glow::TEXTURE_2D, Some(tex));
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_S,
                glow::CLAMP_TO_EDGE as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_T,
                glow::CLAMP_TO_EDGE as i32,
            );
            // placeholder; upload real abaixo
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA8 as i32,
                1.max(w as i32),
                1.max(h as i32),
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                PixelUnpackData::Slice(None),
            );
            self.asset_tex.insert(
                id,
                RefTex {
                    w,
                    h,
                    len: pixels.len(),
                    tex,
                    hash: 0,
                },
            );
        }
        // upload só se o conteúdo mudou (dirty por hash — §36)
        let h = fnv1a(pixels);
        let slot = self
            .asset_tex
            .get_mut(&id)
            .ok_or_else(|| "OpenGL texture cache entry missing".to_owned())?;
        if slot.hash != h {
            gl.bind_texture(glow::TEXTURE_2D, Some(slot.tex));
            gl.tex_sub_image_2d(
                glow::TEXTURE_2D,
                0,
                0,
                0,
                w as i32,
                h as i32,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                PixelUnpackData::Slice(Some(pixels)),
            );
            slot.hash = h;
        }
        Ok(slot.tex)
    }

    unsafe fn draw_edges(&mut self, state: &petunia_core::AppState) {
        let gl = &self.gl;
        let wire = state.shading == Shading::Wireframe;
        let show_edges =
            wire || state.mode == petunia_core::EditMode::Edit || state.show_wireframe_overlay;
        let mut data: Vec<f32> = Vec::new();
        for obj in &state.project.assets {
            if !obj.visible {
                continue;
            }
            if show_edges {
                for (a, b, sel) in obj.mesh.to_edges() {
                    let c = if sel {
                        [1.0, 0.35, 0.1]
                    } else if wire {
                        [1.0, 0.6, 0.2]
                    } else {
                        [0.05, 0.05, 0.06]
                    };
                    // pequeno lift p/ não z-fightar com a malha (igual ao wgpu)
                    let lift = if wire { 0.0 } else { 0.001 };
                    data.extend_from_slice(&[a[0], a[1] + lift, a[2]]);
                    data.extend_from_slice(&c);
                    data.extend_from_slice(&[b[0], b[1] + lift, b[2]]);
                    data.extend_from_slice(&c);
                }
            } else {
                for (a, b, sel) in obj.mesh.to_edges() {
                    if sel {
                        let c = [1.0, 0.35, 0.1];
                        let lift = 0.001;
                        data.extend_from_slice(&[a[0], a[1] + lift, a[2]]);
                        data.extend_from_slice(&c);
                        data.extend_from_slice(&[b[0], b[1] + lift, b[2]]);
                        data.extend_from_slice(&c);
                    }
                }
            }
            if state.show_triangulation {
                let diag_c = [0.3, 0.65, 0.95];
                for (a, b) in obj.mesh.triangulation_wireframe() {
                    let lift = if wire { 0.0 } else { 0.0012 };
                    data.extend_from_slice(&[a[0], a[1] + lift, a[2]]);
                    data.extend_from_slice(&diag_c);
                    data.extend_from_slice(&[b[0], b[1] + lift, b[2]]);
                    data.extend_from_slice(&diag_c);
                }
            }
        }

        // Grid 3D obedecendo show_overlays && show_grid
        if state.show_overlays && state.show_grid {
            let gs = &state.grid_settings;
            for (a, b, c) in petunia_render::scene::grid_lines_custom(
                gs.size,
                gs.subdivisions,
                gs.opacity,
                gs.show_isometric_guide,
                gs.isometric_angle_deg,
            ) {
                data.extend_from_slice(&a);
                data.extend_from_slice(&c);
                data.extend_from_slice(&b);
                data.extend_from_slice(&c);
            }
        }

        // Eixos Mundiais obedecendo show_overlays && show_axes
        if state.show_overlays && state.show_axes {
            let extent = state.grid_settings.size.max(10.0);
            for (a, b, c) in petunia_render::scene::world_axes_lines(extent) {
                data.extend_from_slice(&a);
                data.extend_from_slice(&c);
                data.extend_from_slice(&b);
                data.extend_from_slice(&c);
            }
        }

        if data.is_empty() {
            return;
        }
        gl.use_program(Some(self.line_prog));
        let vbo = match gl.create_buffer() {
            Ok(buffer) => buffer,
            Err(error) => {
                eprintln!("petunia3d: OpenGL edge buffer: {error}");
                return;
            }
        };
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            bytemuck::cast_slice(&data),
            glow::DYNAMIC_DRAW,
        );
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 6 * 4, 0);
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, 6 * 4, 3 * 4);
        gl.draw_arrays(glow::LINES, 0, (data.len() / 6) as i32);
        gl.delete_buffer(vbo);
    }

    /// Seta o `vp` do programa de linhas (chamado dentro de draw()).
    unsafe fn set_line_vp(&self, vp: &[f32; 16]) {
        self.gl.use_program(Some(self.line_prog));
        self.gl
            .uniform_matrix_4_f32_slice(self.line_vp.as_ref(), false, vp);
    }

    unsafe fn draw_refs(
        &mut self,
        state: &petunia_core::AppState,
        vp: &[f32; 16],
        xray_pass: bool,
    ) {
        if !xray_pass {
            self.sync_ref_textures(&state.project.refs);
        }
        let gl = Arc::clone(&self.gl);
        // quads via matemática compartilhada (V flipado: origem GL é embaixo)
        use petunia_render::scene as S;
        let mut quads: Vec<(usize, [f32; 30])> = Vec::new();
        for (i, r) in state.project.refs.iter().enumerate() {
            if !r.visible || r.xray != xray_pass {
                continue;
            }
            let plane = match r.axis {
                RefAxis::Front => S::RefPlane::Front,
                RefAxis::Back => S::RefPlane::Back,
                RefAxis::Left => S::RefPlane::Left,
                RefAxis::Right | RefAxis::Side => S::RefPlane::Right,
                RefAxis::Top => S::RefPlane::Top,
                RefAxis::Bottom => S::RefPlane::Bottom,
            };
            let aspect = r.width as f32 / r.height.max(1) as f32;
            let q = S::ref_quad_with_rot(plane, r.offset, r.size, aspect, r.rotation);
            let mut v = [0.0f32; 30];
            for (k, (p, uv)) in q.iter().zip(S::QUAD_UVS_GL).enumerate() {
                v[k * 5..k * 5 + 3].copy_from_slice(p);
                v[k * 5 + 3..k * 5 + 5].copy_from_slice(&uv);
            }
            // 2 tris: 0,1,2  0,2,3
            let mut t = [0.0f32; 30];
            t[0..5].copy_from_slice(&v[0..5]);
            t[5..10].copy_from_slice(&v[5..10]);
            t[10..15].copy_from_slice(&v[10..15]);
            t[15..20].copy_from_slice(&v[0..5]);
            t[20..25].copy_from_slice(&v[10..15]);
            t[25..30].copy_from_slice(&v[15..20]);
            quads.push((i, t));
        }
        if quads.is_empty() {
            return;
        }
        gl.use_program(Some(self.ref_prog));
        gl.uniform_matrix_4_f32_slice(self.ref_vp.as_ref(), false, vp);
        gl.uniform_1_i32(self.ref_tex_u.as_ref(), 0);
        gl.enable(glow::BLEND);
        gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
        gl.active_texture(glow::TEXTURE0);
        for (i, t) in &quads {
            let (tex, opacity) = match (self.ref_tex.get(*i), state.project.refs.get(*i)) {
                (Some(s), Some(r)) => (s.tex, r.opacity),
                _ => continue,
            };
            if xray_pass {
                gl.disable(glow::DEPTH_TEST);
            } else {
                gl.enable(glow::DEPTH_TEST);
            }
            gl.bind_texture(glow::TEXTURE_2D, Some(tex));
            gl.uniform_1_f32(self.ref_opacity_u.as_ref(), opacity);
            let vbo = match gl.create_buffer() {
                Ok(buffer) => buffer,
                Err(error) => {
                    eprintln!("petunia3d: OpenGL draw buffer: {error}");
                    continue;
                }
            };
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&t[..]),
                glow::DYNAMIC_DRAW,
            );
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 5 * 4, 0);
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, 5 * 4, 3 * 4);
            gl.draw_arrays(glow::TRIANGLES, 0, 6);
            gl.delete_buffer(vbo);
        }
        gl.enable(glow::DEPTH_TEST);
        gl.bind_texture(glow::TEXTURE_2D, None);
    }

    /// Garante 1 textura GL por imagem de referência (recria se mudou).
    unsafe fn sync_ref_textures(&mut self, refs: &[petunia_core::ReferenceImage]) {
        let gl = &self.gl;
        while self.ref_tex.len() < refs.len() {
            let tex = match gl.create_texture() {
                Ok(texture) => texture,
                Err(error) => {
                    eprintln!("petunia3d: OpenGL reference texture: {error}");
                    gl.bind_texture(glow::TEXTURE_2D, None);
                    return;
                }
            };
            gl.bind_texture(glow::TEXTURE_2D, Some(tex));
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_S,
                glow::CLAMP_TO_EDGE as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_T,
                glow::CLAMP_TO_EDGE as i32,
            );
            // placeholder 1x1
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA8 as i32,
                1,
                1,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                PixelUnpackData::Slice(Some(&[128, 128, 128, 255])),
            );
            self.ref_tex.push(RefTex {
                w: 0,
                h: 0,
                len: usize::MAX,
                tex,
                hash: 0,
            });
        }
        if self.ref_tex.len() > refs.len() {
            for s in self.ref_tex.drain(refs.len()..) {
                gl.delete_texture(s.tex);
            }
        }
        for (i, r) in refs.iter().enumerate() {
            let slot = &mut self.ref_tex[i];
            if slot.w != r.width || slot.h != r.height || slot.len != r.rgba.len() {
                if r.rgba.is_empty() {
                    continue;
                }
                gl.bind_texture(glow::TEXTURE_2D, Some(slot.tex));
                gl.tex_image_2d(
                    glow::TEXTURE_2D,
                    0,
                    glow::RGBA8 as i32,
                    r.width as i32,
                    r.height as i32,
                    0,
                    glow::RGBA,
                    glow::UNSIGNED_BYTE,
                    PixelUnpackData::Slice(Some(&r.rgba)),
                );
                slot.w = r.width;
                slot.h = r.height;
                slot.len = r.rgba.len();
            }
        }
        gl.bind_texture(glow::TEXTURE_2D, None);
    }
}

fn locs_first(locs: &[Option<NativeUniformLocation>], i: usize) -> Option<NativeUniformLocation> {
    locs.get(i).copied().flatten()
}

type CompiledProgram = (NativeProgram, Vec<Option<NativeUniformLocation>>);

unsafe fn compile(
    gl: &glow::Context,
    vs_src: &str,
    fs_src: &str,
    uniforms: &[&str],
) -> Result<CompiledProgram, String> {
    // SAFETY: this helper only runs with the renderer's current GL context.
    unsafe {
        let vs = compile_shader(gl, glow::VERTEX_SHADER, vs_src)?;
        let fs = match compile_shader(gl, glow::FRAGMENT_SHADER, fs_src) {
            Ok(shader) => shader,
            Err(error) => {
                gl.delete_shader(vs);
                return Err(error);
            }
        };
        let prog = match gl.create_program() {
            Ok(program) => program,
            Err(error) => {
                gl.delete_shader(vs);
                gl.delete_shader(fs);
                return Err(format!("OpenGL program allocation: {error}"));
            }
        };
        gl.attach_shader(prog, vs);
        gl.attach_shader(prog, fs);
        gl.link_program(prog);
        let linked = gl.get_program_link_status(prog);
        let log = if linked {
            String::new()
        } else {
            gl.get_program_info_log(prog)
        };
        gl.detach_shader(prog, vs);
        gl.detach_shader(prog, fs);
        gl.delete_shader(vs);
        gl.delete_shader(fs);
        if !linked {
            gl.delete_program(prog);
            return Err(format!("OpenGL shader link: {log}"));
        }
        let locs = uniforms
            .iter()
            .map(|uniform| gl.get_uniform_location(prog, uniform))
            .collect();
        Ok((prog, locs))
    }
}

unsafe fn compile_shader(
    gl: &glow::Context,
    stage: u32,
    source: &str,
) -> Result<glow::NativeShader, String> {
    // SAFETY: stage is a GL shader constant and the context is current on this thread.
    unsafe {
        let shader = gl
            .create_shader(stage)
            .map_err(|error| format!("OpenGL shader allocation: {error}"))?;
        gl.shader_source(shader, source);
        gl.compile_shader(shader);
        if !gl.get_shader_compile_status(shader) {
            let log = gl.get_shader_info_log(shader);
            gl.delete_shader(shader);
            return Err(format!("OpenGL shader compilation (stage {stage}): {log}"));
        }
        Ok(shader)
    }
}
