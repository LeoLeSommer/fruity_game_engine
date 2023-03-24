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

/// A vector in 3D dimension
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
pub struct Vector3D {
    /// Horizontal component
    pub x: f32,

    /// Vertical component
    pub y: f32,

    /// Depth component
    pub z: f32,
}

#[export_impl]
impl Vector3D {
    /// Create a new `Vector3D` with the provided components.
    #[export_constructor]
    pub fn new(x: f32, y: f32, z: f32) -> Vector3D {
        Self { x, y, z }
    }

    /// Returns a vector with only the horizontal component of the current one
    ///
    /// # Example
    /// ```
    /// use vector3d::Vector3D;
    /// let v = Vector3D::new(10, 20, 40);
    /// assert_eq!(Vector3D::new(10, 0, 0), v.horizontal());
    /// ```
    #[export]
    pub fn horizontal(&self) -> Self {
        Self {
            x: self.x,
            y: Default::default(),
            z: Default::default(),
        }
    }

    /// Returns a vector with only the vertical component of the current one
    ///
    /// # Example
    /// ```
    /// use vector3d::Vector3D;
    /// let v = Vector3D::new(10, 20, 40);
    /// assert_eq!(Vector3D::new(0, 20, 0), v.vertical());
    #[export]
    pub fn vertical(&self) -> Self {
        Self {
            x: Default::default(),
            y: self.y,
            z: Default::default(),
        }
    }

    /// Returns a vector with only the depth component of the current one
    ///
    /// # Example
    /// ```
    /// use vector3d::Vector3D;
    /// let v = Vector3D::new(10, 20, 40);
    /// assert_eq!(Vector3D::new(0, 0, 40), v.depth());
    #[export]
    pub fn depth(&self) -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: self.z,
        }
    }

    /// Get the scalar/dot product of the two `Vector3D`.
    #[export]
    pub fn dot(&self, v2: Self) -> f32 {
        self.x * v2.x + self.y * v2.y + self.z * v2.z
    }

    /// Get the squared length of a `Vector3D`. This is more performant than using
    /// `length()` -- which is only available for `Vector3D<f32>` and `Vector3D<f64>`
    /// -- as it does not perform any square root operation.
    #[export]
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
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

    /// Add two vectors
    #[export]
    pub fn add(&self, rhs: Self) -> Self {
        return *self + rhs;
    }

    /// Subtract two vectors
    #[export]
    pub fn sub(&self, rhs: Self) -> Self {
        return *self + rhs;
    }

    /// Multiply a vector by a number
    #[export]
    pub fn mul(&self, rhs: f32) -> Self {
        return *self * rhs;
    }

    /// Divide a vector by a number
    #[export]
    pub fn div(&self, rhs: f32) -> Self {
        return *self / rhs;
    }
}

// Ops Implementations
impl Add<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: Vector3D) -> Self::Output {
        Vector3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<Vector3D> for Vector3D {
    fn add_assign(&mut self, rhs: Vector3D) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl Sub<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: Vector3D) -> Self::Output {
        Vector3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign<Vector3D> for Vector3D {
    fn sub_assign(&mut self, rhs: Vector3D) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

impl Mul<f32> for Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vector3D> for Matrix4 {
    type Output = Vector3D;

    fn mul(self, rhs: Vector3D) -> Self::Output {
        let cgmath_vec = cgmath::Vector4::new(rhs.x, rhs.y, rhs.z, 1.0);
        let cgmath_matrix = cgmath::Matrix4::from(self.0);
        let cgmath_result = cgmath_matrix * cgmath_vec;

        Vector3D {
            x: cgmath_result.x,
            y: cgmath_result.y,
            z: cgmath_result.z,
        }
    }
}

impl MulAssign<f32> for Vector3D {
    fn mul_assign(&mut self, rhs: f32) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

impl Div<f32> for Vector3D {
    type Output = Vector3D;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f32> for Vector3D {
    fn div_assign(&mut self, rhs: f32) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}
