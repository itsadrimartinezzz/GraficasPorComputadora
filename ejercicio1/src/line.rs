use crate::framebuffer::Framebuffer;

// Pendiente: y = m*x + b
pub fn draw_line_pendiente(fb: &mut Framebuffer, x0: isize, y0: isize, x1: isize, y1: isize, color: [u8; 3]) {
    let m = (y1 - y0) as f64 / (x1 - x0) as f64;
    let b = y0 as f64 - m * x0 as f64;

    if (x1 - x0).abs() >= (y1 - y0).abs() {
        // línea más horizontal que vertical: camino en x
        for x in x0.min(x1)..=x0.max(x1) {
            let y = (m * x as f64 + b).round() as isize;
            fb.set_pixel(x, y, color);
        }
    } else {
        // línea más vertical que horizontal: camino en y para no dejar huecos
        for y in y0.min(y1)..=y0.max(y1) {
            let x = ((y as f64 - b) / m).round() as isize;
            fb.set_pixel(x, y, color);
        }
    }
}

// DDA: pasitos iguales de x_inc, y_inc
pub fn draw_line_dda(fb: &mut Framebuffer, x0: isize, y0: isize, x1: isize, y1: isize, color: [u8; 3]) {
    let steps = (x1 - x0).abs().max((y1 - y0).abs());
    let x_inc = (x1 - x0) as f64 / steps as f64;
    let y_inc = (y1 - y0) as f64 / steps as f64;

    let mut x = x0 as f64;
    let mut y = y0 as f64;

    for _ in 0..=steps {
        fb.set_pixel(x.round() as isize, y.round() as isize, color);
        x += x_inc;
        y += y_inc;
    }
}

// Bresenham: solo con enteros
pub fn draw_line_bresenham(fb: &mut Framebuffer, x0: isize, y0: isize, x1: isize, y1: isize, color: [u8; 3]) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let mut x = x0;
    let mut y = y0;

    loop {
        fb.set_pixel(x, y, color);
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}
