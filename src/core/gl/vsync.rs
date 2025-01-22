use std::ffi::CStr;
use windows::core::PCSTR;
use windows::Win32::Graphics::OpenGL::wglGetProcAddress;
use crate::core::OverlayError;

type WglGetExtensionsStringEXT = unsafe extern "system" fn() -> *const i8;
type WglSwapIntervalEXT = unsafe extern "system" fn(interval: i32) -> bool;
type WglGetSwapIntervalEXT = unsafe extern "system" fn() -> i32;

#[derive(Clone)]
pub struct VsyncState {
    swap_interval: Option<WglSwapIntervalEXT>,
    get_swap_interval: Option<WglGetSwapIntervalEXT>,
}

impl VsyncState {
    pub fn new() -> Self {
        Self {
            swap_interval: None,
            get_swap_interval: None,
        }
    }

    pub fn init(&mut self) -> Result<(), OverlayError> {
        if !self.wgl_extension_supported() { return Err(OverlayError::VsyncControlNotSupported); }

        unsafe {
            // Get necessary function pointers
            let swap_interval_ptr = wglGetProcAddress(PCSTR(b"wglSwapIntervalEXT\0".as_ptr()));
            let get_swap_interval_ptr = wglGetProcAddress(PCSTR(b"wglGetSwapIntervalEXT\0".as_ptr()));

            if swap_interval_ptr.is_none() || get_swap_interval_ptr.is_none() {
                return Err(OverlayError::FailedToGetVsyncFunctionPointers)
            }

            self.swap_interval = Some(std::mem::transmute(swap_interval_ptr));
            self.get_swap_interval = Some(std::mem::transmute(get_swap_interval_ptr));
        }

        Ok(())
    }

    fn wgl_extension_supported(&self) -> bool {
        unsafe {
            // Get the function pointer for wglGetExtensionsStringEXT
            let wgl_get_extensions_ptr = wglGetProcAddress(PCSTR(b"wglGetExtensionsStringEXT\0".as_ptr()));
            if wgl_get_extensions_ptr.is_none() {
                return false;
            }

            let get_extensions: WglGetExtensionsStringEXT = std::mem::transmute(wgl_get_extensions_ptr);

            // Get extensions string
            let extensions_ptr = get_extensions();
            if extensions_ptr.is_null() {
                return false;
            }

            // Check for extension
            let extensions = CStr::from_ptr(extensions_ptr)
                .to_str()
                .unwrap_or("that didn't work");

            println!("Extensions: {:?}", extensions);

            extensions.contains("WGL_EXT_swap_control")
        }
    }

    pub fn set_enabled(&self, is_enabled: bool) -> Result<(), OverlayError> {
        let swap_interval = self.swap_interval.expect("Failed to retrieve swap interval when setting vsync state!");

        unsafe {
            if !swap_interval(if is_enabled { 1 } else { 0 }) {
                return Err(OverlayError::FailedToSetVsyncState);
            }

            // Verify the change if we can
            if let Some(current_state) = self.get_current_state() {
                if current_state != is_enabled {
                    return Err(OverlayError::FailedToVerifyVsyncState);
                }
            }
        }

        Ok(())
    }

    pub fn get_current_state(&self) -> Option<bool> {
        self.get_swap_interval.map(|func| unsafe {
            func() == 1
        })
    }

    pub fn is_supported(&self) -> bool {
        self.swap_interval.is_some() && self.get_swap_interval.is_some()
    }
}