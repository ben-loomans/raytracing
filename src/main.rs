mod vec3;
mod color;

use crate::color::*;

fn main() {

    // image
    let image_width = 256;
    let image_height = 256;
    
    // render
    print!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..image_height {
        eprintln!("scanlines remaining: {}", image_height - j);
        for i in 0..image_height {

            let pixel = Color {
                x: i as f32 / (image_width-1) as f32,
                y: j as f32 / (image_height-1) as f32,
                z: 0.0,
            };

            write_color(pixel);
        }
    }
    eprintln!("Done rendering");
}
