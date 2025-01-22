use windows::{
    core::PCSTR,
    Win32::{
        Foundation::HWND,
        UI::WindowsAndMessaging::FindWindowA,
    },

};
use crate::core::OverlayError;
use rand::Rng;
use skia_safe::Color4f;
use str_crypter::{sc, decrypt_string};

/// Finds either the NVIDIA GeForce Overlay or the AMD DVR OVERLAY and returns a handle to it
pub fn find_target_window() -> Result<HWND, OverlayError> {
    // Encrypted strings for obscurity
    let nvidia_class_name: String = sc!("CEF-OSC-WIDGET\0", 120)
        .expect("Failed to decrypt string at compile time! (this should be impossible...)");
    let nvidia_window_name: String = sc!("NVIDIA GeForce Overlay\0", 121)
        .expect("Failed to decrypt string at compile time! (this should be impossible...)");
    let amd_class_name: String = sc!("AMDDVROVERLAYWINDOWCLASS\0", 120)
        .expect("Failed to decrypt string at compile time! (this should be impossible...)");
    let amd_window_name: String = sc!("amd dvr overlay\0", 121)
        .expect("Failed to decrypt string at compile time! (this should be impossible...)");

    unsafe {
        // Try to find first window
        let first_window = FindWindowA(
            PCSTR::from_raw(nvidia_class_name.as_ptr()),
            PCSTR::from_raw(nvidia_window_name.as_ptr()),
        );

        // Try to find second window
        let second_window = FindWindowA(
            PCSTR::from_raw(amd_class_name.as_ptr()),
            PCSTR::from_raw(amd_window_name.as_ptr()),
        );

        // Return first available window or error
        match (first_window, second_window) {
            (Ok(window), _) => Ok(window),
            (_, Ok(window)) => Ok(window),
            _ => Err(OverlayError::WindowNotFound),
        }
    }
}

/// Takes an RGBA color tuple and converts it to a Skia Color4f
pub fn to_color_4f(color: (u8, u8, u8, u8)) -> Color4f {
    Color4f::new(
        color.0 as f32 / 255.0, // R
        color.1 as f32 / 255.0, // G
        color.2 as f32 / 255.0, // B
        color.3 as f32 / 255.0  // A
    )
}

/// Generates a random number between two ranges
pub fn generate_random_number(min: i64, max: i64) -> i64 {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}