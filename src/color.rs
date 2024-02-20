use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.sqrt()
}

pub fn write_color(pixel_color: Color, samples_per_pixel: u32) {
    let scale = 1.0 / samples_per_pixel as f64;
    let pixel_color = pixel_color * scale;
    let intensity = Interval::new(0.000, 0.999);

    let r = linear_to_gamma(pixel_color.x);
    let g = linear_to_gamma(pixel_color.y);
    let b = linear_to_gamma(pixel_color.z);

    let r = (256.0 * intensity.clamp(r)) as u32;
    let g = (256.0 * intensity.clamp(g)) as u32;
    let b = (256.0 * intensity.clamp(b)) as u32;

    print!("{r} {g} {b}\n");
}