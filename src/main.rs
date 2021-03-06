#![feature(const_trait_impl)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_mut_refs)]
#![feature(const_eval_limit)]
#![const_eval_limit = "0"]

use image::ImageFormat;
use ray_tracing_in_one_weekend_compile_time::*;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: usize = 100;
const PIXEL_COUNT: usize = IMAGE_WIDTH * IMAGE_HEIGHT;

const fn ray_color<const N: usize>(ray: &Ray, world: &HittableList<N>) -> Color {
    if let Some(rec) = world.hit(&ray, 0.0, std::f64::INFINITY) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = unit_vector(&ray.direction());
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() -> anyhow::Result<()> {
    const PIXEL_COLORS: [Color; PIXEL_COUNT] = ray_trace();
    const BUFFER: [u8; PIXEL_COUNT * 3] = pixels_to_buffer(PIXEL_COLORS);
    let img = image::RgbImage::from_raw(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32, BUFFER.to_vec())
        .unwrap();
    img.save_with_format("./result.png", ImageFormat::Png)?;

    Ok(())
}

const fn ray_trace() -> [Color; PIXEL_COUNT] {
    let mut world: HittableList<2> = HittableList::<2>::new();
    world.add(Hittable::Sphere(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Hittable::Sphere(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    let cam = Camera::new();

    let mut rng = Xorshift::new(include!("rand_seed.txt"));
    let mut pixel_colors = [Color::ZERO; PIXEL_COUNT];
    let mut i = 0;
    let mut j = (IMAGE_HEIGHT - 1) as i32;
    let mut pixel_index = 0;
    let scale = 1.0 / SAMPLES_PER_PIXEL as f64;
    // forが使えないのでwhileで代用
    while 0 <= j {
        while i < IMAGE_WIDTH {
            let mut s = 0;
            let mut pixel_color = Color::ZERO;
            while s < SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen_f64()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rng.gen_f64()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world) * scale;
                s += 1;
            }
            pixel_colors[pixel_index] = pixel_color;
            i += 1;
            pixel_index += 1;
        }
        i = 0;
        j -= 1;
    }
    pixel_colors
}

const fn pixels_to_buffer(pixels: [Color; PIXEL_COUNT]) -> [u8; PIXEL_COUNT * 3] {
    //let buf: Vec<_> = PIXEL_COLORS
    //    .iter()
    //    .map(|&(r, g, b)| (r * 255.999, g * 255.999, b * 255.999))
    //    .map(|&(r, g, b)| [r as u8, g as u8, b as u8])
    //    .flatten()
    //    .collect();
    //buf

    // 上と同じことをしている
    let mut buf = [0; PIXEL_COUNT * 3];
    let mut i = 0;
    while i < PIXEL_COUNT {
        buf[i * 3 + 0] = (pixels[i].x * 255.999) as u8;
        buf[i * 3 + 1] = (pixels[i].y * 255.999) as u8;
        buf[i * 3 + 2] = (pixels[i].z * 255.999) as u8;
        i += 1;
    }
    buf
}
