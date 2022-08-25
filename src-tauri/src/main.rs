#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{fs, io::BufReader};

mod ascii_image;

#[tauri::command]
fn to_ascii(img_path: &str) -> String {
    let img =
        ascii_image::AsciiImage::new(BufReader::new(fs::File::open(img_path).unwrap())).unwrap();
    let ascii_img = img.to_ascii(300, 300);
    print!("{}", ascii_img);
    return ascii_img;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![to_ascii])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
