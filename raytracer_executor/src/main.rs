use raytracer_shared::*;
use raytracer_plugin_line::*;

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

fn main() {
    let y_axis = InfiniteLine::new(Vec2::new(0., 0.), Vec2::new(0., 1.));
    let x_axis = InfiniteLine::new(Vec2::new(0., 0.), Vec2::new(1., 0.));

    let mut scene = Scene::new();
    scene.push(y_axis);
    scene.push(x_axis);

    let ray = Ray::new(Vec2::new(1., 1.), Vec2::new(-0.3, -0.6).normalize());
    let (points, final_direction) = scene.trace(ray, 0.0001, 50);
    println!("{:?}", points);
    println!("{:?}", final_direction);
    let ray = Ray::new(Vec2::new(1., 1.), Vec2::new(0.3, 0.6).normalize());
    let (points, final_direction) = scene.trace(ray, 0.0001, 50);
    println!("{:?}", points);
    println!("{:?}", final_direction);

    let y_axis = InfiniteLine::new(Vec2::new(0., 0.), Vec2::new(0., 1.));
    let x_axis = InfiniteLine::new(Vec2::new(0., 0.), Vec2::new(1., 0.));
    let circle = Circle::new(Vec2::new(0.5, 0.5), 0.5);

    let mut scene = Scene::new();
    scene.push(y_axis);
    scene.push(x_axis);
    scene.push(circle);

    let ray = Ray::new(Vec2::new(1., 1.), Vec2::new(-0.3, -0.6).normalize());
    let (points, final_direction) = scene.trace(ray, 0.0001, 50);
    println!("{:?}", points);
    println!("{:?}", final_direction);
    let ray = Ray::new(Vec2::new(1., 1.), Vec2::new(0.3, 0.6).normalize());
    let (points, final_direction) = scene.trace(ray, 0.0001, 50);
    println!("{:?}", points);
    println!("{:?}", final_direction);

    println!("Hello, world!");
}
