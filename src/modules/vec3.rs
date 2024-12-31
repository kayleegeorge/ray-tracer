/*
 * Vec3.rs
 */

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use super::utils::{random_double, random_double_range};

// Note: Define the Vec3 struct (what it is)
// Note: Structs create instances of data vs. traits define shared behavior (polymorphism)
#[derive(Clone, Copy, Default)]
pub struct Vec3 {
    e: [f64; 3]
}

// Note: Implements the Vec3 struct with methods (what is does)
impl Vec3 {
    // Zero constructor (initializes to 0, 0, 0)
    pub fn zero() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    // Default Constructor from 3 arguments
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    // Getters
    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    // Cross product 
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 { e: [self.e[1] * other.e[2] - self.e[2] * other.e[1], 
                self.e[2] * other.e[0] - self.e[0] * other.e[2], 
                self.e[0] * other.e[1] - self.e[1] * other.e[0]] }
    }

    // Unit vector
    pub fn unit_vector(&self) -> Vec3 {
        self.clone() / self.length()
    }

    // Dot product
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.e[0] * other.e[0] + 
        self.e[1] * other.e[1] + 
        self.e[2] * other.e[2]
    }

    // Random
    pub fn random(&self) -> Vec3 {
        Vec3 { e: [random_double(), random_double(), random_double()] }
    }

    // Random in [min, max]
    pub fn random_in_range(&self, min: f64, max: f64) -> Vec3 {
        Vec3 { e: [random_double_range(min, max), random_double_range(min, max), random_double_range(min, max)] }
    }

    /*
     * Random unit vector
     * 
     * Rejection method:
     * 1. Generate random vector in [-1, 1] (in a unit sphere)
     * 2. If the point lies outside the unit sphere or in "black hole" (1e-160 < len_sq <= 1.0), reject it and try again
     * 3. If the point lies inside the unit sphere, return the unit vector
     */
    pub fn random_unit_vector(&self) -> Vec3 {
        loop {
            let p = self.random_in_range(-1.0, 1.0);
            let len_sq = p.length_squared();
            if (1e-160 < len_sq) && (len_sq <= 1.0) {
                return p / len_sq.sqrt();
            }
        }
    }

    /*
     * Random vector on a hemisphere
     * 
     * If the dot product of the random vector and the normal is positive, vector is in the correct hemisphere
     * Otherwise, vector is in the opposite hemisphere & need to invert
     */
    pub fn random_on_hemisphere(&self, normal: Vec3) -> Vec3 {
        let on_unit_sphere = self.random_unit_vector();
        if on_unit_sphere.dot(&normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
}

pub type Point3 = Vec3; // 3D point

// ----- OPERATORS -----

// Negation (-)
impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3 { e: [-self.e[0], -self.e[1], -self.e[2]] }
    }
}

// += 
impl AddAssign for Vec3 {
    // Note: This is more efficient than doing (*self = *self + other)
    //       because it avoids the creation of a temporary Vec3 object
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

// *= 
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, scalar: f64) {
        self.e[0] *= scalar;
        self.e[1] *= scalar;
        self.e[2] *= scalar;
    }
}

// /= 
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, scalar: f64) {
        *self *= 1.0 / scalar;
    }
}

// Vector Utility Functions
// Note: In Rust implement operators for owned values and let the compiler automatically deref 

// <<
impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

// + 
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]] }
    }
}

// -
impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 { e: [self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]] }
    }
}

// Vec3 * Vec3
impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 { e: [self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2]] }
    }
}

// f64 * Vec3
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3 { e: [self * vec.e[0], self * vec.e[1], self * vec.e[2]] }
    }
}

// Vec3 * f64
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, scalar: f64) -> Vec3 {
        scalar * self
    }
}

// / 
impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, scalar: f64) -> Vec3 {
        (1.0 / scalar) * self 
    }
}

// ----- END OPERATORS -----
