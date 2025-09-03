//! Screenshot capture module for the Bapao communication system.
//! 
//! This module provides screenshot capture functionality that can be accessed
//! remotely through the Bapao communication protocol.

extern crate image_base64;

use std::fs;
use std::process;
use bapao_app_protocal::TransUnitType;

/// Captures a screenshot and returns it as binary data.
/// 
/// This function handles screenshot capture requests from external clients.
/// Currently configured to read a static image file, but includes commented
/// code for dynamic screenshot capture using system commands.
/// 
/// # Returns
/// 
/// `TransUnitType::File(Vec<u8>)` - Binary image data of the screenshot
/// 
/// # Examples
/// 
/// ```rust
/// use bapao_app_protocal::TransUnitType;
/// use shot_pic::shot_pic;
/// 
/// let screenshot = shot_pic();
/// match screenshot {
///     TransUnitType::File(data) => {
///         println!("Screenshot captured: {} bytes", data.len());
///     },
///     TransUnitType::String(error) => {
///         println!("Screenshot failed: {}", error);
///     }
/// }
/// ```
/// 
/// # Implementation Notes
/// 
/// The current implementation reads from a static file path. For production use,
/// uncomment and modify the dynamic capture code to use appropriate screenshot
/// tools for your platform:
/// 
/// - Linux: `scrot`, `gnome-screenshot`, `import` (ImageMagick)
/// - macOS: `screencapture`
/// - Windows: PowerShell with System.Drawing
pub fn shot_pic() -> TransUnitType {
    // if let Ok(mut child) = process::Command::new("fswebcam")
    //     .args(["-r", "1440*720", "/home/pi/image.jpg"])
    //     .spawn()
    // {
    //     child.wait().unwrap();

    //     TransUnitType::File(fs::read("/home/pi/image.jpg").unwrap())
    // } else {
    //     TransUnitType::String(String::from("_"))
    // }

    TransUnitType::File(fs::read("/Users/xxx/Downloads/image.jpg").unwrap())
}
