use crate::math::*;
use crate::scene::Scene;
use crate::light::Light;

pub struct Camera {
    ray: Ray,
    fov: f32,
}

impl Camera {
    pub fn new(ray: Ray, fov: f32) -> Camera {
        Camera{ray, fov}
    }

    pub fn get_transform(&self) -> Mat3 {
        let j : Vec3 = nalgebra::Vector3::y();
        let mut right = self.ray.direction.cross(&j);
        let mut up = right.cross(&self.ray.direction);
        
        right = right.normalize();
        up = up.normalize();

        Matrix3::from_columns(&[right, up, self.ray.direction])
    }

    pub fn render_image(&self, buffer: &mut image::RgbImage, scene: &Scene, depth: u32, N: u32){
        let w = buffer.width();
        let h = buffer.height();
        let a = (w as f32)/(h as f32);

        let transform = self.get_transform();

        for x in 0..w {
            print!("{}\n",x);
            let nx = (2.*x as f32)/(w as f32)-1.;
            for y in 0..h {
                let ny = 1.0-(2.0*y as f32)/(h as f32);
                let f_scale = (self.fov/2.).tan();
                let direction = transform*Vec3::new(a*f_scale*nx, f_scale*ny, 1.0);

                let light = (0..N).map(|_i| scene.shoot_ray(
                    Ray::new(self.ray.origin, direction+Camera::get_offset(f_scale, a, w, h)), 
                    depth))
                    .fold(Light::new(0.,0.,0.), |l1,l2|&l1+&((1./N as f32)*&l2));
                buffer.put_pixel(x, y, light.toRGB());
            }
        }
    }

    fn get_offset(f_scale: f32, a: f32, w: u32, h: u32) -> Vec3{
        return Vec3::new(
            (0.5+ rand::random::<f32>())*a*2./(w as f32)*f_scale, 
            (0.5+ rand::random::<f32>())*2./(h as f32)*f_scale, 0.
        )
    }
}