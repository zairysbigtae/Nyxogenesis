use std::sync::RwLockWriteGuard;
use raylib::prelude::*;
use rand::random_range;
use crate::objects::computations::{
    compute_escape_velocity::compute_escape_velocity_mod,
    compute_tidal_forces::compute_tidal_forces_mod,
    compute_orbital_velocity::compute_orbital_velocity_mod,
    compute_gravitational_force::compute_gravitational_force_mod,
};
use crate::objects::Object;

const G: f32 = 1.0;

#[derive(Debug, Clone, Copy)]
pub struct Planet {
    pub position: Vector2,
    pub velocity: Vector2,
    pub width: f32,
    pub height: f32,
    pub mass: f32,
    pub density: f32,
    pub color: Color,
}

impl Planet {
    pub fn new(&self) -> Self {
        Planet {
            position: Vector2 { x: 0.0, y: 0.0 },
            velocity: Vector2 { x: 0.0, y: 0.0 },
            // width: 10.0,
            // height: 10.0,
            width: self.mass,
            height: self.mass,
            mass: self.density + (self.width * self.height),
            density: random_range(0.5..5.0),
            color: Color::GRAY,
        }
    }

    pub fn compute_gravitational_force(&self, other: &dyn Object) -> Vector2 {
        compute_gravitational_force_mod(self, other, G)
    }

    pub fn compute_escape_velocity(&self, other: &dyn Object, distance: f32) -> f32 {
        compute_escape_velocity_mod(self, other, distance, G)
    }

    pub fn compute_orbital_velocity(&self, other: &dyn Object, distance: f32) -> Vector2 {
        compute_orbital_velocity_mod(self, other, distance, G)
    }

    pub fn compute_tidal_forces(&self, other: &dyn Object) -> Vector2 {
        compute_tidal_forces_mod(self, other, G)
    }
}

impl Object for Planet {
    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_v(
            self.position,
            Vector2 {
                x: self.width,
                y: self.height,
            },
            self.color,
        );
    }

    fn update(&mut self, other: Option<&dyn Object>) {
        self.position += self.velocity;
        if let Some(other) = other {
            self.gravity_system(other);
        } else {
            println!("Other is not some, {}", other.is_some());
        }
    }

    fn apply_force(&mut self, force: Vector2) {
        let acceleration = force / self.mass;
        self.velocity += acceleration;
    }

    fn is_colliding(&self, other: &dyn Object) -> bool {
        self.get_position().distance_to(other.get_position())
            <= (self.get_width() / 2.0 + other.get_width() / 2.0)
    }

    fn is_colliding_box(&mut self, other: &Box<dyn Object>) -> bool {
        self.is_colliding(other.as_ref())
    }

    fn collision_update(
        &self,
        objects: &mut RwLockWriteGuard<Vec<Box<dyn Object>>>,
        cooldown: &mut i32,
    ) {
        // Implement post-collision updates here. (E.g., merge objects or change trajectories) if needed.
        let len = objects.len();

        for i in 0..len {
            let obj_i = objects[i].as_mut() as *mut dyn Object;
            for j in (i + 1)..len {
                let obj_j = objects[j].as_mut() as *mut dyn Object;

                // TODO: Make this unsafe somehow..?
                unsafe {
                    if (*obj_i).is_colliding(&*obj_j) && *cooldown <= 0 {
                        if (*obj_i).get_mass() > (*obj_j).get_mass() {
                            // FIXME: You heard the comment, fix me
                            (*obj_i).set_mass((*obj_i).get_mass() + (*obj_j).get_mass());
                            // Resets the cooldown
                            *cooldown = 600;
                        } else if (*obj_j).get_mass() > (*obj_i).get_mass() {
                            (*obj_j).set_mass((*obj_j).get_mass() + (*obj_i).get_mass());
                            // Resets the cooldown
                            *cooldown = 600;
                        }
                    } else if *cooldown > 0 {
                        *cooldown -= 1;
                    }
                }
            }
        }
    }

    fn gravity_system(&mut self, other: &dyn Object) {
        let distance = self.get_position().distance_to(other.get_position());

        // Apply gravitational force
        let force = self.compute_gravitational_force(other);
        self.apply_force(force);

        // Orbital velocity for rotation (can be expanded depending on the realism).
        if distance > 0.0 {
            let orbital_velocity = self.compute_orbital_velocity(other, distance);
            self.apply_force(orbital_velocity);
        }

        // Escape velocity check: Determine if the object has enough velocity to escape the gravitational pull
        let escape_velocity = self.compute_escape_velocity(other, distance);
        // Use this if statement if you want to do something. For now, seems useless lmfao
        // if self.get_velocity().length() >= escape_velocity {
        //     println!("Object has escaped the gravitational pull!");
        // }

        // Tidal forces (stretching effect) - can be expanded for more realistic effects like planetary deformation
        let tidal_effect = self.compute_tidal_forces(other);
        self.apply_force(tidal_effect);

        // Gravitational slingshot can be added here.
        // Example: If the object is in a near-parabolic orbit, apply additional velocity changes
    }

    fn get_position(&self) -> Vector2 {
        self.position
    }
    fn get_velocity(&self) -> Vector2 {
        self.velocity
    }
    fn get_width(&self) -> f32 {
        self.mass / 10.0
    } // Scale for size here.
    fn get_height(&self) -> f32 {
        self.get_width()
    } // Assuming spherical objects
    fn get_mass(&self) -> f32 {
        self.mass
    }
    fn get_color(&self) -> Color {
        self.color
    }
    fn set_mass(&mut self, new_mass: f32) {
        self.mass = new_mass
    }
    fn set_color(&mut self, new_color: Color) {
        self.color = new_color
    }
    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }
}
