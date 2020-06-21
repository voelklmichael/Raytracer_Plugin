use crate::{Ray, Intersectable,Vec2,Intersection};

pub struct Scene {
    objects: Vec<Box<dyn Intersectable>>,
}
impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
        }
    }
    pub fn push<T: Intersectable + 'static>(&mut self, t: T) {
        self.objects.push(Box::new(t));
    }
    pub fn trace(
        &self,
        mut ray: Ray,
        minimum_distance: f32,
        maximum_hits: usize,
    ) -> (Vec<Vec2>, Vec2) {
        let mut positions = Vec::new();
        positions.push(ray.position());
        while let Some(intersection) = self.trace_single(&ray, minimum_distance) {
            ray.propagete(intersection);
            positions.push(ray.position());
            if positions.len() > maximum_hits {
                break;
            }
        }
        (positions, ray.direction())
    }
    fn trace_single(&self, ray: &Ray, minimum_distance: f32) -> Option<Intersection> {
        let intersections = self
            .objects
            .iter()
            .filter_map(|x| x.intersect(ray))
            .filter(|x| x.distance > minimum_distance)
            .collect::<Vec<_>>();
        let min = intersections
            .iter()
            .map(|x| x.distance)
            .fold(
                std::f32::INFINITY,
                |b, item| if item < b { item } else { b },
            );
        for intersection in intersections {
            if intersection.distance == min {
                return Some(intersection);
            }
        }
        None
    }
}
