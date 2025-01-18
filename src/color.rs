use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn output(&self) {
        let r = self.x();
        let g = self.y();
        let b = self.z();

        // Translate the [0, 1] component values to the byte range [0, 255].
        let rb = (255.999 * r) as i32;
        let gb = (255.999 * g) as i32;
        let bb = (255.999 * b) as i32;

        println!("{} {} {}", rb, gb, bb);
    }
}
