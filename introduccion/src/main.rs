use raylib::prelude::*;

fn main() {
    let image_width = 500;
    let image_height = 500;

    let raw_image = unsafe {
        raylib::ffi::GenImageColor(
            image_width,
            image_height,
            Color::WHITE,
        )
    };

    let mut new_image = unsafe {
        Image::from_raw(raw_image)
    };

    for x in 0..image_width {
        for y in 0..image_height {
            let nx = (x as f32 - 250.0) / 100.0;
            let ny = -(y as f32 - 250.0) / 100.0;

            let equation =
                (nx * nx + ny * ny - 1.0).powi(3)
                - nx * nx * ny.powi(3);

            if equation <= 0.0 {
                new_image.draw_pixel(
                    x,
                    y,
                    Color::RED,
                );
            }
        }
    }

    let output_file_name = "my_first_image.png";

    new_image.export_image(output_file_name);

    println!(
        "Corazón guardado como '{}'!",
        output_file_name
    );
}