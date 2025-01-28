use camera::Camera;
use hittable_list::HittableList;
use sphere::Sphere;
use vec3::Point3;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod random;
mod ray;
mod sphere;
mod vec3;

fn main() {
    let camera = Camera::builder()
        .aspect_ratio(16.0 / 9.0)
        .image_width(400)
        .samples_per_pixel(100)
        .build();

    let mut rng = random::Rng::new(42);

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    camera.render(&world, &mut rng);
}
