use raylib::prelude::*;
use Nyxogenesis::objects::asteroid::Asteroid;
use Nyxogenesis::objects::object::Object;
use rand::random_range;

#[test]
fn expected_removal() {
    let mut asteroids: Vec<Box<dyn Object>> = Vec::new();
    for _ in 0..20 {
        let density = random_range(0.1..10.0);
        let cube_size = random_range(5.0..20.0);

        asteroids.push(Box::new(Asteroid {
            position: Vector2::new(random_range(1.0..699.0), random_range(1.0..560.0)),
            velocity: Vector2::new(random_range(-1.0..1.0), random_range(-1.0..1.0)),
            width: cube_size,
            height: cube_size,
            mass: density * (cube_size * cube_size /*This calculates the area*/),
            density: density,
        }));
    }
}
