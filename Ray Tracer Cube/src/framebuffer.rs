use raylib::prelude::*;

// framebuffer de software: un buffer de pixeles en CPU que se
// sube a una textura de raylib cada frame (swap_buffers)
pub struct Framebuffer {
    pub width: i32,
    pub height: i32,
    background_color: Color,
    current_color: Color,
    buffer: Vec<u8>, // RGBA8, largo = width * height * 4
    texture: Option<Texture2D>,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32, background_color: Color) -> Self {
        Framebuffer {
            width,
            height,
            background_color,
            current_color: Color::WHITE,
            buffer: color_to_rgba_bytes(background_color).repeat((width * height) as usize),
            texture: None,
        }
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    // pinta un pixel con el current_color, tal como pide el for de dos dimensiones del render
    pub fn point(&mut self, x: i32, y: i32) {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return;
        }
        let idx = ((y * self.width + x) * 4) as usize;
        self.buffer[idx..idx + 4].copy_from_slice(&color_to_rgba_bytes(self.current_color));
    }

    // sube el buffer de CPU a la textura y la dibuja estirada al tamano de la ventana
    pub fn swap_buffers(&mut self, window: &mut RaylibHandle, raylib_thread: &RaylibThread) {
        if self.texture.is_none() {
            let image = Image::gen_image_color(self.width, self.height, self.background_color);
            self.texture = window.load_texture_from_image(raylib_thread, &image).ok();
        }

        if let Some(texture) = &mut self.texture {
            let _ = texture.update_texture(&self.buffer);
        }

        let screen_w = window.get_screen_width() as f32;
        let screen_h = window.get_screen_height() as f32;

        let mut d = window.begin_drawing(raylib_thread);
        d.clear_background(Color::BLACK);
        if let Some(texture) = &self.texture {
            d.draw_texture_pro(
                texture,
                Rectangle::new(0.0, 0.0, self.width as f32, self.height as f32),
                Rectangle::new(0.0, 0.0, screen_w, screen_h),
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );
        }
    }
}

fn color_to_rgba_bytes(color: Color) -> [u8; 4] {
    [color.r, color.g, color.b, color.a]
}
