//! Bootstrap OpenGL (glutin + winit): criação de contexto desktop GL 3.3+,
//! surface e loader. Todo conhecimento de plataforma GL vive aqui —
//! `app` só usa `GlWindow` (resize/swap/janela), nunca glutin direto.

use std::num::NonZeroU32;
use std::sync::Arc;

use egui_glow::glow::{self, HasContext as _};
use glutin::config::{ConfigTemplateBuilder, GlConfig};
use glutin::context::{
    ContextApi, ContextAttributesBuilder, GlProfile, NotCurrentGlContext as _, Version,
};
use glutin::display::{Display, DisplayApiPreference, GlDisplay};
use glutin::surface::{GlSurface as _, SurfaceAttributesBuilder, WindowSurface};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes};

use petunia_render::GpuCaps;

/// Janela + contexto + surface OpenGL prontos para desenhar.
pub struct GlWindow {
    surface: glutin::surface::Surface<WindowSurface>,
    context: glutin::context::PossiblyCurrentContext,
    gl: Arc<glow::Context>,
    // Native window must outlive surface/context destruction (fields drop in order).
    window: Window,
}

impl GlWindow {
    /// Cria janela, contexto Core >= 3.3 e surface. Falha com mensagem clara.
    pub fn create(event_loop: &ActiveEventLoop, attrs: WindowAttributes) -> Result<Self, String> {
        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_depth_size(24)
            .with_stencil_size(8);
        // WGL needs a native window before it can enumerate modern configurations.
        #[cfg(target_os = "windows")]
        let window = event_loop
            .create_window(attrs)
            .map_err(|error| format!("OpenGL window: {error}"))?;
        #[cfg(target_os = "windows")]
        let native = window
            .window_handle()
            .map_err(|error| format!("OpenGL window handle: {error}"))?
            .as_raw();
        #[cfg(target_os = "windows")]
        let template = template.compatible_with_native_window(native);
        #[cfg(target_os = "windows")]
        let preference = DisplayApiPreference::WglThenEgl(Some(native));
        #[cfg(target_os = "macos")]
        let preference = DisplayApiPreference::Cgl;
        #[cfg(all(unix, not(target_os = "macos"), not(target_os = "android")))]
        let preference = DisplayApiPreference::GlxThenEgl(Box::new(
            winit::platform::x11::register_xlib_error_hook,
        ));
        #[cfg(target_os = "android")]
        let preference = DisplayApiPreference::Egl;
        let display_handle = event_loop
            .display_handle()
            .map_err(|error| format!("OpenGL display handle: {error}"))?;
        // SAFETY: the event loop owns a live display for the complete window lifetime.
        let gl_display = unsafe { Display::new(display_handle.as_raw(), preference) }
            .map_err(|error| format!("OpenGL display: {error}"))?;
        // Use the fallible display API directly: DisplayBuilder's picker cannot
        // represent an empty configuration list without unwinding.
        // SAFETY: the template references only the live native window above, if needed.
        let gl_config = unsafe { gl_display.find_configs(template.build()) }
            .map_err(|error| format!("OpenGL configurations: {error}"))?
            .max_by_key(|config| config.num_samples())
            .ok_or_else(|| "OpenGL: no compatible framebuffer configuration".to_owned())?;
        #[cfg(not(target_os = "windows"))]
        let window = glutin_winit::finalize_window(event_loop, attrs, &gl_config)
            .map_err(|error| format!("OpenGL window: {error}"))?;
        let raw_window_handle = window
            .window_handle()
            .map_err(|error| format!("OpenGL window handle: {error}"))?
            .as_raw();
        let ctx_attrs = ContextAttributesBuilder::new()
            .with_profile(GlProfile::Core)
            .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 3))))
            .build(Some(raw_window_handle));
        // SAFETY: config and context attributes belong to this live display/window.
        let not_current = unsafe { gl_display.create_context(&gl_config, &ctx_attrs) }
            .map_err(|error| format!("OpenGL 3.3 context: {error}"))?;
        let (w, h): (u32, u32) = window.inner_size().into();
        let surf_attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle,
            NonZeroU32::new(w).unwrap_or(NonZeroU32::MIN),
            NonZeroU32::new(h).unwrap_or(NonZeroU32::MIN),
        );
        // SAFETY: the native window remains owned by GlWindow for this surface's lifetime.
        let surface = unsafe { gl_display.create_window_surface(&gl_config, &surf_attrs) }
            .map_err(|error| format!("OpenGL surface: {error}"))?;
        let context = not_current
            .make_current(&surface)
            .map_err(|error| format!("OpenGL make current: {error}"))?;

        let vsync_off = std::env::var("SIMPLE3D_VSYNC").is_ok_and(|value| value == "0");
        let interval = if vsync_off {
            glutin::surface::SwapInterval::DontWait
        } else {
            glutin::surface::SwapInterval::Wait(NonZeroU32::MIN)
        };
        if let Err(error) = surface.set_swap_interval(&context, interval) {
            eprintln!("petunia3d: swap interval: {error}");
        }
        // SAFETY: the context is current on this thread and the loader uses its display.
        let gl = unsafe {
            Arc::new(glow::Context::from_loader_function(|symbol| {
                std::ffi::CString::new(symbol).map_or(std::ptr::null(), |symbol| {
                    gl_display.get_proc_address(symbol.as_c_str())
                })
            }))
        };
        Ok(Self {
            window,
            surface,
            context,
            gl,
        })
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn gl(&self) -> &Arc<glow::Context> {
        &self.gl
    }

    pub fn size(&self) -> (u32, u32) {
        let s = self.window.inner_size();
        (s.width.max(1), s.height.max(1))
    }

    pub fn resize(&self, w: u32, h: u32) {
        if let (Some(w), Some(h)) = (NonZeroU32::new(w), NonZeroU32::new(h)) {
            self.surface.resize(&self.context, w, h);
        }
    }

    pub fn swap(&self) {
        if let Err(e) = self.surface.swap_buffers(&self.context) {
            eprintln!("petunia3d: swap_buffers: {e:?}");
        }
    }

    /// Capabilities detectadas (§29).
    pub fn caps(&self) -> GpuCaps {
        // SAFETY: GlWindow keeps its GL context current on the rendering thread.
        unsafe {
            GpuCaps {
                gl_version: self.gl.get_parameter_string(glow::VERSION),
                glsl_version: self.gl.get_parameter_string(glow::SHADING_LANGUAGE_VERSION),
                vendor: self.gl.get_parameter_string(glow::VENDOR),
                renderer: self.gl.get_parameter_string(glow::RENDERER),
                max_texture_size: self.gl.get_parameter_i32(glow::MAX_TEXTURE_SIZE),
            }
        }
    }
}
