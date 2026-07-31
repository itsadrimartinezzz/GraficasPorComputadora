use raylib::prelude::*;
use crate::framebuffer::Framebuffer;

// colores del juego viva, muerta, y "recien murio" para el efecto de rastro
pub const ALIVE: Color = Color::new(255, 105, 180, 255); // rosa
pub const DEAD: Color = Color::BLACK;
pub const DYING: Color = Color::new(200, 162, 230, 255); // lila

// compara a mano (r,g,b) en vez de == directo, por si acaso
pub fn es_vivo(color: Color) -> bool {
    color.r == ALIVE.r && color.g == ALIVE.g && color.b == ALIVE.b
}

// cuenta cuantas de las 8 vecinas estan vivas
fn contar_vecinos_vivos(fb: &Framebuffer, x: i32, y: i32) -> u8 {
    let mut contador = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue; // esta celda no es su propia vecina
            }
            if es_vivo(fb.get_color(x + dx, y + dy)) {
                contador += 1;
            }
        }
    }
    contador
}

// aca pasa todo, calcula la siguiente generacion y la escribe con point
pub fn render(fb: &mut Framebuffer) {
    // uso un buffer aparte pq si escribo directo se me mezclan generaciones
    let mut siguiente: Vec<Color> = Vec::with_capacity((fb.width * fb.height) as usize);

    for y in 0..fb.height {
        for x in 0..fb.width {
            let vecinos = contar_vecinos_vivos(fb, x, y);
            let viva = es_vivo(fb.get_color(x, y));

            // las 4 reglas de conway, tal cual
            let nueva_viva = match (viva, vecinos) {
                (true, 2) | (true, 3) => true,  // sobrevive
                (true, _) => false,             // se muere por poca o mucha compania
                (false, 3) => true,             // nace
                (false, _) => false,            // sigue muerta
            };

            // decido el color, viva (rosa), recien muerta (lila), o ya muerta hace rato
            let nuevo_color = if nueva_viva {
                ALIVE
            } else if viva {
                DYING
            } else {
                DEAD
            };

            siguiente.push(nuevo_color);
        }
    }

    // escribo el resultado real usando point
    let mut i = 0;
    for y in 0..fb.height {
        for x in 0..fb.width {
            fb.point(x, y, siguiente[i]);
            i += 1;
        }
    }
}