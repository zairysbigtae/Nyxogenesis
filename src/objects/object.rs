use raylib::prelude::*;
use std::f32::consts::PI;
use std::sync::{Arc, RwLockWriteGuard};
use std::fmt::Debug;

pub trait Object: Debug + Send + Sync {
    fn draw(&self, d: &mut RaylibDrawHandle);
    fn update(&mut self, other: Option<&dyn Object>);
    fn apply_force(&mut self, force: Vector2);
    fn is_colliding(&self, other: &dyn Object) -> bool;
    fn is_colliding_box(&mut self, other: &Box<dyn Object>) -> bool;
    fn collision_update(&self, objects: &mut RwLockWriteGuard<Vec<Box<dyn Object>>>, cooldown: &mut i32);
    fn gravity_system(&mut self, other: &dyn Object);
    fn get_position(&self) -> Vector2;
    fn get_velocity(&self) -> Vector2;
    fn get_width(&self) -> f32;
    fn get_height(&self) -> f32;
    fn get_mass(&self) -> f32;
    fn get_color(&self) -> Color;
    fn set_mass(&mut self, new_mass: f32);
    fn set_color(&mut self, new_color: Color);
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
