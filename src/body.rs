use crate::{
    collision::{CollisionResponse, CollisionWith},
    draw::Color,
    math::Vec2f,
};

use self::{
    circle::Circle,
    components::{Colored, Positionable},
    polygon::Polygon,
};

pub mod circle;
pub mod components;
pub mod polygon;

pub trait BodyType {}
pub struct Dynamic;
pub struct Static;
impl BodyType for Dynamic {}
impl BodyType for Static {}

pub trait Drawable {
    fn draw(&self);
}
pub trait Updatable {
    fn update(&mut self, dt: f32);
}

pub enum Body {
    StaticCircle(Circle<Static>),
    StaticPolygon(Polygon<Static>),
    DynamicCircle(Circle<Dynamic>),
    DynamicPolygon(Polygon<Dynamic>),
}

impl Positionable for Body {
    fn position(&self) -> Vec2f {
        match self {
            Body::StaticCircle(circle) => circle.position(),
            Body::StaticPolygon(polygon) => polygon.position(),
            Body::DynamicCircle(circle) => circle.position(),
            Body::DynamicPolygon(polygon) => polygon.position(),
        }
    }
}

impl Colored for Body {
    fn change_fill_color(&mut self, color: Color) {
        match self {
            Body::StaticCircle(circle) => circle.change_fill_color(color),
            Body::StaticPolygon(polygon) => polygon.change_fill_color(color),
            Body::DynamicCircle(circle) => circle.change_fill_color(color),
            Body::DynamicPolygon(polygon) => polygon.change_fill_color(color),
        }
    }
    fn change_hitbox_color(&mut self, color: Color) {
        match self {
            Body::StaticCircle(circle) => circle.change_hitbox_color(color),
            Body::StaticPolygon(polygon) => polygon.change_hitbox_color(color),
            Body::DynamicCircle(circle) => circle.change_hitbox_color(color),
            Body::DynamicPolygon(polygon) => polygon.change_hitbox_color(color),
        }
    }
    fn remove_hitbox_color(&mut self) {
        match self {
            Body::StaticCircle(circle) => circle.remove_hitbox_color(),
            Body::StaticPolygon(polygon) => polygon.remove_hitbox_color(),
            Body::DynamicCircle(circle) => circle.remove_hitbox_color(),
            Body::DynamicPolygon(polygon) => polygon.remove_hitbox_color(),
        }
    }
}

impl Drawable for Body {
    fn draw(&self) {
        match self {
            Body::StaticCircle(circle) => circle.draw(),
            Body::StaticPolygon(polygon) => polygon.draw(),
            Body::DynamicCircle(circle) => circle.draw(),
            Body::DynamicPolygon(polygon) => polygon.draw(),
        }
    }
}

impl Updatable for Body {
    fn update(&mut self, dt: f32) {
        match self {
            Body::StaticCircle(circle) => circle.update(dt),
            Body::StaticPolygon(polygon) => polygon.update(dt),
            Body::DynamicCircle(circle) => circle.update(dt),
            Body::DynamicPolygon(polygon) => polygon.update(dt),
        }
    }
}

impl CollisionWith<Body> for Body {
    fn collides(&self, other: &Body) -> Option<CollisionResponse> {
        match self {
            Body::StaticCircle(circle) => circle.collides(other),
            Body::StaticPolygon(polygon) => polygon.collides(other),
            Body::DynamicCircle(circle) => circle.collides(other),
            Body::DynamicPolygon(polygon) => polygon.collides(other),
        }
    }
}

impl<T: BodyType> CollisionWith<Body> for Circle<T> {
    fn collides(&self, other: &Body) -> Option<CollisionResponse> {
        match other {
            Body::StaticCircle(circle) => circle.collides(self),
            Body::StaticPolygon(polygon) => polygon.collides(self),
            Body::DynamicCircle(circle) => circle.collides(self),
            Body::DynamicPolygon(polygon) => polygon.collides(self),
        }
    }
}
impl<T: BodyType> CollisionWith<Body> for Polygon<T> {
    fn collides(&self, other: &Body) -> Option<CollisionResponse> {
        match other {
            Body::StaticCircle(circle) => circle.collides(self),
            Body::StaticPolygon(polygon) => polygon.collides(self),
            Body::DynamicCircle(circle) => circle.collides(self),
            Body::DynamicPolygon(polygon) => polygon.collides(self),
        }
    }
}
