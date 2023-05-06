mod shapes;

use self::shapes::{circle::Circle, polygon::Polygon};
use crate::{draw::Color, math::Vec2f};

pub trait Position {
    fn position(&self) -> Vec2f;
}
pub trait Movable {
    fn move_to(&mut self, destination: Vec2f);
    fn offset(&mut self, offset: Vec2f);
}
pub trait Drawable {
    fn draw(&self);
}

pub enum Body {
    Static(StaticBody),
    Dynamic(DynamicBody),
}

pub struct StaticBody {
    shape: Shape,
}

pub struct DynamicBody {
    shape: Shape,
    linear_velocity: Vec2f,
    rotation_velocity: f32,
    // density: f32,
    // mass: f32,
    // restitution: f32,
    // area: f32,
}

pub enum Shape {
    Circle(Circle),
    Polygon(Polygon),
}

impl Position for Body {
    fn position(&self) -> Vec2f {
        match self {
            Body::Static(body) => body.position(),
            Body::Dynamic(body) => body.position(),
        }
    }
}
impl Movable for Body {
    fn move_to(&mut self, destination: Vec2f) {
        match self {
            Body::Static(_) => (),
            Body::Dynamic(body) => body.move_to(destination),
        }
    }
    fn offset(&mut self, offset: Vec2f) {
        match self {
            Body::Static(_) => (),
            Body::Dynamic(body) => body.offset(offset),
        }
    }
}
impl Drawable for Body {
    fn draw(&self) {
        let shape = match &self {
            Body::Static(body) => &body.shape,
            Body::Dynamic(body) => &body.shape,
        };
        match shape {
            Shape::Circle(circle) => circle.draw(),
            Shape::Polygon(polygon) => polygon.draw(),
        }
    }
}

impl Position for StaticBody {
    fn position(&self) -> Vec2f {
        self.shape.position()
    }
}

impl Position for DynamicBody {
    fn position(&self) -> Vec2f {
        self.shape.position()
    }
}
impl Movable for DynamicBody {
    fn move_to(&mut self, destination: Vec2f) {
        match &mut self.shape {
            Shape::Circle(circle) => circle.move_to(destination),
            Shape::Polygon(polygon) => polygon.move_to(destination),
        }
    }
    fn offset(&mut self, offset: Vec2f) {
        match &mut self.shape {
            Shape::Circle(circle) => circle.offset(offset),
            Shape::Polygon(polygon) => polygon.offset(offset),
        }
    }
}

impl Position for Shape {
    fn position(&self) -> Vec2f {
        match self {
            Shape::Circle(circle) => circle.position(),
            Shape::Polygon(polygon) => polygon.position(),
        }
    }
}

impl Body {
    pub fn new_static_circle(
        position: Vec2f,
        radius: f32,
        fill_color: Color,
        hitbox_color: Option<Color>,
    ) -> Self {
        Body::Static(StaticBody {
            shape: Shape::Circle(Circle::new(
                position.x,
                position.y,
                radius,
                fill_color,
                hitbox_color,
            )),
        })
    }
    pub fn new_dynamic_circle(
        position: Vec2f,
        radius: f32,
        fill_color: Color,
        hitbox_color: Option<Color>,
        linear_velocity: Vec2f,
    ) -> Self {
        Body::Dynamic(DynamicBody {
            shape: Shape::Circle(Circle::new(
                position.x,
                position.y,
                radius,
                fill_color,
                hitbox_color,
            )),
            linear_velocity,
            rotation_velocity: 0.0,
        })
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
