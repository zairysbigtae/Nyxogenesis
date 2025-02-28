pub mod objects;
use objects::{object::*, asteroid::Asteroid};
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(600,560)
        .title("mwa")
        .build();

    // let asteroid = Asteroid::new();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
    }
}
