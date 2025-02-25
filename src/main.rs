pub mod objects;
use raylib::prelude::*;

fn main() {
    let monitor = 0;

    let screen_width = unsafe { raylib::ffi::GetMonitorWidth(monitor) };
    let screen_height = unsafe { raylib::ffi::GetMonitorHeight(monitor) };

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
    }
}
