use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: Point3, t: f64, ray: &Ray, outward_normal: Vec3) -> HitRecord {
        let (front_face, normal) = Self::determine_face_normal(ray, outward_normal);
        HitRecord {
            point,
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

pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
