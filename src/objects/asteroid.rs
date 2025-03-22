use crate::objects::object::{Object, ObjectFactory};
// use crate::objects::ObjectAccessor;
use rand::random_range;
use raylib::prelude::*;
use std::sync::{Arc, RwLockWriteGuard};
use rayon::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Asteroid {
    pub position: Vector2,
    pub velocity: Vector2,
    pub width: f32,
    pub height: f32,
    pub mass: f32,
    pub density: f32,
}

impl ObjectFactory for Asteroid {
    fn new(&self) -> Self {
        Self {
            position: Vector2 { x: 0.0, y: 0.0 },
            velocity: Vector2 { x: 0.0, y: 0.0 },
            width: 10.0,
            height: 10.0,
            mass: self.density + (self.width * self.height),
            density: random_range(0.5..5.0),
        }
    }
}

impl Object for Asteroid {
    fn draw(&self, d: &mut raylib::prelude::RaylibDrawHandle) {
        d.draw_rectangle_v(
            self.position,
            Vector2::new(self.width, self.height),
            Color::GRAY,
        );
    }

    fn update(&mut self) {
        // let velocity = self.velocity
        //     / Vector2 {
        //         x: 0xF as f32,
        //         y: 0xF as f32,
        //     };
        self.position += self.velocity;
    }

    fn collision_update(
        &self,
        objects: &mut RwLockWriteGuard<Vec<Box<dyn Object>>>
    ) {
        objects.par_iter().enumerate().for_each(|(i, obj_i)| {
            for obj_j in &objects[(i+1)..objects.len()] {
                if obj_i.get_position().distance_to(obj_j.get_position()) > 20.0 {
                    continue;
                }

                if objects[i].is_colliding(obj_j.as_ref()) {
                    // println!("Collision detected between {:?} and {:?}", i, j);
                }
            }
        })
    }

    fn apply_force(&mut self, force: raylib::prelude::Vector2) {
        todo!()
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
        todo!()
    }

    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }
}
