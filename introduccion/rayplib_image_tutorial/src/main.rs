use raylib::prelude::*;

fn main() {
    let image_width = 500;
    let image_height = 500;

    let mut new_image = Image::gen_image_color(
        image_width,
        image_height,
        Color::BLACK
    );

    let pixel_postion_x  =100;
    let pixel_postion_y  =100;

    new_image.draw_pixel(
        pixel_postion_x,
        pixel_postion_y,
        Color::WHITE
    );

    let output_file_name = " my_fisrt_image.png";
    new_image.export_image(output_file_name);
    println!("Image saved successfully as '{}'!", output_file_name);

}



