use super::super::angles::Angle;

use num::{One, Zero};
use std::{
    fmt::{Debug, Display, Formatter, Result},
    ops::{Add, Div, Mul, Neg, Sub},
};

pub struct Vector3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Default> Default for Vector3D<T> {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
        }
    }
}

impl<T> Vector3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn from_array(array: [T; 3]) -> Self
    where
        T: Copy,
    {
        Self {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }

    pub fn to_array(&self) -> [T; 3]
    where
        T: Copy,
    {
        [self.x, self.y, self.z]
    }

    pub fn magnitude_squared(&self) -> T
    where
        T: Copy + Mul<Output = T> + Add<Output = T>,
    {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn magnitude(&self) -> f64
    where
        T: Copy + Into<f64> + Mul<Output = T> + Add<Output = T>,
    {
        self.magnitude_squared().into().sqrt()
    }

    pub fn normalize(&self) -> Self
    where
        T: Copy + Into<f64> + From<f64> + Div<Output = T> + Add<Output = T> + Mul<Output = T>,
    {
        let mag: f64 = self.magnitude();
        Self {
            x: (self.x.into() / mag).into(),
            y: (self.y.into() / mag).into(),
            z: (self.z.into() / mag).into(),
        }
    }

    pub fn dot(&self, other: &Self) -> T
    where
        T: Copy + Mul<Output = T> + Add<Output = T>,
    {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self
    where
        T: Copy + Mul<Output = T> + Sub<Output = T>,
    {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn pow2(&self) -> T
    where
        T: Copy + Mul<Output = T> + Add<Output = T>,
    {
        self.magnitude_squared()
    }

    pub fn on(&self, other: &Self) -> T
    where
        T: Copy + Into<f64> + From<f64> + Mul<Output = T> + Div<Output = T> + Add<Output = T>,
    {
        self.dot(&other.normalize())
    }

    pub fn angle(&self, other: &Self) -> Angle
    where
        T: Copy + Into<f64> + Mul<Output = T> + Div<Output = T> + Add<Output = T>,
    {
        let dot_product = self.dot(other).into();
        let magnitude_product = (self.magnitude_squared() * other.magnitude_squared())
            .into()
            .sqrt();
        Angle::new_rad(dot_product.acos() / magnitude_product)
    }
}

impl<T> Add for Vector3D<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> Sub for Vector3D<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> Mul<T> for Vector3D<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl<T> Div<T> for Vector3D<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, scalar: T) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl<T> Debug for Vector3D<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({:?}, {:?}, {:?})", self.x, self.y, self.z)
    }
}

impl<T> Display for Vector3D<T>
where
    T: Display + Zero + One + Neg<Output = T> + Copy + PartialEq<T>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", pretty_print(self.x, self.y, self.z))
    }
}

#[inline]
fn pretty_print<T>(x: T, y: T, z: T) -> String
where
    T: Display + Zero + One + Neg<Output = T> + Copy + PartialEq<T>,
{
    let fx: String = if x == T::one() {
        "i".to_string()
    } else if x == -T::one() {
        "-i".to_string()
    } else if x != T::zero() {
        format!("{}i", x)
    } else {
        "".to_string()
    };

    let fy: String = if y == T::one() {
        if x != T::zero() {
            "+j".to_string()
        } else {
            "j".to_string()
        }
    } else if y == -T::one() {
        "-j".to_string()
    } else if y != T::zero() {
        format!("{:+}j", y)
    } else {
        "".to_string()
    };

    let fz: String = if z == T::one() {
        if x != T::zero() || y != T::zero() {
            "+k".to_string()
        } else {
            "k".to_string()
        }
    } else if z == -T::one() {
        "-k".to_string()
    } else if z != T::zero() {
        format!("{:+}k", z)
    } else {
        "".to_string()
    };

    fx + &fy + &fz
}
