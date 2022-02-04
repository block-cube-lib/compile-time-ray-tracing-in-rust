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
const PIXEL_COUNT: usize = IMAGE_WIDTH * IMAGE_HEIGHT;

const fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin() - center.clone();
    let a = dot(&r.direction(), &r.direction());
    let b = 2.0 * dot(&oc, &r.direction());
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

const fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, 1.0), 0.5, &ray) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = unit_vector(&ray.direction());
    let t = 0.5 * unit_direction.y + 1.0;
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
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ASPECT_RATIO;
    let focal_length = 1.0;

    let origin = Point3::ZERO;
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut pixel_colors = [Color::ZERO; PIXEL_COUNT];
    let mut i = 0;
    let mut j = (IMAGE_HEIGHT - 1) as i32;
    // forが使えないのでwhileで代用
    while 0 <= j {
        while i < IMAGE_WIDTH {
            let u = (i as f64) / (IMAGE_WIDTH - 1) as f64;
            let v = (j as f64) / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(
                &origin,
                &(lower_left_corner + u * horizontal + v * vertical - origin),
            );
            pixel_colors[j as usize * IMAGE_WIDTH + i] = ray_color(&r);
            i += 1;
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
