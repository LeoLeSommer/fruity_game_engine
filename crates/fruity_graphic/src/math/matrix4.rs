use cgmath::SquareMatrix;
use fruity_ecs::deserialize_service::DeserializeFactory;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::{export, export_impl, export_struct};
use std::ops::Mul;

#[derive(Debug, FruityAny, DeserializeFactory, Clone, Copy)]
#[export_struct]
pub struct Matrix4(pub [[f32; 4]; 4]);

#[export_impl]
impl Matrix4 {
    pub fn identity() -> Matrix4 {
        Matrix4(cgmath::Matrix4::identity().into())
    }

    pub fn from_rect(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Matrix4 {
        Matrix4(cgmath::ortho(left, right, bottom, top, near, far).into())
    }

    #[export]
    pub fn invert(&self) -> Matrix4 {
        if let Some(result) = cgmath::Matrix4::from(self.0).invert() {
            Matrix4(result.into())
        } else {
            Matrix4::identity()
        }
    }
}

impl Into<[[f32; 4]; 4]> for Matrix4 {
    fn into(self) -> [[f32; 4]; 4] {
        self.0
    }
}

impl Default for Matrix4 {
    fn default() -> Self {
        Matrix4::identity()
    }
}

impl Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: Matrix4) -> Self::Output {
        let result = cgmath::Matrix4::from(self.0) * cgmath::Matrix4::from(rhs.0);
        Matrix4(result.into())
    }
}
