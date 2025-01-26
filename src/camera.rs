use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{unit_vector, Point3, Vec3};

// The maximum value for each color according to the PPM specification.
const MAX_COLOR: i32 = 256;

#[derive(Debug, PartialEq, Default)]
pub struct Camera {
    /// Public modifiable state
    aspect_ratio: f32, // Ratio of image width over height.
    image_width: u32, // Rendered image width in pixel count

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

    pub fn render(&self, world: &HittableList) {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("{}", MAX_COLOR);

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel_center =
                    self.pixel00 + (i * self.pixel_delta_u) + (j * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);

                let color = color(ray, world);
                color.output();
            }
        }
    }
}

impl CameraBuilder {
    pub fn new() -> CameraBuilder {
        CameraBuilder {
            camera: Camera {
                aspect_ratio: 1.0,
                image_width: 100,
                ..Default::default()
            },
        }
    }

    /// Ratio of image width over height.
    pub fn aspect_ratio(mut self, ratio: f32) -> Self {
        self.camera.aspect_ratio = ratio;
        self
    }

    /// Rendered image width in pixel count
    pub fn image_width(mut self, width: u32) -> Self {
        self.camera.image_width = width;
        self
    }

    /// Create a Camera from a CameraBuilder
    pub fn build(self) -> Camera {
        // Extract builder variables
        let image_width = self.camera.image_width;
        let aspect_ratio = self.camera.aspect_ratio;

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
        image_height: 100,
        center: Point3::new(0.0, 0.0, 0.0),
        pixel00: Point3::new(-1.99, 0.99, -1.0),
        pixel_delta_u: Vec3::new(0.02, 0.0, 0.0),
        pixel_delta_v: Vec3::new(0.0, -0.02, 0.0),
    };
    let camera_from_builder = Camera::builder().aspect_ratio(2.0).image_width(200).build();
    assert_eq!(camera, camera_from_builder);
}
