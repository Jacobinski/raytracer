use color::Color;
use ray::Ray;
use vec3::{dot, unit_vector, Point3, Vec3};

mod color;
mod ray;
mod vec3;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
// TODO: Make sure this is always >= 1 with a terenary operator.
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;

const FOCAL_LENGTH: f32 = 1.0;
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32;

const MAX_COLOR: i32 = 256;

fn color(r: Ray) -> Color {
    if let Some(t) = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        let n = unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }
    let unit_direction = unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: Point3, radius: f32, r: Ray) -> Option<f32> {
    let oc = center - r.origin();
    let a = dot(r.direction(), r.direction());
    let b = -2.0 * dot(r.direction(), oc);
    let c = dot(oc, oc) - (radius * radius);
    let discriminant = b * b - (4.0 * a * c);
    if discriminant >= 0.0 {
        Some((-b - f32::sqrt(discriminant)) / (2.0 * a))
    } else {
        None
    }
}

fn main() {
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("{}", MAX_COLOR);

    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the viewport edges
    let viewport_u = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

    // Calculate the delta vectors between pixels
    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f32;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f32;

    // Calculate the location of the upper left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    for j in 0..IMAGE_HEIGHT {
        // eprintln!("Scanlines remaining: {}", IMAGE_HEIGHT - j);
        for i in 0..IMAGE_WIDTH {
            let pixel_center = pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;

            let ray = Ray::new(camera_center, ray_direction);
            let color = color(ray);
            color.output();
        }
    }
    // eprintln!("done");
}
