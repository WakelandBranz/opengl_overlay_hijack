mod gl;
mod helper;
mod skia;
mod draw;

use skia_safe::{Color, Font, FontMgr, FontStyle};
use skia_safe::wrapper::PointerWrapper;
use windows::{
    Win32::{
        Foundation::{
            HWND
        },
        Graphics::{
            Dwm::DwmExtendFrameIntoClientArea,
        },
        UI::{
            WindowsAndMessaging::{
                GetWindowLongA, SetLayeredWindowAttributes, SetWindowLongPtrA, SetWindowPos,
                GWL_EXSTYLE, HWND_TOPMOST, LWA_ALPHA, SWP_NOMOVE, SWP_NOSIZE
            },
            Controls::MARGINS
        }

    }
};
use windows::Win32::Foundation::COLORREF;
use crate::core::{
    gl::GlContext,
    helper::{find_target_window, generate_random_number}
};
use crate::core::skia::SkiaContext;

const LAYERED_WINDOW_STYLE: i32 = 0x20;
const WINDOW_ALPHA: u8 = 0xFF;

// SAFETY: HWND is thread-safe as it's just an identifier
unsafe impl Send for Overlay {}
unsafe impl Sync for Overlay {}

pub struct Overlay {
    // Necessity
    pub window_handle: HWND,

    // Core rendering
    gl_context: Option<GlContext>,
    skia_context: Option<SkiaContext>,

    // Cache
    font: Font,
}

impl Overlay {
    pub fn new(font: impl AsRef<str>, size: f32) -> Self {
        // Create a typeface using FontMgr
        let font_mgr = FontMgr::default();
        let typeface = font_mgr
            .legacy_make_typeface(Some(font.as_ref()), FontStyle::normal())
            .expect("Failed to create Typeface");

        let font = Font::new(typeface, size); // Set font size to 48

        Self {
            // Necessity
            window_handle: HWND::default(),

            // Core rendering
            gl_context: None,
            skia_context: None,

            // Cache
            font,
        }
    }

    // CORE FUNCTIONALITY ----------------
    /// Must be called prior to any rendering.
    pub fn init(&mut self) -> Result<(), OverlayError> {
        // Find and validate window
        self.window_handle = find_target_window()?;

        // Set window style
        let window_info = unsafe { GetWindowLongA(self.window_handle, GWL_EXSTYLE) };
        if window_info == 0 {
            return Err(OverlayError::FailedToGetWindowLong);
        }

        let modified_style = window_info | LAYERED_WINDOW_STYLE;
        let modify_window = unsafe {
            SetWindowLongPtrA(self.window_handle, GWL_EXSTYLE, modified_style as isize)
        };
        if modify_window == 0 {
            return Err(OverlayError::FailedToSetWindowLong);
        }

        // TODO: Add randomization to window margins!
        // Configure window margins
        let margins = MARGINS {
            cxLeftWidth: -1,
            cxRightWidth: -1,
            cyTopHeight: -1,
            cyBottomHeight: -1,
        };

        // Set window properties so that rendering can be transparent
        unsafe {
            SetLayeredWindowAttributes(
                self.window_handle,
                COLORREF(0x000000),
                WINDOW_ALPHA,
                LWA_ALPHA
            ).map_err(|_| OverlayError::FailedSetLayeredWindowAttributes)?;

            DwmExtendFrameIntoClientArea(self.window_handle, &margins)
                .map_err(|_| OverlayError::FailedToExtendFrame)?;

            SetWindowPos(
                self.window_handle,
                Some(HWND_TOPMOST),
                0, 0, 0, 0,
                SWP_NOMOVE | SWP_NOSIZE
            ).map_err(|_| OverlayError::FailedToSetWindowPos)?;
        }

        Ok(())
    }

    pub fn startup_renderer(&mut self, vsync: bool) -> Result<(), OverlayError> {
        let gl_context = GlContext::new(self.window_handle, vsync)
            .map_err(|_| OverlayError::GlContextSetupFailed)?;

        gl_context.print_pixel_format();

        // Make GL context current before creating Skia context
        gl_context.make_current()?;

        let skia_context = SkiaContext::new(1920, 1080)
            .map_err(|_| OverlayError::SkiaContextSetupFailed)?;

        self.gl_context = Some(gl_context);
        self.skia_context = Some(skia_context);

        Ok(())
    }

    pub fn begin_scene(&mut self) -> Result<(), OverlayError> {
        let canvas = self.skia_context.as_mut()
            .expect("Skia context should be initialized")
            .canvas();
        canvas.clear(Color::TRANSPARENT);
        Ok(())
    }

    pub fn present_scene(&mut self) -> Result<(), OverlayError> {
        let sk_context = self.skia_context.as_mut()
            .ok_or(OverlayError::NoRenderTarget)?;

        let gl_context = self.gl_context.as_mut()
            .ok_or(OverlayError::FailedToCreateOpenGLContext)?;

        sk_context.gr_context.flush_and_submit();
        gl_context.swap_buffers()?;

        Ok(())
    }
}

impl Drop for Overlay {
    fn drop(&mut self) {
        // Try to clear the screen one last time before dropping
        if let (Some(gl_context), Some(skia_context)) = (self.gl_context.as_mut(), self.skia_context.as_mut()) {
            // Make context current
            if gl_context.make_current().is_ok() {
                // Clear the canvas
                let canvas = skia_context.canvas();
                canvas.clear(Color::TRANSPARENT);

                // Flush Skia operations
                skia_context.gr_context.flush_and_submit();

                // Swap buffers to show the clear
                gl_context.swap_buffers().ok();
            }
        }

        // Drop contexts explicitly in reverse order of creation
        self.skia_context = None;  // Drop Skia first
        self.gl_context = None;    // Then drop GL context
    }
}

#[derive(Debug)]
pub enum OverlayError {
    WindowNotFound,
    FailedToGetWindowLong,
    FailedToSetWindowLong,
    FailedToExtendFrame,
    FailedSetLayeredWindowAttributes,
    FailedToSetWindowPos,
    ShowWindowFailed,

    GlContextSetupFailed,
    SkiaContextSetupFailed,

    NoRenderTarget,
    GetWindowRectFailed,
    GetWriteTextFormatFailed,
    DrawFailed,
    DrawTextFailed(i32),
    FailedToGetFontWidth,
    CreateBrushFailed(i32),
    CreateSolidColorBrushFailed,
    ID2D1BrushCastFailed,
    CreateGradientStopCollectionFailed,
    CreateLinearGradientBrushFailed,
    CreateRadialGradientBrushFailed,
    NoD2DFactory,
    CreateStrokeStyleFailed,
    FailedToShowWindow,
    FailedToGetDeviceContext,
    FailedToSetPixelFormat,
    FailedToCreateOpenGLContext,
    FailedToMakeOpenGLContextCurrent,
    FailedToCreateSkiaInterface,
    FailedToCreateDirectContext,
    FailedToCreateSkiaSurface,
    FailedToSwapBuffers,
    FailedToRetrieveOpenGLBinary,
}