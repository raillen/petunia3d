//! Renderer wgpu: malha sombreada + arestas + grid estilo Blender + quads de referência.
//! Propositalmente simples: reconstrói os buffers por frame (ok para low-poly).

use bytemuck::{Pod, Zeroable};
use glam::Vec3;
use wgpu::util::DeviceExt;

use petunia_core::Camera;
use petunia_core::RefAxis;
use petunia_project::Project;
use petunia_render::Shading;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct MeshVertex {
    pos: [f32; 3],
    normal: [f32; 3],
    color: [f32; 3],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct LineVertex {
    pos: [f32; 3],
    color: [f32; 3],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct RefVertex {
    pos: [f32; 3],
    uv: [f32; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct RefUniform {
    opacity: f32,
    _pad: [f32; 3],
}

struct RefGpu {
    width: u32,
    height: u32,
    len: usize,
    hash: std::cell::Cell<u64>,
    texture: wgpu::Texture,
    /// Mantida viva junto ao bind group (wgpu é ref-counted, mas explícito é mais seguro).
    #[allow(dead_code)]
    view: wgpu::TextureView,
    params: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

pub struct Renderer {
    depth_format: wgpu::TextureFormat,
    depth_view: Option<wgpu::TextureView>,
    depth_size: (u32, u32),
    mesh_pipeline: wgpu::RenderPipeline,
    mesh_xray_pipeline: wgpu::RenderPipeline,
    line_pipeline: wgpu::RenderPipeline,
    line_xray_pipeline: wgpu::RenderPipeline,
    xray: bool,
    ref_pipeline: wgpu::RenderPipeline,
    ref_xray_pipeline: wgpu::RenderPipeline,
    cam_buffer: wgpu::Buffer,
    cam_bind_group: wgpu::BindGroup,
    ref_tex_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    mesh_vb: Option<wgpu::Buffer>,
    mesh_count: u32,
    line_vb: Option<wgpu::Buffer>,
    line_count: u32,
    grid_vb: wgpu::Buffer,
    grid_count: u32,
    ref_vb: Option<wgpu::Buffer>,
    ref_count: u32,
    ref_gpu: Vec<RefGpu>,
}

const MESH_WGSL: &str = r#"
struct Camera { view_proj: mat4x4<f32> };
@group(0) @binding(0) var<uniform> cam: Camera;

struct In {
    @location(0) pos: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
};
struct Out {
    @builtin(position) clip: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) wpos: vec3<f32>,
};
@vertex
fn vs_main(in: In) -> Out {
    var o: Out;
    o.clip = cam.view_proj * vec4<f32>(in.pos, 1.0);
    o.normal = in.normal;
    o.color = in.color;
    o.wpos = in.pos;
    return o;
}

@fragment
fn fs_main(in: Out) -> @location(0) vec4<f32> {
    if (length(in.normal) < 0.1) {
        return vec4<f32>(in.color, 1.0);
    }
    let light = normalize(vec3<f32>(LIGHT_X, LIGHT_Y, LIGHT_Z));
    let n = normalize(in.normal);
    let diff = max(dot(n, light), 0.0);
    let amb = LIGHT_AMB;
    let c = in.color * (amb + LIGHT_DIF * diff);
    return vec4<f32>(c, 1.0);
}

@fragment
fn fs_xray(in: Out) -> @location(0) vec4<f32> {
    if (length(in.normal) < 0.1) {
        return vec4<f32>(in.color, 0.45);
    }
    let light = normalize(vec3<f32>(LIGHT_X, LIGHT_Y, LIGHT_Z));
    let n = normalize(in.normal);
    let diff = max(dot(n, light), 0.0);
    let amb = LIGHT_AMB;
    let c = in.color * (amb + LIGHT_DIF * diff);
    return vec4<f32>(c, 0.45);
}
"#;

/// Monta o shader da malha com as constantes de luz compartilhadas
/// (`render::scene`), mantendo uma fonte só.
fn mesh_wgsl() -> String {
    MESH_WGSL
        .replace("LIGHT_X", &petunia_render::scene::LIGHT_DIR[0].to_string())
        .replace("LIGHT_Y", &petunia_render::scene::LIGHT_DIR[1].to_string())
        .replace("LIGHT_Z", &petunia_render::scene::LIGHT_DIR[2].to_string())
        .replace(
            "LIGHT_AMB",
            &petunia_render::scene::LIGHT_AMBIENT.to_string(),
        )
        .replace(
            "LIGHT_DIF",
            &petunia_render::scene::LIGHT_DIFFUSE.to_string(),
        )
}

const LINE_WGSL: &str = r#"
struct Camera { view_proj: mat4x4<f32> };
@group(0) @binding(0) var<uniform> cam: Camera;

struct In {
    @location(0) pos: vec3<f32>,
    @location(1) color: vec3<f32>,
};
struct Out {
    @builtin(position) clip: vec4<f32>,
    @location(0) color: vec3<f32>,
};
@vertex
fn vs_main(in: In) -> Out {
    var o: Out;
    o.clip = cam.view_proj * vec4<f32>(in.pos, 1.0);
    o.color = in.color;
    return o;
}
@fragment
fn fs_main(in: Out) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
"#;

const REF_WGSL: &str = r#"
struct Camera { view_proj: mat4x4<f32> };
@group(0) @binding(0) var<uniform> cam: Camera;
struct Params { opacity: f32, _p0: f32, _p1: f32, _p2: f32 };
@group(1) @binding(0) var<uniform> params: Params;
@group(1) @binding(1) var tex: texture_2d<f32>;
@group(1) @binding(2) var smp: sampler;

struct In {
    @location(0) pos: vec3<f32>,
    @location(1) uv: vec2<f32>,
};
struct Out {
    @builtin(position) clip: vec4<f32>,
    @location(0) uv: vec2<f32>,
};
@vertex
fn vs_main(in: In) -> Out {
    var o: Out;
    o.clip = cam.view_proj * vec4<f32>(in.pos, 1.0);
    o.uv = in.uv;
    return o;
}
@fragment
fn fs_main(in: Out) -> @location(0) vec4<f32> {
    let c = textureSample(tex, smp, in.uv);
    return vec4<f32>(c.rgb, c.a * params.opacity);
}
"#;

fn grid_lines() -> Vec<LineVertex> {
    petunia_render::scene::grid_lines()
        .into_iter()
        .flat_map(|(a, b, c)| {
            [
                LineVertex { pos: a, color: c },
                LineVertex { pos: b, color: c },
            ]
        })
        .collect()
}

fn fnv1a_hash(bytes: &[u8]) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

impl Renderer {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let cam_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("simple3d-cam"),
            size: std::mem::size_of::<CameraUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let cam_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("simple3d-cam-layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let cam_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("simple3d-cam-bg"),
            layout: &cam_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: cam_buffer.as_entire_binding(),
            }],
        });

        let mesh_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("simple3d-mesh"),
            source: wgpu::ShaderSource::Wgsl(mesh_wgsl().into()),
        });
        let line_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("simple3d-line"),
            source: wgpu::ShaderSource::Wgsl(LINE_WGSL.into()),
        });
        let ref_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("simple3d-ref"),
            source: wgpu::ShaderSource::Wgsl(REF_WGSL.into()),
        });

        let mesh_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("simple3d-mesh-layout"),
            bind_group_layouts: &[&cam_layout],
            push_constant_ranges: &[],
        });
        let mesh_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("simple3d-mesh-pipe"),
            layout: Some(&mesh_layout),
            vertex: wgpu::VertexState {
                module: &mesh_shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<MeshVertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x3],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &mesh_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: None,
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            multiview: None,
            cache: None,
        });

        let line_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("simple3d-line-pipe"),
            layout: Some(&mesh_layout),
            vertex: wgpu::VertexState {
                module: &line_shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<LineVertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &line_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            multiview: None,
            cache: None,
        });

        let mesh_xray_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("simple3d-mesh-xray-pipe"),
            layout: Some(&mesh_layout),
            vertex: wgpu::VertexState {
                module: &mesh_shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<MeshVertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x3],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &mesh_shader,
                entry_point: Some("fs_xray"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: None,
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            multiview: None,
            cache: None,
        });

        let line_xray_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("simple3d-line-xray-pipe"),
            layout: Some(&mesh_layout),
            vertex: wgpu::VertexState {
                module: &line_shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<LineVertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &line_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::Always,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            multiview: None,
            cache: None,
        });

        // refs: layout do grupo 1 (params + textura + sampler)
        let ref_tex_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("simple3d-ref-tex-layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let ref_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("simple3d-ref-layout"),
            bind_group_layouts: &[&cam_layout, &ref_tex_layout],
            push_constant_ranges: &[],
        });
        let ref_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("simple3d-ref-pipe"),
            layout: Some(&ref_layout),
            vertex: wgpu::VertexState {
                module: &ref_shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<RefVertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &ref_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: None,
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            multiview: None,
            cache: None,
        });

        let ref_xray_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("simple3d-ref-xray-pipeline"),
            layout: Some(&ref_layout),
            vertex: wgpu::VertexState {
                module: &ref_shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<RefVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &ref_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: None,
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::Always,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            multiview: None,
            cache: None,
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("simple3d-sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let grid = grid_lines();
        let grid_count = grid.len() as u32;
        let grid_vb = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("simple3d-grid"),
            contents: bytemuck::cast_slice(&grid),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            depth_format: wgpu::TextureFormat::Depth24Plus,
            depth_view: None,
            depth_size: (0, 0),
            mesh_pipeline,
            mesh_xray_pipeline,
            line_pipeline,
            line_xray_pipeline,
            xray: false,
            ref_pipeline,
            ref_xray_pipeline,
            cam_buffer,
            cam_bind_group,
            ref_tex_layout,
            sampler,
            mesh_vb: None,
            mesh_count: 0,
            line_vb: None,
            line_count: 0,
            grid_vb,
            grid_count,
            ref_vb: None,
            ref_count: 0,
            ref_gpu: Vec::new(),
        }
    }

    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        if self.depth_size == (width, height) && self.depth_view.is_some() {
            return;
        }
        let tex = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("simple3d-depth"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.depth_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        self.depth_view = Some(tex.create_view(&Default::default()));
        self.depth_size = (width, height);
    }

    /// Reconstrói buffers da cena. Barato para low-poly; roda por frame.
    #[allow(clippy::too_many_arguments)]
    pub fn update(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        scene: &Project,
        refs: &[petunia_core::ReferenceImage],
        camera: &Camera,
        shading: Shading,
        xray: bool,
        show_triangulation: bool,
    ) {
        self.xray = xray;
        queue.write_buffer(
            &self.cam_buffer,
            0,
            bytemuck::cast_slice(&[CameraUniform {
                view_proj: camera.view_proj().to_cols_array_2d(),
            }]),
        );

        // malha
        let mut mv: Vec<MeshVertex> = Vec::new();
        let mut lv: Vec<LineVertex> = Vec::new();
        let smooth = shading == Shading::Smooth;
        let unlit = shading == Shading::Unlit;
        let is_wire = shading == Shading::Wireframe;
        for obj in &scene.assets {
            if !obj.visible {
                continue;
            }
            if !is_wire {
                let tris = if unlit {
                    obj.mesh.to_triangles_unlit()
                } else {
                    obj.mesh.to_triangles_smooth(smooth)
                };
                for (pos, n, col, _uv) in tris {
                    mv.push(MeshVertex {
                        pos,
                        normal: n,
                        color: col,
                    });
                }
            }
            if is_wire {
                for (a, b, sel) in obj.mesh.to_edges() {
                    let c = if sel {
                        [1.0, 0.3, 0.1]
                    } else {
                        [1.0, 0.6, 0.2]
                    };
                    lv.push(LineVertex { pos: a, color: c });
                    lv.push(LineVertex { pos: b, color: c });
                }
            } else {
                // overlay sutil das arestas (estilo Blender: wire sobre solid)
                for (a, b, sel) in obj.mesh.to_edges() {
                    let c = if sel {
                        [1.0, 0.35, 0.1]
                    } else {
                        [0.05, 0.05, 0.06]
                    };
                    lv.push(LineVertex {
                        pos: [a[0], a[1] + 0.001, a[2]],
                        color: c,
                    });
                    lv.push(LineVertex {
                        pos: [b[0], b[1] + 0.001, b[2]],
                        color: c,
                    });
                }
            }
            if show_triangulation {
                let diag_c = [0.3, 0.65, 0.95];
                let lift = if is_wire { 0.0 } else { 0.0012 };
                for (a, b) in obj.mesh.triangulation_wireframe() {
                    lv.push(LineVertex {
                        pos: [a[0], a[1] + lift, a[2]],
                        color: diag_c,
                    });
                    lv.push(LineVertex {
                        pos: [b[0], b[1] + lift, b[2]],
                        color: diag_c,
                    });
                }
            }
        }
        self.mesh_count = mv.len() as u32;
        self.mesh_vb = if mv.is_empty() {
            None
        } else {
            Some(
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("simple3d-mesh-vb"),
                    contents: bytemuck::cast_slice(&mv),
                    usage: wgpu::BufferUsages::VERTEX,
                }),
            )
        };
        self.line_count = lv.len() as u32;
        self.line_vb = if lv.is_empty() {
            None
        } else {
            Some(
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("simple3d-edge-vb"),
                    contents: bytemuck::cast_slice(&lv),
                    usage: wgpu::BufferUsages::VERTEX,
                }),
            )
        };

        // referências: garante texturas e monta quads (matemática em render::scene)
        self.ensure_ref_textures(device, refs);
        let mut rv: Vec<RefVertex> = Vec::new();
        for r in refs.iter().filter(|r| r.visible) {
            let plane = match r.axis {
                RefAxis::Front => petunia_render::scene::RefPlane::Front,
                RefAxis::Back => petunia_render::scene::RefPlane::Back,
                RefAxis::Left => petunia_render::scene::RefPlane::Left,
                RefAxis::Right | RefAxis::Side => petunia_render::scene::RefPlane::Right,
                RefAxis::Top => petunia_render::scene::RefPlane::Top,
                RefAxis::Bottom => petunia_render::scene::RefPlane::Bottom,
            };
            let aspect = r.width as f32 / r.height.max(1) as f32;
            let quad = petunia_render::scene::ref_quad_with_rot(
                plane, r.offset, r.size, aspect, r.rotation,
            );
            for (p, uv) in quad.iter().zip(petunia_render::scene::QUAD_UVS_TOP_LEFT) {
                rv.push(RefVertex { pos: *p, uv });
            }
        }
        // expande quads (4 verts) p/ 2 tris (6 verts)
        let mut tris: Vec<RefVertex> = Vec::new();
        for q in rv.chunks(4) {
            if q.len() == 4 {
                tris.extend_from_slice(&[q[0], q[1], q[2], q[0], q[2], q[3]]);
            }
        }
        self.ref_count = tris.len() as u32;
        self.ref_vb = if tris.is_empty() {
            None
        } else {
            Some(
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("simple3d-ref-vb"),
                    contents: bytemuck::cast_slice(&tris),
                    usage: wgpu::BufferUsages::VERTEX,
                }),
            )
        };
        let _ = queue;
    }

    fn ensure_ref_textures(
        &mut self,
        device: &wgpu::Device,
        refs: &[petunia_core::ReferenceImage],
    ) {
        // Reconstrói slots cujo tamanho/conteúdo mudou; mantém os demais (evita re-upload).
        while self.ref_gpu.len() < refs.len() {
            self.ref_gpu
                .push(self.make_ref_slot(device, &refs[self.ref_gpu.len()]));
        }
        self.ref_gpu.truncate(refs.len());
        for (i, r) in refs.iter().enumerate() {
            let slot = &self.ref_gpu[i];
            if slot.width != r.width || slot.height != r.height || slot.len != r.rgba.len() {
                self.ref_gpu[i] = self.make_ref_slot(device, r);
            }
        }
    }

    fn make_ref_slot(&self, device: &wgpu::Device, r: &petunia_core::ReferenceImage) -> RefGpu {
        let (w, h) = (r.width.max(1), r.height.max(1));
        let tex = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("simple3d-ref"),
            size: wgpu::Extent3d {
                width: w,
                height: h,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let params = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("simple3d-ref-params-slot"),
            size: std::mem::size_of::<RefUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let view = tex.create_view(&Default::default());
        let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("simple3d-ref-bg"),
            layout: &self.ref_tex_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: params.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        });
        RefGpu {
            width: r.width,
            height: r.height,
            len: r.rgba.len(),
            hash: std::cell::Cell::new(0),
            texture: tex,
            view,
            params,
            bind_group: bg,
        }
    }

    /// Upload dos pixels + opacidade por ref (chamado todo frame; wgpu ignora se igual via hash cache).
    pub fn upload_ref_pixels(&self, queue: &wgpu::Queue, refs: &[petunia_core::ReferenceImage]) {
        for (i, r) in refs.iter().enumerate() {
            let Some(slot) = self.ref_gpu.get(i) else {
                continue;
            };
            let h = fnv1a_hash(&r.rgba);
            if !r.rgba.is_empty()
                && (r.width, r.height) == (slot.width, slot.height)
                && r.rgba.len() == slot.len
                && slot.hash.get() != h
            {
                slot.hash.set(h);
                queue.write_texture(
                    wgpu::TexelCopyTextureInfo {
                        texture: &slot.texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    &r.rgba,
                    wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(4 * r.width),
                        rows_per_image: Some(r.height),
                    },
                    wgpu::Extent3d {
                        width: r.width,
                        height: r.height,
                        depth_or_array_layers: 1,
                    },
                );
            }
            queue.write_buffer(
                &slot.params,
                0,
                bytemuck::cast_slice(&[RefUniform {
                    opacity: if r.visible { r.opacity } else { 0.0 },
                    _pad: [0.0; 3],
                }]),
            );
        }
    }

    pub fn render(&self, pass: &mut wgpu::RenderPass<'_>, refs: &[petunia_core::ReferenceImage]) {
        pass.set_bind_group(0, &self.cam_bind_group, &[]);
        // grid
        pass.set_pipeline(&self.line_pipeline);
        pass.set_vertex_buffer(0, self.grid_vb.slice(..));
        pass.draw(0..self.grid_count, 0..1);

        // Referências padrão (não X-Ray): desenhadas ANTES da geometria sólida com depth test
        if let Some(vb) = &self.ref_vb {
            pass.set_pipeline(&self.ref_pipeline);
            pass.set_vertex_buffer(0, vb.slice(..));
            let mut start = 0u32;
            for (i, r) in refs.iter().enumerate().filter(|(_, r)| r.visible) {
                if !r.xray {
                    if let Some(slot) = self.ref_gpu.get(i) {
                        pass.set_bind_group(1, &slot.bind_group, &[]);
                        pass.draw(start..start + 6, 0..1);
                    }
                }
                start += 6;
            }
        }

        // malha sólida (ou raio-x com transparência)
        if let Some(vb) = &self.mesh_vb {
            if self.xray {
                pass.set_pipeline(&self.mesh_xray_pipeline);
            } else {
                pass.set_pipeline(&self.mesh_pipeline);
            }
            pass.set_vertex_buffer(0, vb.slice(..));
            pass.draw(0..self.mesh_count, 0..1);
        }
        // arestas
        if let Some(vb) = &self.line_vb {
            if self.xray {
                pass.set_pipeline(&self.line_xray_pipeline);
            } else {
                pass.set_pipeline(&self.line_pipeline);
            }
            pass.set_vertex_buffer(0, vb.slice(..));
            pass.draw(0..self.line_count, 0..1);
        }

        // Referências X-Ray: overlay pass desenhado APÓS a geometria com depth test bypass
        if let Some(vb) = &self.ref_vb {
            pass.set_pipeline(&self.ref_xray_pipeline);
            pass.set_vertex_buffer(0, vb.slice(..));
            let mut start = 0u32;
            for (i, r) in refs.iter().enumerate().filter(|(_, r)| r.visible) {
                if r.xray {
                    if let Some(slot) = self.ref_gpu.get(i) {
                        pass.set_bind_group(1, &slot.bind_group, &[]);
                        pass.draw(start..start + 6, 0..1);
                    }
                }
                start += 6;
            }
        }

        let _ = Vec3::ZERO;
    }

    pub fn depth_view(&self) -> Option<&wgpu::TextureView> {
        self.depth_view.as_ref()
    }
}
