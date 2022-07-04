extern crate image_base64;

use std::fs;

use std::process;

use bapao_app_protocal::TransUnitType;

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
