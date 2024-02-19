mod vec3;

fn main() {

    // image
    let image_width = 256;
    let image_height = 256;
    
    // render
    print!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..image_height {
        eprintln!("scanlines remaining: {}", image_height - j);
        for i in 0..image_height {
            let r = i as f32 / (image_width-1) as f32;
            let g = j as f32 / (image_height-1) as f32;
            let b = 0.0;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            print!("{ir} {ig} {ib}\n");
        }
    }
    eprintln!("Done rendering");
}
