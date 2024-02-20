use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

pub fn write_color(pixel_color: Color, samples_per_pixel: u32) {
    let scale = 1.0 / samples_per_pixel as f64;
    let pixel_color = pixel_color * scale;
    let intensity = Interval::new(0.000, 0.999);

    let r = (256.0 * intensity.clamp(pixel_color.x)) as u32;
    let g = (256.0 * intensity.clamp(pixel_color.y)) as u32;
    let b = (256.0 * intensity.clamp(pixel_color.z)) as u32;

    print!("{r} {g} {b}\n");
}