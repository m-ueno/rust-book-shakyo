use std::collections::HashMap;
use std::io::Cursor;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use image::io::Reader as ImageReader;
use image::{ImageBuffer, Rgb, RgbImage};

// use complex::Complex128;
use crate::complex::Complex128;

const W: u32 = 2000;
const N: u32 = 8;

pub fn draw_zoom1() {
    let (xmin, ymin) = (-1.28525, 0.327272);
    let (xmax, ymax) = (xmin + 0.1, ymin + 0.1);

    draw("zoom1", N, xmin, ymin, xmax, ymax);
}

pub fn draw_zoom_go() {
    let (xmin, ymin) = (-1.5, -0.);
    let (xmax, ymax) = (xmin + 1e-14, ymin + 1e-14);

    draw("zoomgo", N, xmin, ymin, xmax, ymax);
}

pub fn draw0() {
    let xmin = -2.;
    let ymin = -2.;
    let xmax = 2.;
    let ymax = 2.;

    draw("origin", N, xmin, ymin, xmax, ymax);
}

pub fn draw(name: &str, concurrency: u32, xmin: f64, ymin: f64, xmax: f64, ymax: f64) {
    let width = W;
    let height = W;
    let height_t = W / concurrency;

    // let mut img: RgbImage = ImageBuffer::new(width, height);
    let background = Rgb([0, 0, 0]);

    let mut threads = vec![];
    let mut threadid_bytes = Arc::new(Mutex::new(HashMap::new()));

    for i in 0..concurrency {
        let threadid_bytes = Arc::clone(&threadid_bytes);
        let handle = thread::spawn(move || {
            // for py in 0..height {
            let mut bytes: Vec<u8> = vec![];
            for py in i * height_t..(i + 1) * height_t {
                let y = (py as f64) / (height as f64) * (ymax - ymin) + ymin;
                for px in 0..width {
                    let x = (px as f64) / (width as f64) * (xmax - xmin) + xmin;
                    let z = Complex128::new(x, y);
                    let rgb = match ycbcr(z) {
                        Some(color) => color,
                        None => background,
                    };
                    // let pixel = img.get_pixel_mut(px, py);
                    // *pixel = rgb;
                    // let image::Rgb(data) = *pixel;
                    bytes.push(rgb.0[0]);
                    bytes.push(rgb.0[1]);
                    bytes.push(rgb.0[2]);
                    // let mut img = img.lock().unwrap();
                    // img.put_pixel(px, py, rgb);
                }
            }
            threadid_bytes.lock().unwrap().insert(i, bytes);
        });
        threads.push(handle);
    }
    for t in threads {
        t.join();
    }

    let mut bytes: Vec<u8> = vec![];
    let mut threadid_bytes = threadid_bytes.lock().unwrap();
    for i in 0..concurrency {
        match threadid_bytes.get_mut(&i) {
            Some(v) => bytes.append(v),
            None => panic!("oh no"),
        };
    }

    let r = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?;
    // let img = ImageReader::new(Cursor::new(bytes)).decode()?;
    img.save(String::from("output_") + name + ".png").unwrap();
}

#[allow(dead_code)]
fn gray(z: Complex128) -> Option<Rgb<u8>> {
    const ITERATIONS: u8 = 250;

    let mut c = Complex128::new(0., 0.);
    for i in 0..ITERATIONS {
        c = c * c + z;
        if c.abs() > 2.0 {
            return Some(Rgb([i, i, i]));
        }
    }

    None
}

fn ycbcr_to_rgb(y: u8, cb: u8, cr: u8) -> [u8; 3] {
    // https://golang.org/src/image/color/ycbcr.go
    let yy1 = (y as u32) * 0x10101;
    let cb1 = (cb as u32) - 128;
    let cr1 = (cr as u32) - 128;

    let mut r = yy1 + 91881 * cr1;

    if r & 0xff000000 == 0 {
        r >>= 16;
    } else {
        // r = !^(r >> 31)
        r = !(r >> 31);
    }

    let mut g = yy1 - 22554 * cb1 - 46802 * cr1;

    if g & 0xff000000 == 0 {
        g >>= 16;
    } else {
        g = !(g >> 31);
    }

    let mut b = yy1 + 116130 * cb1;
    if b & 0xff000000 == 0 {
        b >>= 16;
    } else {
        b = !(b >> 31);
    }

    return [r as u8, g as u8, b as u8];
}

fn ycbcr(z: Complex128) -> Option<Rgb<u8>> {
    const ITERATIONS: u8 = 250;
    const CONTRAST: u8 = 150;

    let mut c = Complex128::new(0., 0.);
    for i in 0..ITERATIONS {
        c = c * c + z;
        if c.x * c.x + c.y * c.y > 4.0 {
            let rgb = ycbcr_to_rgb(
                i * CONTRAST,
                i * CONTRAST,
                // (((i as f64) * (i as f64) * CONTRAST) as u8) as f64,
                128,
            );

            return Some(Rgb(rgb));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ycbcr() {
        let z = Complex128::new(0.1, 0.);
        assert_eq!(ycbcr(z), None);
    }
}
