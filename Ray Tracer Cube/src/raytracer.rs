use raylib::prelude::Color;

use crate::framebuffer::Framebuffer;
use crate::light::Light;
use crate::ray_intersect::{Object, RayIntersect};
use crate::vec3::{normalize, Vec3};

// solo luz ambiente + difusa (Lambert), sin especular ni reflejos
pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Object], light: &Light) -> Color {
    let mut zbuffer = f32::INFINITY;
    let mut hit = None;

    // el objeto mas cercano al origen del rayo es el que se pinta
    for object in objects {
        if let Some(intersect) = object.ray_intersect(ray_origin, ray_direction) {
            if intersect.distance < zbuffer {
                zbuffer = intersect.distance;
                hit = Some(intersect);
            }
        }
    }

    match hit {
        Some(intersect) => {
            let light_dir = normalize(&(light.position - intersect.point));
            let diffuse_intensity = intersect.normal.dot(&light_dir).max(0.0);
            let ambient = 0.2;
            let intensity = (ambient + diffuse_intensity * light.intensity).min(1.0);

            let base = intersect.material.diffuse;
            Color::new(
                (base.r as f32 * intensity) as u8,
                (base.g as f32 * intensity) as u8,
                (base.b as f32 * intensity) as u8,
                255,
            )
        }
        None => Color::BLACK,
    }
}

// el raytracer no es mas que un for de dos dimensiones que recorre la pantalla
pub fn render(framebuffer: &mut Framebuffer, objects: &[Object], light: &Light) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            // mapea el pixel a espacio de pantalla [-1, 1]
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            // ajuste por aspect ratio
            let screen_x = screen_x * aspect_ratio;

            // direccion del rayo para este pixel
            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));

            // dispara el rayo y obtiene el color del pixel
            let pixel_color = cast_ray(&Vec3::new(0.0, 0.0, 0.0), &ray_direction, objects, light);

            // pinta el pixel en pantalla
            framebuffer.set_current_color(pixel_color);
            framebuffer.point(x, y);
        }
    }
}
