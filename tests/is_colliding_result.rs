use raylib::prelude::*;
use Nyxogenesis::objects::asteroid::Asteroid;
use Nyxogenesis::objects::object::Object;

#[test]
fn collision_detected() {
    let mut asteroid1 = Asteroid {
        position: Vector2::new(10.0, 10.0),
        velocity: Vector2::new(0.0, 0.0),
        width: 20.0,
        height: 20.0,
        mass: 10.0,
        density: 1.0,
    };

    #[allow(unused)]
    let mut asteroid2 = Asteroid {
        position: Vector2::new(20.0, 20.0), // Slightly overlapping
        velocity: Vector2::new(0.0, 0.0),
        width: 20.0,
        height: 20.0,
        mass: 10.0,
        density: 1.0,
    };

    assert!(asteroid1.is_colliding(&asteroid2));
}

#[test]
fn no_collision() {
    let mut asteroid1 = Asteroid {
        position: Vector2::new(10.0, 10.0),
        velocity: Vector2::new(0.0, 0.0),
        width: 20.0,
        height: 20.0,
        mass: 10.0,
        density: 1.0,
    };

    #[allow(unused)]
    let mut asteroid2 = Asteroid {
        position: Vector2::new(40.0, 40.0), // Not overlapping
        velocity: Vector2::new(0.0, 0.0),
        width: 20.0,
        height: 20.0,
        mass: 10.0,
        density: 1.0,
    };

    assert!(!asteroid1.is_colliding(&asteroid2));
}
