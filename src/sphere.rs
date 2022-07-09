use std::sync::Arc;

use crate::hit::{Hit, HitRecord};
use crate::material::Scatter;
use crate::point3::Point3;
use crate::ray::Ray;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Scatter>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Scatter>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length().powi(2);
        let half_b = oc.dot(ray.direction());
        let c = oc.length().powi(2) - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Find the nearest root that lies in the acceptable range
        let sqrtd = discriminant.sqrt();
        let mut t_hit = (-half_b - sqrtd) / a;
        if t_hit < t_min || t_max < t_hit {
            t_hit = (-half_b + sqrtd) / a;
            if t_hit < t_min || t_max < t_hit {
                return None;
            }
        }

        let point_hit = ray.at(t_hit);
        let outward_normal = (point_hit - self.center) / self.radius;
        let rec = HitRecord::new(
            point_hit,
            self.material.clone(),
            t_hit,
            &ray,
            outward_normal,
        );

        Some(rec)
    }
}
