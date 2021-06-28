// Rust version from https://github.com/m-ueno/gopl.io/blob/master/ch03/mandelbrot/newton.go

use image::{ImageBuffer, Rgb, RgbImage};
use num_complex::Complex64;

const W: u32 = 800;

pub fn draw() {
    let (xmin, ymin, xmax, ymax) = (-2., -2., 2., 2.);

    let width = W;
    let height = W;

    let mut img: RgbImage = ImageBuffer::new(width, height);
    let background = Rgb([0, 0, 0]);

    for py in 0..height {
        let y = (py as f64) / (height as f64) * (ymax - ymin) + ymin;
        for px in 0..width {
            let x = (px as f64) / (width as f64) * (xmax - xmin) + xmin;
            let z = Complex64::new(x, y);
            let rgb = match newton(z) {
                Some(color) => color,
                None => background,
            };
            img.put_pixel(px, py, rgb);
        }
    }

    img.save("newton.png").unwrap();
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

// f(z) = z^4 - 1
fn newton(zz: Complex64) -> Option<Rgb<u8>> {
    let mut z = zz;
    let iterations = 200;
    let contrast = 15;
    let epsilon = 1e-6;

    for i in 0..iterations {
        let y = z * z * z * z - 1.;
        if y.norm() < epsilon {
            return Some(Rgb(ycbcr_to_rgb(contrast * i, contrast * i, 128)));
        }
        z = -y / (z * z * z * 4.) + z;
    }
    None
}
