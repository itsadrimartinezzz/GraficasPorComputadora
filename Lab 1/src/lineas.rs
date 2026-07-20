use image::RgbImage;
use crate::framebuffer::pintar_pixel;

pub type Punto = (i32, i32);

pub fn dibujar_linea(img: &mut RgbImage, a: Punto, b: Punto, color: [u8; 3]) {
    let (mut x0, mut y0) = a;
    let (x1, y1) = b;
 
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let paso_x = if x0 < x1 { 1 } else { -1 };
    let paso_y = if y0 < y1 { 1 } else { -1 };
    let mut error = dx + dy;
 
    loop {
        pintar_pixel(img, x0, y0, color);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * error;
        if e2 >= dy {
            error += dy;
            x0 += paso_x;
        }
        if e2 <= dx {
            error += dx;
            y0 += paso_y;
        }
    }
}
 

// El contorno 
pub fn dibujar_contorno(img: &mut RgbImage, puntos: &[Punto], color: [u8; 3]) {
    for i in 0..puntos.len() {
        let actual = puntos[i];
        let siguiente = puntos[(i + 1) % puntos.len()]; // cierra el poligono
        dibujar_linea(img, actual, siguiente, color);
    }
}


// Relleno del poligono con espacios
pub fn rellenar_poligono(img: &mut RgbImage, exterior: &[Punto], agujeros: &[&[Punto]], color: [u8; 3]) {
    
    let mut anillos: Vec<&[Punto]> = vec![exterior];
    anillos.extend_from_slice(agujeros);
 
    
    let mut y_min = i32::MAX;
    let mut y_max = i32::MIN;
    for anillo in &anillos {
        for &(_, y) in anillo.iter() {
            if y < y_min { y_min = y; }
            if y > y_max { y_max = y; }
        }
    }
 
    
    for y in y_min..=y_max {
        
        let mut cruces: Vec<i32> = Vec::new();
 
        for anillo in &anillos {
            let n = anillo.len();
            for i in 0..n {
                let (x0, y0) = anillo[i];
                let (x1, y1) = anillo[(i + 1) % n];
 
                
                let (ya, yb, xa, xb) = if y0 < y1 {
                    (y0, y1, x0, x1)
                } else {
                    (y1, y0, x1, x0)
                };
 
                
                if y >= ya && y < yb {
                    let x = xa + (xb - xa) * (y - ya) / (yb - ya);
                    cruces.push(x);
                }
            }
        }

        cruces.sort();
 
        let mut i = 0;
        while i + 1 < cruces.len() {
            for x in cruces[i]..=cruces[i + 1] {
                pintar_pixel(img, x, y, color);
            }
            i += 2;
        }
    }
}