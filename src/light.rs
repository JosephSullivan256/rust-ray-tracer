use nalgebra::Vector3;
pub type Vec3 = Vector3<f32>;


#[derive(Debug, Copy, Clone)]
pub struct Light {
    pub light: Vec3,
}

impl Light {
    pub fn new(r:f32,g:f32,b:f32) -> Light {
        Light{ light: Vec3::new(r,g,b) }
    }

    pub fn toRGB(&self) -> image::Rgb<u8> {
        image::Rgb([(self.light.x*255.0) as u8, (self.light.y*255.0) as u8, (self.light.z*255.0) as u8])
    }
    pub fn toRGBA(&self) -> image::Rgba<u8> {
        image::Rgba([(self.light.x*255.0) as u8, (self.light.y*255.0) as u8, (self.light.z*255.0) as u8, 255])
    }
}

impl std::ops::Add<&Light> for &Light {
    type Output = Light;

    fn add(self, l2: &Light) -> Light {
        Light {light: self.light+l2.light}
    }
}

impl std::ops::Mul<&Light> for f32 {
    type Output = Light;

    fn mul(self, l2: &Light) -> Light {
        Light {light: self*l2.light}
    }
}

impl std::ops::Mul<&Light> for &Light {
    type Output = Light;

    fn mul(self, l2: &Light) -> Light {
        Light {light: self.light.component_mul(&l2.light)}
    }
}