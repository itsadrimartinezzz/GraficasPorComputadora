mod framebuffer;
mod line;

use framebuffer::Framebuffer;
use line::{draw_line_bresenham, draw_line_dda, draw_line_pendiente};

fn main() {
    let pink = [255, 105, 180];

    let mut fb = Framebuffer::new(800, 600);
    draw_line_pendiente(&mut fb, 173, 537, 289, 97, pink);
    fb.write_bmp("pendiente.bmp").expect("no se pudo escribir pendiente.bmp");

    let mut fb = Framebuffer::new(800, 600);
    draw_line_dda(&mut fb, 173, 537, 289, 97, pink);
    fb.write_bmp("dda.bmp").expect("no se pudo escribir dda.bmp");

    let mut fb = Framebuffer::new(800, 600);
    draw_line_bresenham(&mut fb, 173, 537, 289, 97, pink);
    fb.write_bmp("bresenham.bmp").expect("no se pudo escribir bresenham.bmp");

    println!("Imagenes escritas: pendiente.bmp, dda.bmp, bresenham.bmp");
}
