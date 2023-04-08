use crate::{math::vector2d::Vector2D, Matrix4};
use cgmath::{Angle, Rad, SquareMatrix};
use fruity_ecs::serialization::{Deserialize, Serialize};
use fruity_game_engine::{any::FruityAny, export, export_impl, export_struct};
use std::ops::Mul;

#[derive(Debug, FruityAny, Serialize, Deserialize, Clone, Copy)]
#[export_struct]
pub struct Matrix3(pub [[f32; 3]; 3]);

#[export_impl]
impl Matrix3 {
    pub fn new_identity() -> Matrix3 {
        Matrix3(cgmath::Matrix3::identity().into())
    }

    pub fn new_translation(vec: Vector2D) -> Matrix3 {
        Matrix3(
            cgmath::Matrix3::from_translation(cgmath::Vector2::<f32> { x: vec.x, y: vec.y }).into(),
        )
    }

    pub fn new_rotation(angle: f32) -> Matrix3 {
        let (s, c) = Rad::sin_cos(Rad(angle));
        Matrix3([[c, s, 0.0], [-s, c, 0.0], [0.0, 0.0, 1.0]])
    }

    pub fn new_scaling(vec: Vector2D) -> Matrix3 {
        Matrix3(cgmath::Matrix3::from_nonuniform_scale(vec.x, vec.y).into())
    }

    #[export]
    pub fn translation(&self) -> Vector2D {
        Vector2D::new(self.0[2][0], self.0[2][1])
    }

    #[export]
    pub fn rotation(&self) -> f32 {
        f32::atan(self.0[0][1] / self.0[0][0])
    }

    #[export]
    pub fn scale(&self) -> Vector2D {
        // TODO: Take in care negative scaling
        Vector2D::new(
            f32::sqrt(self.0[0][0].powf(2.0) + self.0[0][1].powf(2.0)),
            f32::sqrt(self.0[1][0].powf(2.0) + self.0[1][1].powf(2.0)),
        )
    }

    #[export]
    pub fn invert(&self) -> Matrix3 {
        if let Some(result) = cgmath::Matrix3::from(self.0).invert() {
            Matrix3(result.into())
        } else {
            Matrix3::new_identity()
        }
    }
}

impl Into<[[f32; 3]; 3]> for Matrix3 {
    fn into(self) -> [[f32; 3]; 3] {
        self.0
    }
}

impl Into<Matrix4> for Matrix3 {
    fn into(self) -> Matrix4 {
        Matrix4([
            [self.0[0][0], self.0[0][1], 0.0, self.0[0][2]],
            [self.0[1][0], self.0[1][1], 0.0, self.0[1][2]],
            [0.0, 0.0, 1.0, 0.0],
            [self.0[2][0], self.0[2][1], 0.0, self.0[2][2]],
        ])
    }
}

impl Default for Matrix3 {
    fn default() -> Self {
        Matrix3::new_identity()
    }
}

impl Mul<Matrix3> for Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: Matrix3) -> Self::Output {
        let result = cgmath::Matrix3::from(self.0) * cgmath::Matrix3::from(rhs.0);
        Matrix3(result.into())
    }
}

#[cfg(test)]
mod matrix3 {
    use crate::Matrix3;
    use crate::Vector2D;
    use std::f32::consts::PI;

    #[test]
    fn translation() {
        assert_eq!(
            (Matrix3::new_translation(Vector2D::new(1.0, 1.0))
                * Matrix3::new_rotation(PI / 5.0)
                * Matrix3::new_scaling(Vector2D::new(-1.4, 2.3)))
            .translation(),
            Vector2D::new(1.0, 1.0)
        );
        assert_eq!(
            (Matrix3::new_translation(Vector2D::new(1.4, -2.3))
                * Matrix3::new_rotation(PI / 5.0)
                * Matrix3::new_scaling(Vector2D::new(-1.4, 2.3)))
            .translation(),
            Vector2D::new(1.4, -2.3)
        );
        assert_eq!(
            (Matrix3::new_translation(Vector2D::new(0.0, 0.0))
                * Matrix3::new_rotation(PI / 5.0)
                * Matrix3::new_scaling(Vector2D::new(-1.4, 2.3)))
            .translation(),
            Vector2D::new(0.0, 0.0)
        );
        assert_eq!(
            (Matrix3::new_translation(Vector2D::new(13.2, 0.3))
                * Matrix3::new_rotation(PI / 5.0)
                * Matrix3::new_scaling(Vector2D::new(-1.4, 2.3)))
            .translation(),
            Vector2D::new(13.2, 0.3)
        );
    }

