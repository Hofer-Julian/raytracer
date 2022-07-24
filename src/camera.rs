use std::f64::consts::PI;

use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        v_up: Vec3,
        v_fov: f64,
        aspect_ratio: f64,
    ) -> Self {
        // Vertical field-of-view in degrees
        let theta = PI / 180.0 * v_fov;
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let cw = (look_from - look_at).normalized();
        let cu = v_up.cross(cw).normalized();
        let cv = cw.cross(cu);

        let horizontal = viewport_width * cu;
        let vertical = viewport_height * cv;

        let lower_left_corner = look_from - horizontal / 2.0 - vertical / 2.0 - cw;

        Self {
            origin: look_from,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
