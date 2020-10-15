use crate::math::*;
use crate::hittables::hittable::Hittable;
use crate::light::Light;

pub struct Scene {
    hittables: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene{hittables:vec![]}
    }

    pub fn push(&mut self, h: impl Hittable+'static){
        self.hittables.push(Box::new(h));
    }

    pub fn shoot_ray(&self, incoming: Ray, depth: u32) -> Light {
        if depth > 0 {
            let mut closest: Option<(Vec3,f32,&Box<dyn Hittable>)> = None;

            for h in self.hittables.iter() {
                if let Some(current_intersection) = h.intersect(incoming) {
                    let current_dist2 = (current_intersection-incoming.origin).norm_squared();

                    match closest {
                        Some((_old_intersection,old_dist2,_hit)) =>{
                            if current_dist2 < old_dist2 {
                                closest = Some((current_intersection,current_dist2,h));
                            }
                        },
                        None =>{
                            closest = Some((current_intersection,current_dist2,h));
                        }
                    }
                }
            }
        
            if let Some((intersection,_d2,hit)) = closest {
                return hit.hit(incoming,intersection,self,depth-1);
            } else {
                return Scene::sky(incoming);
            }
        }
        Light::new(0.,0.,0.)
    }

    fn sky(ray: Ray) -> Light {
        let t = (ray.direction.y+1.0)/2.0;
        &((1.-t)*&Light::new(1.,1.,1.))+&(t*&Light::new(0., 0.8, 1.))
    }
}