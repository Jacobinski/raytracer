use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::random::RNG;
use crate::ray::Ray;
use crate::vec3::{unit_vector, Point3, Vec3};

// The maximum value for each color according to the PPM specification.
const MAX_COLOR: i32 = 256;

#[derive(Debug, PartialEq, Default)]
pub struct Camera {
    /// Public modifiable state
    aspect_ratio: f32, // Ratio of image width over height.
    image_width: u32,       // Rendered image width in pixel count
    samples_per_pixel: u32, // Number of samples for anti-aliasing

    /// Private internal state
    image_height: u32, // Height of rendered image
    center: Point3,      // Center of camera
    pixel00: Point3,     // Location of pixel (0,0)
    pixel_delta_u: Vec3, // Offset to rightward pixel
    pixel_delta_v: Vec3, // Offset to downward pixel
}

pub struct CameraBuilder {
    camera: Camera,
}

fn color(r: Ray, world: &HittableList) -> Color {
    if let Some(rec) = world.hit(r, &Interval::new(0.0, f32::INFINITY)) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

impl Camera {
    pub fn builder() -> CameraBuilder {
        CameraBuilder::new()
    }

    pub fn render(&self, world: &HittableList, rng: &mut RNG) {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("{}", MAX_COLOR);

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                if self.antialiasing_enabled() {
                    let mut c = Color::new(0.0, 0.0, 0.0);
                    for _sample in 0..self.samples_per_pixel {
                        let ray = self.stochastic_ray(i, j, rng);
                        c = c + color(ray, world);
                    }
                    c = c / self.samples_per_pixel as f32;
                    c.output();
                } else {
                    let ray = self.standard_ray(i, j);
                    let c = color(ray, world);
                    c.output();
                }
            }
        }
    }

    fn antialiasing_enabled(&self) -> bool {
        self.samples_per_pixel > 1
    }

    // Constructs a Ray from the Camera to the viewport pixel (u, v) with a bit
    // of stochastic offsetting to allow for anti-aliasing.
    fn stochastic_ray(&self, u: u32, v: u32, rng: &mut RNG) -> Ray {
        let x = rng.generate() - 0.5;
        let y = rng.generate() - 0.5;
        let pixel = self.pixel00
            + ((u as f32 + x) * self.pixel_delta_u)
            + ((v as f32 + y) * self.pixel_delta_v);
        let origin = self.center;
        let direction = pixel - origin;
        Ray::new(origin, direction)
    }

    // Constructs a Ray from the Camera to the viewport pixel (u, v) without
    // any randomness.
    fn standard_ray(&self, u: u32, v: u32) -> Ray {
        let pixel = self.pixel00 + (u * self.pixel_delta_u) + (v * self.pixel_delta_v);
        let origin = self.center;
        let direction = pixel - origin;
        Ray::new(origin, direction)
    }
}

impl CameraBuilder {
    pub fn new() -> CameraBuilder {
        CameraBuilder {
            camera: Camera {
                aspect_ratio: 1.0,
                image_width: 100,
                samples_per_pixel: 1,
                ..Default::default()
            },
        }
    }

    /// Ratio of image width over height.
    pub fn aspect_ratio(mut self, ratio: f32) -> Self {
        assert!(ratio > 0.0);
        self.camera.aspect_ratio = ratio;
        self
    }

    /// Rendered image width in pixel count
    pub fn image_width(mut self, width: u32) -> Self {
        assert!(width > 0);
        self.camera.image_width = width;
        self
    }

    /// Number of samples for anti-aliasing.
    /// Input value must be greater than or equal to one.
    /// A value of exactly one disables anti-aliasing.
    pub fn samples_per_pixel(mut self, samples_per_pixel: u32) -> Self {
        assert!(samples_per_pixel >= 1);
        self.camera.samples_per_pixel = samples_per_pixel;
        self
    }

    /// Create a Camera from a CameraBuilder
    pub fn build(self) -> Camera {
        // Extract builder variables
        let image_width = self.camera.image_width;
        let aspect_ratio = self.camera.aspect_ratio;
        let samples_per_pixel = self.camera.samples_per_pixel;
        assert!(samples_per_pixel >= 1);

        // Determine height of output image
        let image_height = (image_width as f32 / aspect_ratio) as u32;
        assert!(image_height >= 1);

        let center = Point3::new(0.0, 0.0, 0.0);

        // Determine viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * image_width as f32 / image_height as f32;

        // Calculate the vectors across the viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the delta vectors between pixels
        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            image_height,
            center,
            pixel00,
            pixel_delta_u,
            pixel_delta_v,
        }
    }
}

#[test]
fn builder_test() {
    let camera = Camera {
        aspect_ratio: 2.0,
        image_width: 200,
        samples_per_pixel: 1,
        image_height: 100,
        center: Point3::new(0.0, 0.0, 0.0),
        pixel00: Point3::new(-1.99, 0.99, -1.0),
        pixel_delta_u: Vec3::new(0.02, 0.0, 0.0),
        pixel_delta_v: Vec3::new(0.0, -0.02, 0.0),
    };
    let camera_from_builder = Camera::builder().aspect_ratio(2.0).image_width(200).build();
    assert_eq!(camera, camera_from_builder);
}
