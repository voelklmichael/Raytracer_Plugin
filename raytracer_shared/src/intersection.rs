use crate::{Vec2,Ray};
pub struct Intersection {
    pub distance: f32,
    pub normal: Vec2,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}
