use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

impl Color {
    pub fn output(&self) {
        let r = self.x();
        let g = self.y();
        let b = self.z();

        // Translate the [0, 1] component values to the byte range [0, 255].
        let interval = Interval::new(0.0, 0.999999);
        let rb = (256.0 * interval.clamp(r)) as i32;
        let gb = (256.0 * interval.clamp(g)) as i32;
        let bb = (256.0 * interval.clamp(b)) as i32;

        println!("{} {} {}", rb, gb, bb);
    }
}