    #[test]
    fn rotation() {
        assert_eq!(
            (Matrix3::new_translation(Vector2D::new(13.2, 0.3))
                * Matrix3::new_rotation(0.0)
                * Matrix3::new_scaling(Vector2D::new(1.0, 1.0)))
            .rotation(),
            0.0
        );
        assert_eq!(
            (Matrix3::new_translation(Vector2D::new(13.2, 0.3))
                * Matrix3::new_rotation(PI / 5.0)
                * Matrix3::new_scaling(Vector2D::new(1.0, 1.0)))
            .rotation(),
            PI / 5.0
        );
        assert_eq!(
            (Matrix3::new_translation(Vector2D::new(13.2, 0.3))
                * Matrix3::new_rotation(-PI / 5.0)
                * Matrix3::new_scaling(Vector2D::new(1.0, 1.0)))
            .rotation(),
            -PI / 5.0
        );
        assert_eq!(
            (Matrix3::new_translation(Vector2D::new(13.2, 0.3))
                * Matrix3::new_rotation(2.0 * PI + PI / 5.0)
                * Matrix3::new_scaling(Vector2D::new(1.0, 1.0)))
            .rotation(),
            PI / 5.0
        );
        assert_eq!(
            (Matrix3::new_translation(Vector2D::new(13.2, 0.3))
                * Matrix3::new_rotation(-2.0 * PI - PI / 5.0)
                * Matrix3::new_scaling(Vector2D::new(1.0, 1.0)))
            .rotation(),
            -PI / 5.0
        );
    }

    #[test]
    fn scale() {
        assert_eq!(
            (Matrix3::new_translation(Vector2D::new(13.2, 0.3))
                * Matrix3::new_rotation(PI / 5.0)
                * Matrix3::new_scaling(Vector2D::new(1.0, 1.0)))
            .scale(),
            Vector2D::new(1.0, 1.0)
        );
        assert_eq!(
            (Matrix3::new_translation(Vector2D::new(13.2, 0.3))
                * Matrix3::new_rotation(PI / 5.0)
                * Matrix3::new_scaling(Vector2D::new(1.4, 2.3)))
            .scale(),
            Vector2D::new(1.4, 2.3)
        );
        assert_eq!(
            (Matrix3::new_translation(Vector2D::new(13.2, 0.3))
                * Matrix3::new_rotation(PI / 5.0)
                * Matrix3::new_scaling(Vector2D::new(1.4, -2.3)))
            .scale(),
            Vector2D::new(1.4, -2.3)
        );
        assert_eq!(
            (Matrix3::new_translation(Vector2D::new(13.2, 0.3))
                * Matrix3::new_rotation(PI / 5.0)
                * Matrix3::new_scaling(Vector2D::new(-1.4, 2.3)))
            .scale(),
            Vector2D::new(-1.4, 2.3)
        );
        assert_eq!(
            (Matrix3::new_translation(Vector2D::new(13.2, 0.3))
                * Matrix3::new_rotation(PI / 5.0)
                * Matrix3::new_scaling(Vector2D::new(-1.4, -2.3)))
            .scale(),
            Vector2D::new(-1.4, -2.3)
        );
        assert_eq!(
            (Matrix3::new_translation(Vector2D::new(13.2, 0.3))
                * Matrix3::new_rotation(PI / 5.0)
                * Matrix3::new_scaling(Vector2D::new(0.0, 0.0)))
            .scale(),
            Vector2D::new(0.0, 0.0)
        );
        assert_eq!(
            (Matrix3::new_translation(Vector2D::new(13.2, 0.3))
                * Matrix3::new_rotation(PI / 5.0)
                * Matrix3::new_scaling(Vector2D::new(13.2, 0.3)))
            .scale(),
            Vector2D::new(13.2, 0.3)
        );
    }
}
