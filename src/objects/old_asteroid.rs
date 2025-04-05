use crate::objects::object::{Object, ObjectFactory};
// use crate::objects::ObjectAccessor;
use rand::random_range;
use raylib::prelude::*;
use std::sync::{Arc, RwLockWriteGuard};

const G: f32 = 0.01;

#[derive(Clone, Copy, Debug)]
pub struct Asteroid {
    pub position: Vector2,
    pub velocity: Vector2,
    pub width: f32,
    pub height: f32,
    pub mass: f32,
    pub density: f32,
    pub color: Color,
}

impl Asteroid {
    pub fn new(&self) -> Self {
        Asteroid {
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

    // Calculates the gravitational force between this planet and another object.
    fn compute_gravitational_force(&self, other: &dyn Object) -> Vector2 {
        let distance = self.get_position().distance_to(other.get_position());
        if distance == 0.0 {
            return Vector2::zero();
        }

        let force_magnitude = (G * self.get_mass() * other.get_mass()) / (distance * distance);
        let direction = (other.get_position() - self.get_position()).normalized();
        (direction * force_magnitude) / rand::random_range(0.0..2.0)
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
        let tidal_effect =
            (self.get_position() - other.get_position()).normalized() * force_magnitude * 0.1; // Simplified. (Here is where you can add more realism)
        tidal_effect
    }
}

impl Object for Asteroid {
    fn draw(&self, d: &mut raylib::prelude::RaylibDrawHandle) {
        d.draw_rectangle_v(
            self.position,
            Vector2::new(self.width, self.height),
            self.color,
        );
    }

    fn update(&mut self, other: Option<&dyn Object>) {
        // let velocity = self.velocity
        //     / Vector2 {
        //         x: 0xF as f32,
        //         y: 0xF as f32,
        //     };
        self.position += self.velocity;
        if let Some(other) = other {
            self.gravity_system(other);
        }
    }

    fn collision_update(
        &self,
        objects: &mut RwLockWriteGuard<Vec<Box<dyn Object>>>,
        cooldown: &mut i32,
    ) {
        // Gets the number of objects so we dont have to run it again, and cause another borrow
        // checker issues...
        // let len = objects.len();
        //
        // for i in 0..len {
        //     let obj_i = objects[i].as_mut() as *mut dyn Object;
        //     for j in (i + 1)..len {
        //         let obj_j = objects[j].as_mut() as *mut dyn Object;

        // TODO: Make this safe somehow..?
        // unsafe {
        //     if (*obj_i).is_colliding(&*obj_j) && *cooldown <= 0 {
        //         if (*obj_i).get_mass() > (*obj_j).get_mass() {
        // FIXME: You heard the comment, fix me
        // (*obj_i).set_mass((*obj_i).get_mass() + (*obj_j).get_mass());
        // Resets the cooldown
        // *cooldown = 600;
        //                 } else if (*obj_j).get_mass() > (*obj_i).get_mass() {
        //                     (*obj_j).set_mass((*obj_j).get_mass() + (*obj_i).get_mass());
        //                     // Resets the cooldown
        //                     *cooldown = 600;
        //                 }
        //             } else if *cooldown > 0 {
        //                 *cooldown -= 1;
        //             }
        //         }
        //     }
        // }
    }

    fn set_mass(&mut self, new_mass: f32) {
        self.mass = new_mass;
    }

    fn apply_force(&mut self, force: Vector2) {
        let acceleration = force / self.mass;
        self.velocity += acceleration;
    }

    fn is_colliding(&self, other: &dyn Object) -> bool {
        let other_pos = other.get_position();

        let x_overlap = (self.position.x < other_pos.x + other.get_width())
            && (self.position.x + self.width > other_pos.x);

        let y_overlap = (self.position.y < other_pos.y + other.get_height())
            && (self.position.y + self.height > other_pos.y);

        x_overlap && y_overlap
    }

    fn is_colliding_box(&mut self, other: &Box<dyn Object>) -> bool {
        let other_pos = other.get_position();

        let x_overlap = (self.position.x < other_pos.x + other.get_width())
            && (self.position.x + self.width > other_pos.x);

        let y_overlap = (self.position.y < other_pos.y + other.get_height())
            && (self.position.y + self.height > other_pos.y);

        x_overlap && y_overlap
    }

    // fn engulf(&mut self, other_vector: &mut Vec<Box<dyn Object>>, index: usize) {
    //     let self_mass = self.get_mass();
    //     other_vector.retain(|other| self_mass <= other.get_mass());
    // }

    fn get_position(&self) -> Vector2 {
        self.position
    }

    fn get_velocity(&self) -> Vector2 {
        self.velocity
    }

    fn get_width(&self) -> f32 {
        self.width
    }

    fn get_height(&self) -> f32 {
        self.height
    }

    fn get_mass(&self) -> f32 {
        self.mass
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

    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn set_color(&mut self, new_color: Color) {
        self.color = new_color;
    }
}
