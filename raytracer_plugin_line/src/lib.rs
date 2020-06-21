pub use raytracer_shared::version as version_shared;
use raytracer_shared::{Intersectable, Intersection, Ray, Vec2};

pub fn version() -> (u32, u32, u32) {
    use pkg_version::{pkg_version_major, pkg_version_minor, pkg_version_patch};
    const MAJOR: u32 = pkg_version_major!();
    const MINOR: u32 = pkg_version_minor!();
    const PATCH: u32 = pkg_version_patch!();
    (MAJOR, MINOR, PATCH)
}

pub struct InfiniteLine {
    point_on_line: Vec2,
    normal: Vec2,
}

impl InfiniteLine {
    pub fn new(p: Vec2, q: Vec2) -> Self {
        let tanget = (p - q).normalize();
        let normal = tanget.rotate_90_degree_counter_clockwise();
        Self {
            point_on_line: p,
            normal,
        }
    }
}

impl Intersectable for InfiniteLine {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let projection = ray.direction() * self.normal;
        if projection.abs() < 1e-7 {
            None
        } else {
            let distance = (self.point_on_line - ray.position()) * self.normal / projection;
            if !distance.is_finite() || distance <= 0. {
                None
            } else {
                Some(Intersection {
                    distance,
                    normal: self.normal,
                })
            }
        }
    }
}
