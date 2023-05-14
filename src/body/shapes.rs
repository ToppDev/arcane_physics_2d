use enum_as_inner::EnumAsInner;

use crate::math::Vec2f;

use self::{circle::Circle, polygon::Polygon};

use super::{
    collision::{CollisionResponse, CollisionWith},
    Position,
};

pub mod circle;
pub mod polygon;
pub mod rectangle;

pub const SHAPE_BORDER_WIDTH: f32 = 0.15; // [m]

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
impl CollisionWith<Shape> for Shape {
    fn collides(&self, other: &Shape) -> Option<CollisionResponse> {
        match self {
            Shape::Circle(circle) => match other {
                Shape::Circle(other) => circle.collides(other),
                Shape::Polygon(other) => circle.collides(other),
            },
            Shape::Polygon(polygon) => match other {
                Shape::Circle(other) => polygon.collides(other),
                Shape::Polygon(other) => polygon.collides(other),
            },
        }
    }
}

impl CollisionWith<Circle> for Circle {
    fn collides(&self, other: &Circle) -> Option<CollisionResponse> {
        None
    }
}

impl CollisionWith<Polygon> for Circle {
    fn collides(&self, other: &Polygon) -> Option<CollisionResponse> {
        None
    }
}

impl CollisionWith<Circle> for Polygon {
    fn collides(&self, other: &Circle) -> Option<CollisionResponse> {
        other.collides(self)
    }
}

impl CollisionWith<Polygon> for Polygon {
    fn collides(&self, other: &Polygon) -> Option<CollisionResponse> {
        None
    }
}
