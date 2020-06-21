use raytracer_shared::{Vec2,Intersection,Intersectable,Ray};

pub struct Circle {
    center: Vec2,
    radius_squared: f32,
}

impl Circle {
    pub fn new(center: Vec2, radius: f32) -> Self {
        Self {
            center,
            radius_squared: radius.powi(2),
        }
    }
}

impl Intersectable for Circle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let o = ray.position() - self.center;
        let projection = ray.direction() * o;
        let discrimanant = self.radius_squared + projection.powi(2) - o * o;
        if discrimanant <= 0. {
            None
        } else {
            let root = discrimanant.sqrt();
            let first = -projection - root;
            let distance = if first < 0. { first + root * 2. } else { first };
            let hit_point_relative_circle_center = o + ray.direction() * distance;
            let normal = -1.0 * hit_point_relative_circle_center.normalize();
            Some(Intersection { distance, normal })
        }
    }
}
