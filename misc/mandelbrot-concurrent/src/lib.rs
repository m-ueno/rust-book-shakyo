mod complex;
pub mod mandelbrot;

#[cfg(test)]
mod tests {
    #[test]
    fn test_chunks_mut() {
        use image::{ImageBuffer, RgbImage};

        const N: u32 = 100;
        let mut buf: RgbImage = ImageBuffer::new(N, N);
        for (i, chunk) in buf.chunks_mut((N * N * 3 / 4) as usize).enumerate() {
            println!("i={} len(chunk)={}", i, chunk.len());
            for p in chunk.iter_mut() {
                *p = i as u8 * (255 / 4);
            }
        }

        buf.save("test.png").unwrap();

        assert_eq!(1 + 2, 3);
    }
}
