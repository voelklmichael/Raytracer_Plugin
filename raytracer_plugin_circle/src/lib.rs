use raytracer_shared::{Intersectable, Intersection, Ray, Vec2};
pub use raytracer_shared::version as version_shared;

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

pub fn version() -> (u32, u32, u32) {
    use pkg_version::{pkg_version_major, pkg_version_minor, pkg_version_patch};
    const MAJOR: u32 = pkg_version_major!();
    const MINOR: u32 = pkg_version_minor!();
    const PATCH: u32 = pkg_version_patch!();
    (MAJOR, MINOR, PATCH)
}
