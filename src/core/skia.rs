
use skia_safe::gpu::{backend_render_targets, surfaces, DirectContext, Protected, SurfaceOrigin};
use skia_safe::gpu::gl::FramebufferInfo;
use skia_safe::{ColorType, Surface};
use windows::Win32::Graphics::OpenGL::{glGetIntegerv,};
use crate::core::OverlayError;

const GL_FRAMEBUFFERBINDING: u32 = 0x8CA6;

#[derive(Clone)]
pub struct SkiaContext {
    pub(crate) gr_context: DirectContext,
    pub(crate) surface: Surface,
}

impl SkiaContext {
    pub fn new(
        width: i32,
        height: i32
    ) -> Result<Self, OverlayError> {
        let interface = skia_safe::gpu::gl::Interface::new_native()
            .ok_or_else(|| {
                println!("Failed to create native Skia interface");
                OverlayError::FailedToCreateSkiaInterface
            })?;

        println!("Created interface: {:?}", interface);

        let mut gr_context = skia_safe::gpu::direct_contexts::make_gl(interface, None)
            .ok_or_else(|| {
                println!("Failed to create DirectContext");
                OverlayError::FailedToCreateDirectContext
            })?;

        println!("Created DirectContext: {:?}", gr_context);

        let mut fboid: i32 = 0;
        unsafe {
            glGetIntegerv(GL_FRAMEBUFFERBINDING, &mut fboid);
            println!("Current framebuffer: {}", fboid);
        }

        let frame_buffer_info = FramebufferInfo {
            fboid: fboid as u32,
            format: skia_safe::gpu::gl::Format::RGBA8.into(),
            protected: Protected::No // PLAY WITH THIS LATER! (relates to DRM)
        };

        let backend_render_target = backend_render_targets::make_gl(
            (width, height),
            0,
            0,
            frame_buffer_info
        );

        println!("Created BackendRenderTarget: {:?}", backend_render_target);

        let surface = surfaces::wrap_backend_render_target(
            &mut gr_context,
            &backend_render_target,
            SurfaceOrigin::BottomLeft, // PLAY WITH THIS (TopLeft might be better for this overlay)
            ColorType::RGBA8888,
            None,
            None,
        ).ok_or_else(|| {
            println!("Failed to create Skia surface");
            OverlayError::FailedToCreateSkiaSurface
        })?;

        println!("Created Skia Surface: {:?}", surface);

        Ok(Self {
            gr_context,
            surface,
        })
    }

    pub fn canvas(&mut self) -> &skia_safe::Canvas {
        self.surface.canvas()
    }
}