use raylib::prelude::*;
use crate::objects::Object;

// Computes the orbital velocity based on gravitational attraction (I will assume it is a circular orbit).
pub fn compute_orbital_velocity_mod(obj1: &dyn Object, obj2: &dyn Object, distance: f32, g: f32) -> Vector2 {
    let orbital_velocity = (g * obj2.get_mass() / distance).sqrt();
    Vector2::new(-obj1.get_velocity().y, obj1.get_velocity().x) * orbital_velocity
}
