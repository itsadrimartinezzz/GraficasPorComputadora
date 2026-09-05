mod cube;
mod framebuffer;
mod light;
mod material;
mod ray_intersect;
mod raytracer;
mod vec3;

use raylib::prelude::*;

use cube::Cube;
use framebuffer::Framebuffer;
use light::Light;
use material::Material;
use ray_intersect::Object;
use raytracer::render;
use vec3::Vec3;

fn main() {
    let width = 1280;
    let height = 720;

    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Raytracer - Cubo Rosa Pastel")
        .build();

    let mut framebuffer = Framebuffer::new(width, height, Color::BLACK);

    let material = Material::new(Color::new(255, 182, 213, 255)); // rosa pastel, igual que la esfera

    let mut cube = Cube {
        center: Vec3::new(0.0, 0.0, -5.0),
        size: 2.5,
        material,
        rotation_y: 0.0,
        rotation_x: 0.0,
    };

    let light = Light {
        position: Vec3::new(3.0, 3.0, 2.0),
        intensity: 1.0,
    };

    // el raytracing es costoso, se calcula una sola vez y luego solo si algo se mueve
    render(&mut framebuffer, &[Object::Cube(cube)], &light);

    let move_speed = 2.5; // unidades por segundo

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        let mut moved = false;

        // el cubo se traslada de verdad en el mundo (no la camara), la camara sigue fija en el origen
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            cube.center.x += move_speed * dt;
            moved = true;
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            cube.center.x -= move_speed * dt;
            moved = true;
        }
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            cube.center.y += move_speed * dt;
            moved = true;
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            cube.center.y -= move_speed * dt;
            moved = true;
        }

        if moved {
            render(&mut framebuffer, &[Object::Cube(cube)], &light);
        }

        framebuffer.swap_buffers(&mut rl, &thread);
    }
}
