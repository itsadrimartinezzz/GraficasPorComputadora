use crate::material::Material;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::vec3::{normalize, Vec3};

#[derive(Clone, Copy)]
pub struct Cube {
    pub center: Vec3,
    pub size: f32, // largo de la arista
    pub material: Material,
    pub rotation_y: f32, // giro horizontal (yaw) - flechas izquierda/derecha
    pub rotation_x: f32, // giro vertical (pitch) - flechas arriba/abajo
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect> {
        // el cubo vive axis-aligned en su propio espacio local; para poder girarlo,
        // el rayo se pasa a ese espacio deshaciendo la rotacion (en vez de rotar la caja)
        let local_origin = self.to_object_space(*ray_origin - self.center);
        let local_direction = self.to_object_space(*ray_direction);

        let half = self.size * 0.5;
        let min = Vec3::new(-half, -half, -half);
        let max = Vec3::new(half, half, half);

        // interseccion rayo-caja por el metodo de "slabs"
        let mut t_min = f32::NEG_INFINITY;
        let mut t_max = f32::INFINITY;

        for axis in 0..3 {
            let (origin, dir, lo, hi) = match axis {
                0 => (local_origin.x, local_direction.x, min.x, max.x),
                1 => (local_origin.y, local_direction.y, min.y, max.y),
                _ => (local_origin.z, local_direction.z, min.z, max.z),
            };

            if dir.abs() < 1e-8 {
                if origin < lo || origin > hi {
                    return None;
                }
            } else {
                let inv = 1.0 / dir;
                let mut t0 = (lo - origin) * inv;
                let mut t1 = (hi - origin) * inv;
                if t0 > t1 {
                    std::mem::swap(&mut t0, &mut t1);
                }
                t_min = t_min.max(t0);
                t_max = t_max.min(t1);
                if t_min > t_max {
                    return None;
                }
            }
        }

        let t = if t_min > 0.001 {
            t_min
        } else if t_max > 0.001 {
            t_max
        } else {
            return None;
        };

        let local_point = local_origin + local_direction * t;
        let world_point = *ray_origin + *ray_direction * t;

        let local_normal = face_normal(&local_point, &min, &max);
        let normal = normalize(&self.to_world_space(local_normal));

        Some(Intersect {
            distance: t,
            point: world_point,
            normal,
            material: self.material,
        })
    }
}

impl Cube {
    // mundo -> local: deshace primero el yaw y luego el pitch
    fn to_object_space(&self, v: Vec3) -> Vec3 {
        let cy = self.rotation_y.cos();
        let sy = self.rotation_y.sin();
        let x1 = v.x * cy - v.z * sy;
        let z1 = v.x * sy + v.z * cy;
        let y1 = v.y;

        let cx = self.rotation_x.cos();
        let sx = self.rotation_x.sin();
        let y2 = y1 * cx + z1 * sx;
        let z2 = -y1 * sx + z1 * cx;

        Vec3::new(x1, y2, z2)
    }

    // local -> mundo: aplica primero el pitch y luego el yaw (inverso del anterior)
    fn to_world_space(&self, v: Vec3) -> Vec3 {
        let cx = self.rotation_x.cos();
        let sx = self.rotation_x.sin();
        let y1 = v.y * cx - v.z * sx;
        let z1 = v.y * sx + v.z * cx;

        let cy = self.rotation_y.cos();
        let sy = self.rotation_y.sin();
        let x2 = v.x * cy + z1 * sy;
        let z2 = -v.x * sy + z1 * cy;

        Vec3::new(x2, y1, z2)
    }
}

// decide a cual de las 6 caras locales pertenece el punto de impacto
fn face_normal(point: &Vec3, min: &Vec3, max: &Vec3) -> Vec3 {
    let epsilon = 0.0005;

    if (point.x - min.x).abs() < epsilon {
        return Vec3::new(-1.0, 0.0, 0.0);
    }
    if (point.x - max.x).abs() < epsilon {
        return Vec3::new(1.0, 0.0, 0.0);
    }
    if (point.y - min.y).abs() < epsilon {
        return Vec3::new(0.0, -1.0, 0.0);
    }
    if (point.y - max.y).abs() < epsilon {
        return Vec3::new(0.0, 1.0, 0.0);
    }
    if (point.z - min.z).abs() < epsilon {
        return Vec3::new(0.0, 0.0, -1.0);
    }
    Vec3::new(0.0, 0.0, 1.0)
}
