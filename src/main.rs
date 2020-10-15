mod math;
use math::*;

mod camera;
use camera::Camera;

mod light;
use light::Light;

mod material;
use material::Material;

mod scene;
use scene::Scene;

mod hittables;
use hittables::sphere::Sphere;
use hittables::plane::Plane;

const N: u32 = 20;
const DEPTH: u32 = 8;

fn main() {
    println!("ray-tracer");

    let mut buffer : image::RgbImage = image::ImageBuffer::new(2048, 2048);
    //let light = Light::new(1.0, 1.0, 1.0);
    //buffer.put_pixel(20, 20, light.toRGB());

    let camera = Camera::new(
        Ray::new(Vec3::new(0., -0.5, 0.), Vec3::new(0., 0.2, 1.)),
        std::f32::consts::PI/2.0
    );

    let mut scene = Scene::new();
    scene.push(Sphere::new(Vec3::new(1.1, 0., 3.), 1.0, Material::new(1., Light::new(0.4,0.8,0.4))));
    scene.push(Sphere::new(Vec3::new(-1.1, 0., 3.), 1.0, Material::new(0.4, Light::new(0.8,0.4,0.4))));
    scene.push(Plane::new(Vec3::new(0., -1., 0.), Vec3::new(0., 1., 0.), Material::new (1., Light::new(0.4,0.4,0.8))));

    camera.render_image(&mut buffer, &scene, DEPTH, N);

    buffer.save("test.png").unwrap();
}
