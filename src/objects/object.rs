use raylib::prelude::*;
use std::sync::{Arc, RwLockWriteGuard};

pub trait Object: std::fmt::Debug + Send + Sync {
    fn draw(&self, d: &mut RaylibDrawHandle);
    fn update(&mut self);

    fn apply_force(&mut self, force: Vector2);
    fn is_colliding(&self, other: &dyn Object) -> bool;
    fn is_colliding_box(&mut self, other: &Box<dyn Object>) -> bool;
    fn collision_update(&self, objects: &mut RwLockWriteGuard<Vec<Box<dyn Object>>>);

    fn gravity_system(&mut self, other: &dyn Object);
    // fn engulf(&mut self, other_vector: &mut Vec<Box<dyn Object>>, index: usize);

    // Get some properties from those space objects
    fn get_position(&self) -> Vector2;
    fn get_velocity(&self) -> Vector2;
    fn get_width(&self) -> f32;
    fn get_height(&self) -> f32;
    fn get_mass(&self) -> f32;
    fn clone_box(&self) -> Box<dyn Object>;
}

pub trait ObjectFactory: Sized {
    fn new(&self) -> Self;
}

impl Clone for Box<dyn Object> {
    fn clone(&self) -> Box<dyn Object> {
        self.clone_box()
    }
}
