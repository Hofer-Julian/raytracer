use std::{
    fmt::{self, Display},
    ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign},
};

#[derive(Clone, Copy)]
pub struct Color {
    e: [f64; 3],
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { e: [r, g, b] }
    }

    pub fn format_color(self) -> String {
        let red = (256 as f64 * self[0]) as u64;
        let green = (256 as f64 * self[1]) as u64;
        let blue = (256 as f64 * self[2]) as u64;

        format!("{red} {green} {blue}")
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(R{}, G{}, B{})", self[0], self[1], self[2])
    }
}

impl Index<usize> for Color {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}

impl IndexMut<usize> for Color {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.e[index]
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            e: [self[0] + other[0], self[1] + other[1], self[2] + other[2]],
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        *self = Color {
            e: [self[0] + other[0], self[1] + other[1], self[2] + other[2]],
        };
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Color {
        Color {
            e: [self[0] * other, self[1] * other, self[2] * other],
        }
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, other: f64) -> () {
        *self = Color {
            e: [self[0] * other, self[1] * other, self[2] * other],
        };
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            e: [self * other[0], self * other[1], self * other[2]],
        }
    }
}
