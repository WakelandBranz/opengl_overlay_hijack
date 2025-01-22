mod vsync;

use windows::Win32::Foundation::{GetLastError, HWND};
use windows::Win32::Graphics::Gdi::{GetDC, ReleaseDC, HDC, WGL_SWAP_MAIN_PLANE};
use windows::Win32::Graphics::OpenGL::{PFD_DRAW_TO_WINDOW, PFD_SUPPORT_OPENGL, PFD_TYPE_RGBA, PIXELFORMATDESCRIPTOR, PFD_DOUBLEBUFFER, ChoosePixelFormat, SetPixelFormat, wglCreateContext, wglMakeCurrent, HGLRC, wglDeleteContext, GetPixelFormat, DescribePixelFormat, wglSwapLayerBuffers};
use crate::core::gl::vsync::VsyncState;
use crate::core::OverlayError;

#[derive(Clone)]
pub struct GlContext {
    pub window_handle: HWND,
    pub device_context: HDC,
    pub gl_context: HGLRC,
    vsync_state: VsyncState,
}

impl GlContext {
    pub fn new(hwnd: HWND) -> Result<Self, OverlayError> {
        unsafe {
            // Get device context
            let device_context = GetDC(Some(hwnd));
            if device_context.is_invalid() {
                return Err(OverlayError::FailedToGetDeviceContext);
            };

            // Set up pixel format
            let pfd = PIXELFORMATDESCRIPTOR {
                nSize: std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16,
                nVersion: 1,
                dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
                iPixelType: PFD_TYPE_RGBA,
                cColorBits: 32,
                cAlphaBits: 8,
                //cDepthBits: 24,
                //cStencilBits: 8,
                //iLayerType: 0,
                ..Default::default()
            };

            let pixel_format = ChoosePixelFormat(device_context, &pfd);
            if pixel_format == 0 {
                println!("Failed to choose pixel format");
                return Err(OverlayError::GlContextSetupFailed);
            };

            SetPixelFormat(device_context, pixel_format, &pfd)
                .map_err(|e| {
                    println!("Failed to set pixel format: {:?}", e);
                    OverlayError::FailedToSetPixelFormat
                })?;

            let gl_context = wglCreateContext(device_context)
                .map_err(|e| {
                    println!("Failed to create OpenGL Context: {:?}", e);
                    OverlayError::FailedToCreateOpenGLContext
                })?;

            println!("GL Context: {:?}", gl_context);

            wglMakeCurrent(device_context, gl_context)
                .map_err(|e| {
                    println!("Failed to make GL context current: {:?}", e);
                    OverlayError::FailedToMakeOpenGLContextCurrent
                })?;

            let mut vsync_state = VsyncState::new();
            vsync_state.init().expect("Failed to initialize Vsync State!");

            Ok(Self {
                window_handle: hwnd,
                device_context,
                gl_context,
                vsync_state,
            })
        }
    }

    pub fn print_pixel_format(&self) {
        unsafe {
            // Get current pixel format index
            let current_pixel_format = GetPixelFormat(self.device_context);

            // Get pixel format description
            let mut pfd = PIXELFORMATDESCRIPTOR::default();
            let bytes = std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u32;

            let result = DescribePixelFormat(
                self.device_context,
                current_pixel_format,
                bytes,
                Some(&mut pfd)
            );

            if result != 0 {
                let double_buffered = (pfd.dwFlags & PFD_DOUBLEBUFFER) == PFD_DOUBLEBUFFER;
                let supports_opengl = (pfd.dwFlags & PFD_SUPPORT_OPENGL) == PFD_SUPPORT_OPENGL;
                let draw_to_window = (pfd.dwFlags & PFD_DRAW_TO_WINDOW) == PFD_DRAW_TO_WINDOW;

                println!("Current pixel format description:");
                println!("  Color bits: {}", pfd.cColorBits);
                println!("  Alpha bits: {}", pfd.cAlphaBits);
                println!("  Depth bits: {}", pfd.cDepthBits);
                println!("  Stencil bits: {}", pfd.cStencilBits);
                println!("  Double buffered: {}", double_buffered);
                println!("  Supports OpenGL: {}", supports_opengl);
                println!("  Draw to window: {}", draw_to_window);
            } else {
                println!("Failed to describe pixel format.");
            }
        }
    }

    pub(crate) fn make_current(&self) -> Result<(), OverlayError> {
        unsafe {
            wglMakeCurrent(self.device_context, self.gl_context)
                .map_err(|e| {
                    println!("Failed to make GL context current: {:?}", e);
                    println!("{:?}", GetLastError());
                    OverlayError::FailedToMakeOpenGLContextCurrent
                })
        }
    }

    pub(crate) fn swap_buffers(&self) -> Result<(), OverlayError> {
        unsafe {
            wglSwapLayerBuffers(self.device_context, WGL_SWAP_MAIN_PLANE)
                .map_err(|e| {
                    println!("Failed to swap buffers: {:?}", e);
                    OverlayError::FailedToSwapBuffers
                })
        }
    }

    // Expose vsync control methods
    pub fn set_vsync(&self, enabled: bool) -> Result<(), OverlayError> {
        self.vsync_state.set_enabled(enabled)
    }

    pub fn get_vsync_state(&self) -> Option<bool> {
        self.vsync_state.get_current_state()
    }

    pub fn is_vsync_supported(&self) -> bool {
        self.vsync_state.is_supported()
    }
}

impl Drop for GlContext {
    fn drop(&mut self) {
        unsafe {
            // Release current context first
            wglMakeCurrent(HDC::default(), HGLRC::default()).ok();

            // Delete the GL context
            wglDeleteContext(self.gl_context).ok();

            // Release the DC we got from GetDC
            if !self.device_context.is_invalid() {
                ReleaseDC(Some(self.window_handle), self.device_context);
            }
        }
    }
}