use raytracer_shared::*;

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
