#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl std::ops::Mul<Vec2> for f32 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl std::ops::Mul<Vec2> for Vec2 {
    type Output = f32;
    fn mul(self, rhs: Vec2) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn rotate_90_degree_counter_clockwise(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }
    pub fn length_squared(self) -> f32 {
        self * self
    }
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }
    pub fn normalize(self) -> Self {
        self / self.length()
    }
    pub fn reflect(self, normal: Self) -> Self {
        self - 2.0 * (self * normal) * normal
    }
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.y,
            y: -self.y * rhs.x,
        }
    }
}

pub struct Ray {
    position: Vec2,
    direction: Vec2,
}

impl Ray {
    pub fn new(position: Vec2, direction: Vec2) -> Self {
        Self {
            position,
            direction,
        }
    }
    pub fn direction(&self) -> Vec2 {
        self.direction
    }
    pub fn position(&self) -> Vec2 {
        self.position
    }
    fn propagete(&mut self, intersection: Intersection) {
        self.position = self.position + self.direction * intersection.distance;
        self.direction = self.direction.reflect(intersection.normal)
    }
}

pub struct Intersection {
    pub distance: f32,
    pub normal: Vec2,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
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
        let projection = ray.direction * o;
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
