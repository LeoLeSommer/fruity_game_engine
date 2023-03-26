use bytemuck::{Pod, Zeroable};
use css_color_parser::Color as CssColor;
use fruity_ecs::serializable::Serializable;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::{export_constructor, export_impl, export_struct};
use std::str::FromStr;

pub mod matrix3;
pub mod matrix4;
pub mod vector2d;
pub mod vector3d;

#[repr(C)]
#[derive(Debug, FruityAny, Serializable, Copy, Clone, Pod, Zeroable)]
#[export_struct]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[export_impl]
impl Color {
    #[export_constructor]
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Self { r, g, b, a }
    }

    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0, 1.0)
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }

    pub fn alpha() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn overlay() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.3)
    }

    pub fn red() -> Self {
        Self::new(1.0, 0.0, 0.0, 1.0)
    }

    pub fn green() -> Self {
        Self::new(0.0, 1.0, 0.0, 1.0)
    }

    pub fn blue() -> Self {
        Self::new(0.0, 0.0, 1.0, 1.0)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::red()
    }
}

impl FromStr for Color {
    type Err = String;

    fn from_str(string: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let css = string
            .parse::<CssColor>()
            .map_err(|_| "Parse color failed".to_string())?;

        Ok(Color::new(
            css.r as f32 / 255.0,
            css.g as f32 / 255.0,
            css.b as f32 / 255.0,
            css.a,
        ))
    }
}
