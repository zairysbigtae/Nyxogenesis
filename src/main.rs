pub mod objects;
use objects::{asteroid::Asteroid, object::*};
use raylib::prelude::*;
use rand::random_range;

fn main() {
    let (mut rl, thread) = raylib::init().size(600, 560).title("mwa").build();

    let mut asteroids: Vec<Asteroid> = Vec::new();
    for _ in 0..2 {
        asteroids.push(Asteroid {
            position: Vector2::new(20.0, 20.0),
            velocity: Vector2::new(random_range(0.0..1.0), random_range(0.0..1.0)),
            width: 20.0,
            height: 20.0,
            mass: 10.0,
            density: 1.0,
        });
    }

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        for asteroid in &mut asteroids {
            asteroid.draw(&mut d);
            asteroid.update();
        }

        d.clear_background(Color::BLACK);
    }
}
