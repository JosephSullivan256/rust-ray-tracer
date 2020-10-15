pub use nalgebra::Vector3;
pub type Vec3 = Vector3<f32>;

pub use nalgebra::Matrix3;
pub type Mat3 = Matrix3<f32>;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray{origin, direction:direction.normalize()}
    }
    pub fn eval(&self, t: f32) -> Vec3 {
        self.origin+t*self.direction
    }
}
