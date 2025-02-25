use raylib::{math::Vector2, prelude::RaylibDrawHandle};

pub trait Object: Send + Sync {
    fn new(&self) -> Box<dyn Object>;
    fn draw(&self, d: &mut RaylibDrawHandle);
    fn update(&mut self);

    fn apply_force(&mut self, force: &Vector2) -> Vector2;
    fn is_colliding(&self, other: &dyn Object) -> bool;

    // Get some properties from those space objects
    fn get_position(&self) -> Vector2;
    fn get_velocity(&self) -> Vector2;
    fn get_radius(&self) -> f32;
    fn get_mass(&self) -> f32;
}

impl Clone for Box<dyn Object> {
    fn clone(&self) -> Box<dyn Object> {
        self.new()
    }
}
