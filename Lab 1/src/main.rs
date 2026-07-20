mod framebuffer;
mod lineas;

use framebuffer::{crear_imagen, guardar};
use lineas::{dibujar_contorno, rellenar_poligono, Punto};


fn main() {
    let mut img = crear_imagen(800, 600, [245, 240, 230]);

  // los poligodos a dibujar
    let poligono1: Vec<Punto> = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),
    ];
 
    let poligono2: Vec<Punto> = vec![
        (321, 335), (288, 286), (339, 251), (374, 302),
    ];
 
    let poligono3: Vec<Punto> = vec![
        (377, 249), (411, 197), (436, 249),
    ];
 
    let poligono4: Vec<Punto> = vec![
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52),
        (750, 145), (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230),
        (597, 215), (552, 214), (517, 144), (466, 180),
    ];
 
    
    let poligono5: Vec<Punto> = vec![
        (682, 175), (708, 120), (735, 148), (739, 170),
    ];
 
    // rellenar poligonos

    rellenar_poligono(&mut img, &poligono1, &[], [255, 179, 198]);        // rosa
    rellenar_poligono(&mut img, &poligono2, &[], [199, 178, 222]);        // lila
    rellenar_poligono(&mut img, &poligono3, &[], [255, 236, 153]);        // amarillo
    rellenar_poligono(&mut img, &poligono4, &[&poligono5], [155, 226, 222]); // aqua
 
    // dibujar contornos
    dibujar_contorno(&mut img, &poligono1, [128, 128, 128]);
    dibujar_contorno(&mut img, &poligono2, [128, 128, 128]);
    dibujar_contorno(&mut img, &poligono3, [128, 128, 128]);
    dibujar_contorno(&mut img, &poligono4, [128, 128, 128]);
    dibujar_contorno(&mut img, &poligono5, [128, 128, 128]);
 
    // out png
    guardar(&img, "out.png");
    println!("Imagen generada: out.png");


}