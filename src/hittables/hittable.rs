use crate::math::*;
use crate::material::Material;
use crate::scene::Scene;
use crate::light::Light;

const EPSILON: f32 = 0.001;

pub trait Hittable {
    fn hit(&self, incoming: Ray, intersection: Vec3, scene: &Scene, depth: u32) -> Light {
        let direction = self.get_material().sample_direction(incoming, self.get_normal(intersection));
        &self.get_material().color * &scene.shoot_ray(Ray::new(intersection+EPSILON*direction, direction), depth)
    }
    fn get_material(&self) -> Material;
    fn intersect(&self, r: Ray) -> Option<Vec3>;
    fn get_normal(&self, i: Vec3) -> Vec3;
}
