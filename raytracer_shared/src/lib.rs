mod vec2;
pub use vec2::Vec2;
mod ray;
pub use ray::Ray;
mod intersection;
pub use intersection::{Intersectable, Intersection};
mod scene;
pub use scene::Scene;

fn version() -> (u32, u32, u32) {
    use pkg_version::{pkg_version_major, pkg_version_minor, pkg_version_patch};
    const MAJOR: u32 = pkg_version_major!();
    const MINOR: u32 = pkg_version_minor!();
    const PATCH: u32 = pkg_version_patch!();
    (MAJOR, MINOR, PATCH)
}
pub use version as line_version;
