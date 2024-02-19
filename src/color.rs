use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel_color: Color) {
    let r = (255.999 * pixel_color.x) as u32;
    let g = (255.999 * pixel_color.y) as u32;
    let b = (255.999 * pixel_color.z) as u32;

    print!("{r} {g} {b}\n");
}