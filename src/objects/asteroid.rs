use crate::objects::object::{Object, ObjectFactory};
use rand::random_range;
use raylib::prelude::*;

pub struct Asteroid {
    pub position: Vector2,
    pub velocity: Vector2,
    pub width: f32,
    pub height: f32,
    pub mass: f32,
    pub density: f32,
}

impl ObjectFactory for Asteroid {
    fn new(&self, position: Vector2, velocity: Vector2, width: f32, height: f32) -> Self {
        Self {
            position,
            velocity,
            width: width,
            height: height,
            mass: self.density + (width * height),
            density: random_range(0.5..5.0),
        }
    }
}

impl Object for Asteroid {
    fn draw(&self, d: &mut raylib::prelude::RaylibDrawHandle) {
        d.draw_rectangle_v(
            self.position,
            Vector2::new(self.width, self.height),
            Color::GRAY,
        );
    }

    fn update(&mut self) {
        let velocity = self.velocity
            / Vector2 {
                x: 0xF as f32,
                y: 0xF as f32,
            };
        self.position += velocity;
    }

    fn apply_force(&mut self, force: raylib::prelude::Vector2) {
        todo!()
    }

    fn is_colliding(&mut self, other: &dyn Object) -> bool {
        let other_pos = other.get_position();

        let x_overlap = (self.position.x < other_pos.x + other.get_width())
            && (self.position.x + self.width > other_pos.x);

        let y_overlap = (self.position.y < other_pos.y + other.get_height())
            && (self.position.y + self.height > other_pos.y);

        x_overlap && y_overlap
    }

    fn get_position(&self) -> Vector2 {
        self.position
    }

    fn get_velocity(&self) -> Vector2 {
        self.velocity
    }

    fn get_width(&self) -> f32 {
        self.width
    }

    fn get_height(&self) -> f32 {
        self.height
    }

    fn get_mass(&self) -> f32 {
        self.mass
    }
}
