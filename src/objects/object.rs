use raylib::prelude::*;
use std::f32::consts::PI;
use std::sync::{Arc, RwLockWriteGuard};

const G: f32 = 6.67430e-11; // Gravitational constant.

pub trait Object: std::fmt::Debug + Send + Sync {
    fn draw(&self, d: &mut RaylibDrawHandle);
    fn update(&mut self);
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

#[derive(Debug)]
pub struct Planet {
    position: Vector2,
    velocity: Vector2,
    mass: f32,
    color: Color,
}

impl Planet {
    pub fn new(position: Vector2, velocity: Vector2, mass: f32, color: Color) -> Self {
        Planet { position, velocity, mass, color }
    }

    // Calculates the gravitational force between this planet and another object.
    fn compute_gravitational_force(&self, other: &dyn Object) -> Vector2 {
        let distance = self.get_position().distance_to(other.get_position());
        if distance == 0.0 { return Vector2::zero(); }

        let force_magnitude = (G * self.get_mass() * other.get_mass()) / (distance * distance);
        let direction = (other.get_position() - self.get_position()).normalize();
        direction * force_magnitude
    }

    // Computes the orbital velocity based on gravitational attraction (I will assume it is a circular orbit).
    fn compute_orbital_velocity(&self, other: &dyn Object, distance: f32) -> Vector2 {
        let orbital_velocity = (G * other.get_mass() / distance).sqrt();
        Vector2::new(-self.velocity.y, self.velocity.x) * orbital_velocity
    }

    // Computes the total escape velocity needed to escape gravitational attraction
    fn compute_escape_velocity(&self, other: &dyn Object, distance: f32) -> f32 {
        (2.0 * G * other.get_mass() / distance).sqrt()
    }

    // Simplified tidal force model (stretching effect) - Simplified because you could implement further physics for more realism.
    fn compute_tidal_forces(&self, other: &dyn Object) -> Vector2 {
        let distance = self.get_position().distance_to(other.get_position());
        let force_magnitude = (G * self.get_mass() * other.get_mass()) / (distance * distance);
        let tidal_effect = (self.get_position() - other.get_position()).normalize() * force_magnitude * 0.1; // Simplified. (Here is where you can add more realism)
        tidal_effect
    }
}

impl Object for Planet {
    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle_v(self.position, self.get_width() / 2.0, self.get_color());
    }

    fn update(&mut self) {
        self.position += self.velocity;
    }

    fn apply_force(&mut self, force: Vector2) {
        let acceleration = force / self.mass;
        self.velocity += acceleration;
    }

    fn is_colliding(&self, other: &dyn Object) -> bool {
        self.get_position().distance_to(other.get_position()) <= (self.get_width() / 2.0 + other.get_width() / 2.0)
    }

    fn is_colliding_box(&mut self, other: &Box<dyn Object>) -> bool {
        self.is_colliding(other.as_ref())
    }

    fn collision_update(&self, objects: &mut RwLockWriteGuard<Vec<Box<dyn Object>>>, cooldown: &mut i32) {
        // Implement post-collision updates here. (E.g., merge objects or change trajectories) if needed.
    }

    fn gravity_system(&mut selfgit , other: &dyn Object) {
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
        if self.get_velocity().length() >= escape_velocity {
            println!("Object has escaped the gravitational pull!");
        }

        // Tidal forces (stretching effect) - can be expanded for more realistic effects like planetary deformation
        let tidal_effect = self.compute_tidal_forces(other);
        self.apply_force(tidal_effect);

        // Gravitational slingshot can be added here.
        // Example: If the object is in a near-parabolic orbit, apply additional velocity changes
    }

    fn get_position(&self) -> Vector2 { self.position }
    fn get_velocity(&self) -> Vector2 { self.velocity }
    fn get_width(&self) -> f32 { self.mass / 10.0 } // Scale for size here.
    fn get_height(&self) -> f32 { self.get_width() } // Assuming spherical objects
    fn get_mass(&self) -> f32 { self.mass }
    fn get_color(&self) -> Color { self.color }
    fn set_mass(&mut self, new_mass: f32) { self.mass = new_mass }
    fn set_color(&mut self, new_color: Color) { self.color = new_color }
    fn clone_box(&self) -> Box<dyn Object> { Box::new(self.clone()) }
}
