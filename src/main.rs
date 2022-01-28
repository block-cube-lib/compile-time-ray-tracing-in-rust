#![feature(const_fn_floating_point_arithmetic)] // const fnの中で浮動小数点演算を行うために必要

use image::ImageFormat;
use ray_tracing_in_one_weekend_compile_time::Color;

const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;
const PIXEL_COUNT: usize = IMAGE_WIDTH * IMAGE_HEIGHT;

fn main() -> anyhow::Result<()> {
    const PIXEL_COLORS: [Color; PIXEL_COUNT] = ray_trace();
    const BUFFER: [u8; PIXEL_COUNT * 3] = pixels_to_buffer(PIXEL_COLORS);
    let img = image::RgbImage::from_raw(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32, BUFFER.to_vec())
        .unwrap();
    img.save_with_format("./result.png", ImageFormat::Png)?;

    Ok(())
}

const fn ray_trace() -> [Color; PIXEL_COUNT] {
    let mut pixel_colors = [Color::ZERO; PIXEL_COUNT];
    let mut i = 0;
    let mut j = (IMAGE_HEIGHT - 1) as i32;
    // forが使えないのでwhileで代用
    while 0 <= j {
        while i < IMAGE_WIDTH {
            pixel_colors[j as usize * IMAGE_WIDTH + i] = Color::new(
                (i as f64) / ((IMAGE_WIDTH - 1) as f64),
                j as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.25,
            );
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
