pub mod objects;
use objects::{asteroid::Asteroid, object::*};
use rand::random_range;
use raylib::prelude::*;
use std::sync::{Arc, RwLock};
use std::fmt::Write;

fn main() {
    let (mut rl, thread) = raylib::init().size(600, 560).title("mwa").build();

    let mut asteroids: Arc<RwLock<Vec<Box<dyn Object>>>> = Arc::new(RwLock::new(Vec::new()));
    
    {
        let mut asteroids = asteroids.write().unwrap();
        for _ in 0..5 {
            let density = random_range(0.1..10.0);
            let cube_size = random_range(5.0..20.0);

            asteroids.push(Box::new(Asteroid {
                // position: Vector2::new(random_range(1.0..699.0), random_range(1.0..560.0)),
                position: Vector2::new(300.0, 300.0),
                velocity: Vector2::new(random_range(-1.0..1.0), random_range(-1.0..1.0)),
                width: cube_size,
                height: cube_size,
                mass: density * (cube_size * cube_size/*This calculates the area*/),
                density: density,
                color: Color::GRAY,
            }));
        }
    }
    let mut collision_cooldown = 600;

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        // println!("Frame Start!");
        
        let fps = rl.get_fps();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // if frame_count % 60 == 0 {
        let mut fps_text = String::new();
        write!(&mut fps_text, "FPS: {}", fps).unwrap();

        d.draw_text(&fps_text, 10, 10, 16, Color::WHITE);
        // }

        {
            let asteroids = &mut asteroids.write().unwrap();
            for a in asteroids.iter_mut() {
                a.draw(&mut d);
                a.update();
            }

        }

        {
            // TODO: Find a way on how to NOT use Clone and Copy features
            let cloned_asteroids = {
                let asteroids = asteroids.read().unwrap();
                asteroids.clone()
            };

            let mut asteroids_lock = asteroids.write().unwrap();

            for a in &cloned_asteroids {
                a.collision_update(&mut asteroids_lock, &mut collision_cooldown);
            }
        }
    }
}
