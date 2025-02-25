use rand::Rng;
use raylib::prelude::*;
use super::object::Object;

pub struct Asteroid {
    pub position: Vector2,
    pub velocity: Vector2,
    pub radius: f32,
    pub mass: f32
}

impl Object for Asteroid {
    fn new(&self) -> Box<dyn Object> {
        let mut rng = rand::rng();

        Box::new(Asteroid {
            position: self.position,
            velocity: self.velocity,
            radius: self.radius,
            mass: self.radius + rng.random_range(-10..10) as f32,
        })
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle_v(
            self.position,
            self.radius,
            Color::GRAY,
        );
    }

    fn update(&mut self) {
        let acceleration = self.apply_force(&Vector2::new(1.0, 1.0));
        self.velocity += acceleration;
        self.position += self.velocity;
    }

    fn apply_force(&mut self, force: &Vector2) -> Vector2 {
        self.velocity += *force;
        self.velocity
    }

    fn is_colliding(&self, other: &dyn Object) -> bool {
        let dx = self.get_position().x - other.get_position().x;
        let dy = self.get_position().y - other.get_position().y;
        let squared_distance = dx * dx + dy * dy;
        let radius_sum = self.get_radius() + other.get_radius();

        squared_distance <= radius_sum * radius_sum
    }

    fn get_position(&self) -> Vector2 {
        self.position
    }

    fn get_velocity(&self) -> Vector2 {
        self.velocity
    }

    fn get_radius(&self) -> f32 {
        self.radius
    }

    fn get_mass(&self) -> f32 {
        self.mass
    }
}
