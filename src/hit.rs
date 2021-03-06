use std::sync::Arc;

use crate::material::Scatter;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    /// The point of intersection
    pub point: Point3,
    /// The normal of the surface at the point of intersection
    pub normal: Vec3,
    /// The material of the object that was hit
    pub material: Arc<dyn Scatter>,
    /// The distance from the ray's origin to the hit point
    pub t: f64,
    /// Whether or not the ray was inside the object
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        point: Point3,
        material: Arc<dyn Scatter>,
        t: f64,
        ray: &Ray,
        outward_normal: Vec3,
    ) -> HitRecord {
        let (front_face, normal) = Self::determine_face_normal(ray, outward_normal);
        HitRecord {
            point,
            material,
            normal,
            t,
            front_face,
        }
    }

    pub fn determine_face_normal(r: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
        let front_face = r.direction().dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            (-1.0) * outward_normal
        };

        return (front_face, normal);
    }
}

pub trait Hit: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct World(Vec<Box<dyn Hit>>);

impl World {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, element: Box<dyn Hit>) {
        self.0.push(element);
    }
}

impl Hit for World {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut t_closest = t_max;

        for object in &self.0 {
            if let Some(rec) = object.hit(r, t_min, t_closest) {
                t_closest = rec.t;
                hit_record = Some(rec);
            }
        }

        hit_record
    }
}
