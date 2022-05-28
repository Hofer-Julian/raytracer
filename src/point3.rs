use std::{
    fmt::{self, Display},
    ops::{Add, Index, IndexMut, Sub},
};

use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Point3 {
    e: [f64; 3],
}

impl Point3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Point3 {
        Point3 { e: [e0, e1, e2] }
    }
    pub fn x(self) -> f64 {
        self[0]
    }

    pub fn y(self) -> f64 {
        self[1]
    }

    pub fn z(self) -> f64 {
        self[2]
    }
}

impl Display for Point3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self[0], self[1], self[2])
    }
}

impl Index<usize> for Point3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}

impl IndexMut<usize> for Point3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.e[index]
    }
}

impl Add<Vec3> for Point3 {
    type Output = Point3;

    fn add(self, other: Vec3) -> Point3 {
        Point3 {
            e: [self[0] + other[0], self[1] + other[1], self[2] + other[2]],
        }
    }
}

impl Sub<Vec3> for Point3 {
    type Output = Point3;

    fn sub(self, other: Vec3) -> Point3 {
        Point3 {
            e: [self[0] - other[0], self[1] - other[1], self[2] - other[2]],
        }
    }
}

impl Sub<Point3> for Point3 {
    type Output = Vec3;

    fn sub(self, other: Point3) -> Vec3 {
        Vec3::new(self[0] - other[0], self[1] - other[1], self[2] - other[2])
    }
}
