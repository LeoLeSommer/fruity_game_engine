use crate::math::matrix3::Matrix3;
use crate::math::matrix4::Matrix4;
use fruity_ecs::deserialize::Deserialize;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::export;
use fruity_game_engine::export_constructor;
use fruity_game_engine::export_impl;
use fruity_game_engine::export_struct;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;

/// A vector in 2D dimension
#[repr(C)]
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    FruityAny,
    Deserialize,
    bytemuck::Pod,
    bytemuck::Zeroable,
)]
#[export_struct]
pub struct Vector2D {
    /// Horizontal component
    pub x: f32,

    /// Vertical component
    pub y: f32,
}

#[export_impl]
impl Vector2D {
    /// Create a new `Vector2D` with the provided components.
    #[export_constructor]
    pub fn new(x: f32, y: f32) -> Vector2D {
        Self { x, y }
    }

    /// Returns a vector with only the horizontal component of the current one
    ///
    /// # Example
    /// ```
    /// use vector2d::Vector2D;
    /// let v = Vector2D::new(10, 20);
    /// assert_eq!(Vector2D::new(10, 0), v.horizontal());
    /// ```
    #[export]
    pub fn horizontal(&self) -> Self {
        Self {
            x: self.x,
            y: Default::default(),
        }
    }

    /// Returns a vector with only the vertical component of the current one
    ///
    /// # Example
    /// ```
    /// use vector2d::Vector2D;
    /// let v = Vector2D::new(10, 20);
    /// assert_eq!(Vector2D::new(0, 20), v.vertical());
    #[export]
    pub fn vertical(&self) -> Self {
        Self {
            x: Default::default(),
            y: self.y,
        }
    }

    /// Get the absolute value of the vector
    #[export]
    pub fn abs(&self) -> Vector2D {
        Vector2D::new(self.x.abs(), self.y.abs())
    }

    /// Returns a vector perpendicular to the current one.
    ///
    /// # Example
    /// ```
    /// use vector2d::Vector2D;
    /// let v = Vector2D::new(21.3, -98.1);
    /// assert_eq!(Vector2D::new(98.1, 21.3), v.normal());
    /// ```
    #[export]
    pub fn normal(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    /// Get the scalar/dot product of the two `Vector2D`.
    #[export]
    pub fn dot(&self, v2: Self) -> f32 {
        self.x * v2.x + self.y * v2.y
    }

    /// Get the squared length of a `Vector2D`. This is more performant than using
    /// `length()` -- which is only available for `Vector2D<f32>` and `Vector2D<f64>`
    /// -- as it does not perform any square root operation.
    #[export]
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// Linearly interpolates between two vectors
    #[export]
    pub fn lerp(&self, end: Self, progress: f32) -> Self {
        *self + ((end - *self) * progress)
    }

    /// Get the length of the vector. If possible, favour `length_squared()` over
    /// this function, as it is more performant.
    #[export]
    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    /// Get a new vector with the same direction as this vector, but with a length
    /// of 1.0. If the the length of the vector is 0, then the original vector is
    /// returned.
    #[export]
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len == 0.0 {
            *self
        } else {
            *self / len
        }
    }

    /// Get the vector's direction in radians.
    #[export]
    pub fn angle(&self) -> f32 {
        self.y.atan2(self.x)
    }

    /// Check if the point is in a triangle
    #[export]
    pub fn in_triangle(&self, p1: Vector2D, p2: Vector2D, p3: Vector2D) -> bool {
        pub fn sign(p1: &Vector2D, p2: &Vector2D, p3: &Vector2D) -> f32 {
            (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y)
        }

        let d1 = sign(self, &p1, &p2);
        let d2 = sign(self, &p2, &p3);
        let d3 = sign(self, &p3, &p1);

        let has_neg = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
        let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);

        return !(has_neg && has_pos);
    }

    /// Check if the point is in a circle
    #[export]
    pub fn in_circle(&self, center: Vector2D, radius: f32) -> bool {
        return (*self - center).length() <= radius;
    }

    /// Add two vectors
    #[export]
    pub fn add(&self, rhs: Vector2D) -> Vector2D {
        return *self + rhs;
    }

    /// Subtract two vectors
    #[export]
    pub fn sub(&self, rhs: Vector2D) -> Vector2D {
        return *self + rhs;
    }

    /// Multiply a vector by a number
    #[export]
    pub fn mul(&self, rhs: f32) -> Vector2D {
        return *self * rhs;
    }

    /// Divide a vector by a number
    #[export]
    pub fn div(&self, rhs: f32) -> Vector2D {
        return *self / rhs;
    }
}

// Ops Implementations
impl Add<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Vector2D) -> Self::Output {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Vector2D> for Vector2D {
    fn add_assign(&mut self, rhs: Vector2D) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl Sub<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Vector2D) -> Self::Output {
        Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<Vector2D> for Vector2D {
    fn sub_assign(&mut self, rhs: Vector2D) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl Mul<f32> for Vector2D {
    type Output = Vector2D;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Vector2D> for Matrix3 {
    type Output = Vector2D;

    fn mul(self, rhs: Vector2D) -> Self::Output {
        let cgmath_vec = cgmath::Vector3::new(rhs.x, rhs.y, 1.0);
        let cgmath_matrix = cgmath::Matrix3::from(self.0);
        let cgmath_result = cgmath_matrix * cgmath_vec;

        Vector2D {
            x: cgmath_result.x,
            y: cgmath_result.y,
        }
    }
}

impl Mul<Vector2D> for Matrix4 {
    type Output = Vector2D;

    fn mul(self, rhs: Vector2D) -> Self::Output {
        let cgmath_vec = cgmath::Vector4::new(rhs.x, rhs.y, 0.0, 1.0);
        let cgmath_matrix = cgmath::Matrix4::from(self.0);
        let cgmath_result = cgmath_matrix * cgmath_vec;

        Vector2D {
            x: cgmath_result.x,
            y: cgmath_result.y,
        }
    }
}

impl MulAssign<f32> for Vector2D {
    fn mul_assign(&mut self, rhs: f32) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl Div<f32> for Vector2D {
    type Output = Vector2D;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<f32> for Vector2D {
    fn div_assign(&mut self, rhs: f32) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}
