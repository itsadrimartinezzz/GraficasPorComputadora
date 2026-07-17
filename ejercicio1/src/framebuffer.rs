use std::fs::File;
use std::io::{self, Write};

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pixels: Vec<[u8; 3]>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer { width, height, pixels: vec![[0, 0, 0]; width * height] }
    }

    pub fn set_pixel(&mut self, x: isize, y: isize, color: [u8; 3]) {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            self.pixels[y as usize * self.width + x as usize] = color;
        }
    }

    pub fn write_bmp(&self, path: &str) -> io::Result<()> {
        let row = self.width * 3 + (4 - self.width * 3 % 4) % 4;
        let size = 54 + row * self.height;
        let mut buf = vec![0u8; size];

        buf[0..2].copy_from_slice(b"BM");
        buf[2..6].copy_from_slice(&(size as u32).to_le_bytes());
        buf[10..14].copy_from_slice(&54u32.to_le_bytes());
        buf[14..18].copy_from_slice(&40u32.to_le_bytes());
        buf[18..22].copy_from_slice(&(self.width as i32).to_le_bytes());
        buf[22..26].copy_from_slice(&(self.height as i32).to_le_bytes());
        buf[26..28].copy_from_slice(&1u16.to_le_bytes());
        buf[28..30].copy_from_slice(&24u16.to_le_bytes());

        for y in 0..self.height {
            for x in 0..self.width {
                let [r, g, b] = self.pixels[y * self.width + x];
                let i = 54 + (self.height - 1 - y) * row + x * 3;
                buf[i..i + 3].copy_from_slice(&[b, g, r]);
            }
        }

        File::create(path)?.write_all(&buf)
    }
}
