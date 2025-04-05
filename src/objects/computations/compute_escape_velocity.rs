use crate::objects::Object;

// Computes the total escape velocity needed to escape gravitational attraction
pub fn compute_escape_velocity_mod(obj1: &dyn Object, obj2: &dyn Object, distance: f32, g: f32) -> f32 {
    (2.0 * g * obj2.get_mass() / distance).sqrt()
}
