#![feature(const_fn_floating_point_arithmetic)] // const fnの中で浮動小数点演算を行うために必要

use image::ImageFormat;

const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;
const PIXEL_COUNT: usize = IMAGE_WIDTH * IMAGE_HEIGHT;

fn main() -> anyhow::Result<()> {
    const PIXEL_COLORS: [(u8, u8, u8); PIXEL_COUNT] = ray_trace();
    const BUFFER: [u8; PIXEL_COUNT * 3] = pixels_to_buffer(PIXEL_COLORS);
    let img = image::RgbImage::from_raw(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32, BUFFER.to_vec())
        .unwrap();
    img.save_with_format("./result.png", ImageFormat::Png)?;

    Ok(())
}

const fn ray_trace() -> [(u8, u8, u8); PIXEL_COUNT] {
    let mut pixel_colors = [(0, 0, 0); PIXEL_COUNT];
    let mut i = 0;
    let mut j = (IMAGE_HEIGHT - 1) as i32;
    // forが使えないのでwhileで代用
    while 0 < j {
        while i < IMAGE_WIDTH {
            let r = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;
            let (r, g, b) = (
                (r * 255.999) as u8,
                (g * 255.999) as u8,
                (b * 255.999) as u8,
            );
            pixel_colors[j as usize * IMAGE_WIDTH + i] = (r, g, b);
            i += 1;
        }
        i = 0;
        j -= 1;
    }
    pixel_colors
}

const fn pixels_to_buffer(pixels: [(u8, u8, u8); PIXEL_COUNT]) -> [u8; PIXEL_COUNT * 3] {
    //let buf: Vec<_> = PIXEL_COLORS
    //    .iter()
    //    .map(|&(r, g, b)| [r, g, b])
    //    .flatten()
    //    .collect();
    //buf

    // 上と同じことをしている
    let mut buf = [0; PIXEL_COUNT * 3];
    let mut i = 0;
    while i < PIXEL_COUNT {
        buf[i * 3 + 0] = pixels[i].0;
        buf[i * 3 + 1] = pixels[i].1;
        buf[i * 3 + 2] = pixels[i].2;
        i += 1;
    }
    buf
}
