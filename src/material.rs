use crate::math::*;
use crate::light::Light;

#[derive(Debug, Copy, Clone)]
pub struct Material {
    diffusivity: f32,
    pub color: Light
}

impl Material {
    pub fn new(diffusivity: f32, color: Light) -> Material {
        Material{diffusivity, color}
    }
    pub fn sample_directions(&self, n: u32, incoming: Ray, normal: Vec3) -> Vec<Vec3> {
        let reflection = incoming.direction-2.*(incoming.direction.dot(&normal))*normal;
        let focus = self.diffusivity*normal + (1.-self.diffusivity)*reflection;
        (0..n).map(|_| focus+self.diffusivity*Material::sample_sphere()).collect()
    }
    pub fn sample_direction(&self, incoming: Ray, normal: Vec3) -> Vec3 {
        let reflection = incoming.direction-2.*(incoming.direction.dot(&normal))*normal;
        let focus = self.diffusivity*normal + (1.-self.diffusivity)*reflection;
        focus+self.diffusivity*Material::sample_sphere()
    }
    fn sample_sphere() -> Vec3 {
        let z = 2.*rand::random::<f32>()-1.;
        let r = (1.-z.powi(2)).sqrt();
        let a = 2.*std::f32::consts::PI*rand::random::<f32>();
        Vec3::new(r*a.cos(), r*a.sin(), z)
    }
}