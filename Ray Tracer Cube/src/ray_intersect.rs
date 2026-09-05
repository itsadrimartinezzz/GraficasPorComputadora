use crate::cube::Cube;
use crate::material::Material;
use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Intersect {
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect>;
}

// enum de objetos de la escena; hoy solo hay un cubo pero queda listo para mas figuras
pub enum Object {
    Cube(Cube),
}

impl RayIntersect for Object {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect> {
        match self {
            Object::Cube(c) => c.ray_intersect(ray_origin, ray_direction),
        }
    }
}
