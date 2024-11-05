use std::{
    error::Error,
    ffi::{c_char, c_int, CString},
};

use rust_raylib::raylib;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    unsafe {
        raylib::InitWindow(600, 400, "Rust raylib".as_ptr() as *const c_char);

        while !raylib::WindowShouldClose() {
            raylib::BeginDrawing();
            raylib::ClearBackground(raylib::Color::BLACK);
            raylib::DrawText(
                CString::new("Hello, raylib!")?.as_ptr(),
                (200.0f64 + (raylib::GetTime() * 10.0f64).cos() * 80.0f64) as c_int,
                (120.0f64 + (raylib::GetTime() * 10.0f64).sin() * 30.0f64) as c_int,
                24,
                raylib::Color::new_rgb(255, 100, 10),
            );
            raylib::EndDrawing();
        }

        raylib::CloseWindow();
    }

    Ok(())
}
