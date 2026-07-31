mod framebuffer;
mod life;
mod patterns;

use raylib::prelude::*;
use rand::Rng;
use framebuffer::Framebuffer;
use life::{render, ALIVE, DEAD};
use patterns::{colocar_patron, glider, blinker, toad, gosper_glider_gun};

fn main() {
    // tamano real del juego, lo mantengo chico por rendimiento
    let fb_width = 100;
    let fb_height = 100;

    // cada celda se ve como un bloque de "escala" pixeles en la ventana
    let escala = 6;
    let win_width = fb_width * escala;
    let win_height = fb_height * escala;

    let (mut rl, thread) = raylib::init()
        .size(win_width, win_height)
        .title("Conway's Game of Life")
        .build();

    rl.set_target_fps(10); // esto es mi delay entre frames

    let mut fb = Framebuffer::new(fb_width, fb_height, DEAD);

    // relleno aleatorio para que arranque bien lleno, no vacio
    let mut rng = rand::thread_rng();
    for y in 0..fb.height {
        for x in 0..fb.width {
            if rng.gen_bool(0.25) {
                fb.point(x, y, ALIVE);
            }
        }
    }

    // encima del ruido, meto un par de patrones reconocibles
    colocar_patron(&mut fb, &glider(), 5, 5);
    colocar_patron(&mut fb, &gosper_glider_gun(), 60, 60);

    while !rl.window_should_close() {
        render(&mut fb); // calcula la siguiente generacion

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(DEAD); // limpia la ventana, no el framebuffer del juego

        // dibuja el framebuffer completo, celda por celda, escalado
        for y in 0..fb.height {
            for x in 0..fb.width {
                let color = fb.get_color(x, y);
                d.draw_rectangle(x * escala, y * escala, escala, escala, color);
            }
        }
    }
}