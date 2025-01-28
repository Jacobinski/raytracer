use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

/// Applies an inverse Gamma-2 correction to the linear component to allow
/// photo viewing software to more accurately display the image.
/// https://en.wikipedia.org/wiki/Gamma_correction
#[inline]
pub fn linear_to_gamma(component: f64) -> f64 {
    if component > 0.0 {
        f64::sqrt(component)
    } else {
        0.0
    }
}

impl Color {
    pub fn output(&self) {
        let r = linear_to_gamma(self.x());
        let g = linear_to_gamma(self.y());
        let b = linear_to_gamma(self.z());

        // Translate the [0, 1] component values to the byte range [0, 255].
        let interval = Interval::new(0.0, 0.999999);
        let rb = (256.0 * interval.clamp(r)) as i32;
        let gb = (256.0 * interval.clamp(g)) as i32;
        let bb = (256.0 * interval.clamp(b)) as i32;

        println!("{} {} {}", rb, gb, bb);
    }
}
