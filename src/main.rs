pub mod objects;
use objects::planet::Planet;
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
        for _ in 0..100 {
            let density = random_range(0.1..10.0);
            let cube_size = 5.0;

            asteroids.push(Box::new(Asteroid {
                position: Vector2::new(random_range(1.0..699.0), random_range(1.0..560.0)),
                // position: Vector2::new(300.0, 300.0),
                velocity: Vector2::new(random_range(-1.0..1.0), random_range(-1.0..1.0)),
                width: cube_size,
                height: cube_size,
                mass: density * (cube_size * cube_size/*This calculates the area*/),
                density: density,
                color: Color::GRAY,
            }));
        }
    }


    let mut planets: Arc<RwLock<Vec<Box<dyn Object>>>> = Arc::new(RwLock::new(Vec::new()));
    
    {
        let mut planets = planets.write().unwrap();
        for _ in 0..1 {
            let density = random_range(0.1..10.0);
            let cube_size = 5.0;

            planets.push(Box::new(Planet {
                position: Vector2::new(random_range(1.0..699.0), random_range(1.0..560.0)),
                // position: Vector2::new(300.0, 300.0),
                velocity: Vector2::new(random_range(-1.0..1.0), random_range(-1.0..1.0)),
                width: cube_size,
                height: cube_size,
                mass: density * (cube_size * cube_size/*This calculates the area*/),
                density: density,
                color: Color::GREEN,
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
            let mut asteroids = asteroids.write().unwrap();
            let mut planets = planets.write().unwrap();

            for i in 0..asteroids.len() {
                let mut nearest_dist = f32::MAX;
                // let mut nearest_planet: Option<&dyn Object> = None;
                let mut nearest_index: Option<usize> = None;

                for j in 0..asteroids.len() {
                    if i != j {
                        let dist = asteroids[i].get_position().distance_to(asteroids[j].get_position());
                        if dist < nearest_dist {
                            nearest_dist = dist;
                            nearest_index = Some(j);
                        }
                    }
                }
                if let Some(j) = nearest_index {
                    // let mut nearest_planet: &Box<dyn Object> = &planets[j].clone();  
                    let mut nearest_asteroid: &Box<dyn Object> = &asteroids[j].clone();  
                    // asteroids[i].update(Some(nearest_planet.as_ref()));
                    asteroids[i].update(Some(nearest_asteroid.as_ref()));
                } else {
                    asteroids[i].update(None);
                }
                asteroids[i].draw(&mut d);
            }
        }
 
        {
            let mut planets = planets.write().unwrap();
            let mut asteroids = asteroids.write().unwrap();

            for i in 0..planets.len() {
                let mut nearest_dist = f32::MAX;
                // let mut nearest_planet: Option<&dyn Object> = None;
                let mut nearest_index: Option<usize> = None;

                for j in 0..planets.len() {
                    if i != j {
                        let dist = planets[i].get_position().distance_to(planets[j].get_position());
                        if dist < nearest_dist {
                            nearest_dist = dist;
                            nearest_index = Some(j);
                        }
                    }
                }
                if let Some(j) = nearest_index {
                    let mut nearest_planet: &Box<dyn Object> = &planets[j].clone();  
                    let mut nearest_asteroid: &Box<dyn Object> = &asteroids[j].clone();  
                    planets[i].update(Some(nearest_planet.as_ref()));
                    planets[i].update(Some(nearest_asteroid.as_ref()));
                } else {
                    planets[i].update(None);
                }
                planets[i].draw(&mut d);
            }
        }

        {
            // NOTE: Cloning Section
            // TODO: Find a way on how to NOT use Clone and Copy features
            let cloned_asteroids = {
                let asteroids = asteroids.read().unwrap();
                asteroids.clone()
            };

            let cloned_planets = {
                let planets = planets.read().unwrap();
                planets.clone()
            };
            
            // NOTE: Locking Section
            let mut asteroids_lock = asteroids.write().unwrap();
            let mut planets_lock = planets.write().unwrap();

            // NOTE: Collision Updating Section
            for a in &cloned_asteroids {
                a.collision_update(&mut asteroids_lock, &mut collision_cooldown);
                a.collision_update(&mut planets_lock, &mut collision_cooldown)
            }
            for p in &cloned_planets {
                p.collision_update(&mut asteroids_lock, &mut collision_cooldown);
                p.collision_update(&mut planets_lock, &mut collision_cooldown);
            }
        }
    }
}
