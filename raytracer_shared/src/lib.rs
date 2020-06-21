mod vec2;
pub use vec2::Vec2;
mod ray;
pub use ray::Ray;
mod intersection;
pub use intersection::{Intersectable, Intersection};
mod scene;
pub use scene::Scene;

pub struct VersionInfo {
    pub crate_name: &'static str,
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub dependencies: Vec<VersionInfo>,
}

pub const fn version_info() -> VersionInfo {
    const CRATE_NAME: &'static str = env!("CARGO_PKG_NAME");
    use pkg_version::{pkg_version_major, pkg_version_minor, pkg_version_patch};
    const MAJOR: u32 = pkg_version_major!();
    const MINOR: u32 = pkg_version_minor!();
    const PATCH: u32 = pkg_version_patch!();
    VersionInfo {
        crate_name: CRATE_NAME,
        major: MAJOR,
        minor: MINOR,
        patch: PATCH,
        dependencies: Vec::new(),
    }
}
