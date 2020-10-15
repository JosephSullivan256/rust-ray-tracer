use crate::math::*;
use crate::material::Material;
use crate::hittables::hittable::Hittable;

const EPSILON: f32 = 0.001;

pub struct Plane {
    point: Vec3,
    normal: Vec3,
    material: Material
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3, material: Material) -> Plane {
        Plane{point, normal: normal.normalize(), material}
    }
}

impl Hittable for Plane {
    fn get_material(&self) -> Material {
        self.material
    }
    fn intersect(&self, r: Ray) -> Option<Vec3> {
        let denom = r.direction.dot(&self.normal);
        if denom.abs() < EPSILON {
            return None;
        }
        let t = (self.point-r.origin).dot(&self.normal)/denom;
        if t>=0. {
            return Some(r.eval(t));
        }
        None
    }
    fn get_normal(&self, _: Vec3) -> Vec3 {
        self.normal
    }
}