use crate::framebuffer::Framebuffer;
use crate::life::ALIVE;

// coloca cualquier patron (lista de offsets) en el punto que yo quiera
pub fn colocar_patron(fb: &mut Framebuffer, patron: &[(i32, i32)], origen_x: i32, origen_y: i32) {
    for &(dx, dy) in patron {
        fb.point(origen_x + dx, origen_y + dy, ALIVE);
    }
}

// se mueve solo en diagonal, sin que yo programe ningun movimiento 
pub fn glider() -> Vec<(i32, i32)> {
    vec![
        (1, 0),
        (2, 1),
        (0, 2), (1, 2), (2, 2),
    ]
}

// oscilador simple, parpadea entre horizontal y vertical
pub fn blinker() -> Vec<(i32, i32)> {
    vec![(0, 0), (1, 0), (2, 0)]
}

// otro oscilador, un poco mas grande que el blinker
pub fn toad() -> Vec<(i32, i32)> {
    vec![
        (1, 0), (2, 0), (3, 0),
        (0, 1), (1, 1), (2, 1),
    ]
}

// el cañon dispara gliders para siempre, nunca se apaga
pub fn gosper_glider_gun() -> Vec<(i32, i32)> {
    vec![
        (24, 0),
        (22, 1), (24, 1),
        (12, 2), (13, 2), (20, 2), (21, 2), (34, 2), (35, 2),
        (11, 3), (15, 3), (20, 3), (21, 3), (34, 3), (35, 3),
        (0, 4), (1, 4), (10, 4), (16, 4), (20, 4), (21, 4),
        (0, 5), (1, 5), (10, 5), (14, 5), (16, 5), (17, 5), (22, 5), (24, 5),
        (10, 6), (16, 6), (24, 6),
        (11, 7), (15, 7),
        (12, 8), (13, 8),
    ]
}