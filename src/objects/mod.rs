use asteroid::Asteroid;
use object::Object;

pub mod asteroid;
pub mod planet;
pub mod object;

pub trait ObjectAccessor {
    fn get_object<'a>(&'a self, i: usize) -> &'a Box<dyn Object>;
    fn get_object_mut<'a>(&'a mut self, i: usize) -> &'a Box<dyn Object>;
}

impl ObjectAccessor for (&mut [Box<dyn Object>], &mut [Box<dyn Object>]) {
    fn get_object<'a>(&'a self, i: usize) -> &'a Box<dyn Object> {
        let (left, right) = self;

        match i < left.len() {
            true => &left[i],
            false => &right[i - left.len()],
        }
    }

    fn get_object_mut<'a>(&'a mut self, i: usize) -> &'a Box<dyn Object> {
        let (left, right) = self;

        match i < left.len() {
            true => &mut left[i],
            false => &mut right[i - left.len()],
        }
    }
}
