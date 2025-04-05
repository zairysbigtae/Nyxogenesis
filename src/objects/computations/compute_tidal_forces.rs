use crate::objects::Object;
use raylib::prelude::*;

// Simplified tidal force model (stretching effect) - Simplified because you could implement further physics for more realism.
pub fn compute_tidal_forces_mod(obj1: &dyn Object, obj2: &dyn Object, g: f32) -> Vector2 {
    let distance = obj1.get_position().distance_to(obj2.get_position());
    let force_magnitude = (g * obj1.get_mass() * obj2.get_mass()) / (distance * distance);
    let tidal_effect =
        (obj1.get_position() - obj2.get_position()).normalized() * force_magnitude * 0.1; // Simplified. (Here is where you can add more realism)
    tidal_effect
}
