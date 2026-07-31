use raylib::prelude::*;

// el tablero del juego vive en memoria,no es una imagen todavia
pub struct Framebuffer {
    pub width: i32,
    pub height: i32,
    pub buffer: Vec<Color>, // un color por celda, todo en una sola lista
}

impl Framebuffer {
    pub fn new(width: i32, height: i32, fondo: Color) -> Self {
        // arranca todo del mismo color (negro en mi caso)
        let buffer = vec![fondo; (width * height) as usize];
        Framebuffer { width, height, buffer }
    }

    // pinta una celda - la unica forma permitida de escribir en el framebuffer
    pub fn point(&mut self, x: i32, y: i32, color: Color) {
        // si se sale del tablero, simplemente no hace nada
        if x >= 0 && y >= 0 && x < self.width && y < self.height {
            let idx = (y * self.width + x) as usize; // paso de (x,y) a indice plano
            self.buffer[idx] = color;
        }
    }

    // lee el color de una celda - orillas en modo loop (se conecta con el otro lado)
    pub fn get_color(&self, x: i32, y: i32) -> Color {
        // el doble modulo es pq en rust -1 % ancho da negativo, esto lo arregla
        let wx = ((x % self.width) + self.width) % self.width;
        let wy = ((y % self.height) + self.height) % self.height;
        let idx = (wy * self.width + wx) as usize;
        self.buffer[idx]
    }
}