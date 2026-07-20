use image::{Rgb, RgbImage};

// crea la imagen con color de fondo
pub fn crear_imagen(ancho: u32, alto: u32, fondo: [u8; 3]) -> RgbImage {
    let mut img = RgbImage::new(ancho, alto);
    for pixel in img.pixels_mut() {
        *pixel = Rgb(fondo);
    }
    img
}
 
// pinta un pixel con sus coordenadas
pub fn pintar_pixel(img: &mut RgbImage, x: i32, y: i32, color: [u8; 3]) {
    if x >= 0 && y >= 0 && (x as u32) < img.width() && (y as u32) < img.height() {
        img.put_pixel(x as u32, y as u32, Rgb(color));
    }
}
 
// out png
pub fn guardar(img: &RgbImage, nombre: &str) {
    let volteada = image::imageops::flip_horizontal(img);
    let volteada = image::imageops::flip_vertical(&volteada);
    volteada.save(nombre).expect("no se pudo guardar la imagen");
}
