use raylib::prelude::*;
use crate::objects::Object;

// Calculates the gravitational force between this planet and another object.
pub fn compute_gravitational_force_mod(obj1: &dyn Object, obj2: &dyn Object, g: f32) -> Vector2 {
    let distance = obj1.get_position().distance_to(obj2.get_position());
    if distance == 0.0 {
        return Vector2::zero();
    }

    let force_magnitude = (g * obj1.get_mass() * obj2.get_mass()) / (distance * distance);
    let direction = (obj2.get_position() - obj1.get_position()).normalized();
    direction * force_magnitude
}
