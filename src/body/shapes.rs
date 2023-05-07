use enum_as_inner::EnumAsInner;

use crate::math::Vec2f;

use self::{circle::Circle, polygon::Polygon};

use super::Position;

pub mod circle;
pub mod polygon;
pub mod rectangle;

pub const SHAPE_BORDER_WIDTH: f32 = 0.1; // [m]

#[derive(EnumAsInner)]
pub enum Shape {
    Circle(Circle),
    Polygon(Polygon),
}

impl Position for Shape {
    fn position(&self) -> Vec2f {
        match self {
            Shape::Circle(circle) => circle.position(),
            Shape::Polygon(polygon) => polygon.position(),
        }
    }
}
