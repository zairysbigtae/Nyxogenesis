use raylib::prelude::*;

pub trait Object: Send + Sync {
    fn draw(&self, d: &mut RaylibDrawHandle);
    fn update(&mut self);

    fn apply_force(&mut self, force: Vector2);
    fn is_colliding(&mut self, other: &dyn Object) -> bool;

    // Get some properties from those space objects
    fn get_position(&self) -> Vector2;
    fn get_velocity(&self) -> Vector2;
    fn get_width(&self) -> f32;
    fn get_height(&self) -> f32;
    fn get_mass(&self) -> f32;
}

pub trait ObjectFactory: Sized {
    fn new(&self, position: Vector2, velocity: Vector2, width: f32, height: f32) -> Self;
}
