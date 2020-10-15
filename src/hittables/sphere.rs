use crate::math::*;
use crate::material::Material;
use crate::hittables::hittable::Hittable;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Sphere{
        Sphere{center,radius,material}
    }
}

impl Hittable for Sphere {
    fn get_material(&self) -> Material {
        self.material
    }
    fn intersect(&self, r: Ray) -> Option<Vec3> {
        let t = (self.center-r.origin).dot(&r.direction);
        let min_dist2 = (r.eval(t)-self.center).norm_squared();
        if min_dist2 >= self.radius.powi(2) {
            return None
        }
        let offset = (self.radius.powi(2)-min_dist2).sqrt();

        let s = t-offset;
        if s < 0.0 {
            return None;
        }
        Some(r.eval(s))
    }
    fn get_normal(&self, i: Vec3) -> Vec3 {
        (i-self.center).normalize()
    }
}