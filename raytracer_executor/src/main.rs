use raytracer_plugin_circle::Circle;
use raytracer_plugin_line::InfiniteLine;
use raytracer_shared::*;

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
