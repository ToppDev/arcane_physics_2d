pub mod body_circle;
pub mod body_polygon;
pub mod body_rectangle;
pub mod collision;
mod shapes;

use enum_as_inner::EnumAsInner;

use self::{collision::CollisionWith, shapes::Shape};
use crate::math::Vec2f;

pub trait Position {
    fn position(&self) -> Vec2f;
}
pub trait Movable {
    fn move_to(&mut self, destination: Vec2f);
    fn offset(&mut self, offset: Vec2f);
}
pub trait Rotatable {
    fn rotate(&mut self, angle_rad: f32);
}
pub trait Drawable {
    fn draw(&self);
}
pub trait Updatable {
    fn update(&mut self, dt: f32);
}

#[derive(EnumAsInner)]
pub enum Body {
    Static(StaticBody),
    Dynamic(DynamicBody),
}

pub struct CommonBody {
    shape: Shape,

    density: f32,
    mass: f32,
    /// The coefficient of restitution (COR, also denoted by e), is the ratio of the final to
    /// initial relative speed between two objects after they collide.
    ///
    /// It normally ranges from 0 to 1 where 1 would be a perfectly elastic collision.
    /// A perfectly inelastic collision has a coefficient of 0, but a 0 value does not have to be
    /// perfectly inelastic
    restitution: f32,
    area: f32,
}

pub struct StaticBody {
    data: CommonBody,
}

pub struct DynamicBody {
    data: CommonBody,

    linear_velocity: Vec2f,
    rotation_velocity: f32,
}

impl Position for Body {
    fn position(&self) -> Vec2f {
        match self {
            Body::Static(body) => body.position(),
            Body::Dynamic(body) => body.position(),
        }
    }
}
impl Drawable for Body {
    fn draw(&self) {
        let shape = match &self {
            Body::Static(body) => &body.data.shape,
            Body::Dynamic(body) => &body.data.shape,
        };
        match shape {
            Shape::Circle(circle) => circle.draw(),
            Shape::Polygon(polygon) => polygon.draw(),
        }
    }
}
impl Updatable for Body {
    fn update(&mut self, dt: f32) {
        if let Body::Dynamic(dyn_body) = self {
            dyn_body.offset(dyn_body.linear_velocity * dt);
            if let Shape::Polygon(polygon) = &mut dyn_body.data.shape {
                polygon.rotate(dyn_body.rotation_velocity * dt);
            }
        }
    }
}
impl CollisionWith<Body> for Body {
    fn collides(&self, other: &Body) -> Option<collision::CollisionResponse> {
        let shape = match &self {
            Body::Static(body) => &body.data.shape,
            Body::Dynamic(body) => &body.data.shape,
        };
        let other = match other {
            Body::Static(body) => &body.data.shape,
            Body::Dynamic(body) => &body.data.shape,
        };
        shape.collides(other)
    }
}

impl Position for StaticBody {
    fn position(&self) -> Vec2f {
        self.data.shape.position()
    }
}

impl Position for DynamicBody {
    fn position(&self) -> Vec2f {
        self.data.shape.position()
    }
}
impl Movable for DynamicBody {
    fn move_to(&mut self, destination: Vec2f) {
        match &mut self.data.shape {
            Shape::Circle(circle) => circle.move_to(destination),
            Shape::Polygon(polygon) => polygon.move_to(destination),
        }
    }
    fn offset(&mut self, offset: Vec2f) {
        match &mut self.data.shape {
            Shape::Circle(circle) => circle.offset(offset),
            Shape::Polygon(polygon) => polygon.offset(offset),
        }
    }
}

impl StaticBody {
    // pub fn shape(&self) -> &Shape {
    //     &self.shape
    // }
}
impl DynamicBody {
    // pub fn shape(&self) -> &Shape {
    //     &self.shape
    // }
    pub fn linear_velocity(&self) -> &Vec2f {
        &self.linear_velocity
    }
    pub fn linear_velocity_mut(&mut self) -> &mut Vec2f {
        &mut self.linear_velocity
    }
    pub fn rotation_velocity(&self) -> f32 {
        self.rotation_velocity
    }
    pub fn rotation_velocity_mut(&mut self) -> &mut f32 {
        &mut self.rotation_velocity
    }
}
