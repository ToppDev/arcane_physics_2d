mod shapes;

use self::shapes::{circle::Circle, polygon::Polygon};
use crate::{draw::Color, math::Vec2f, MAX_BODY_SIZE, MAX_DENSITY, MIN_BODY_SIZE, MIN_DENSITY};

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

pub enum BodyType {
    Static,
    Dynamic,
}

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
            Body::Static(body) => &body.data.shape,
            Body::Dynamic(body) => &body.data.shape,
        };
        match shape {
            Shape::Circle(circle) => circle.draw(),
            Shape::Polygon(polygon) => polygon.draw(),
        }
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

impl Position for Shape {
    fn position(&self) -> Vec2f {
        match self {
            Shape::Circle(circle) => circle.position(),
            Shape::Polygon(polygon) => polygon.position(),
        }
    }
}

impl Body {
    pub fn new_circle(
        position: Vec2f,
        radius: f32,
        fill_color: Color,
        hitbox_color: Option<Color>,
        density: f32,
        restitution: f32,
        body_type: BodyType,
    ) -> Result<Self, String> {
        let area = radius.powi(2) * std::f32::consts::PI;
        if area < MIN_BODY_SIZE {
            let min_radius = (MIN_BODY_SIZE / std::f32::consts::PI).sqrt();
            return Err(format!(
                "Circle radius too small. Min circle radius is {min_radius}"
            ));
        }
        if area > MAX_BODY_SIZE {
            let max_radius = (MAX_BODY_SIZE / std::f32::consts::PI).sqrt();
            return Err(format!(
                "Circle radius too large. Max circle radius is {max_radius}"
            ));
        }
        if density < MIN_DENSITY {
            return Err(format!(
                "Circle density too small. Min density is {MIN_DENSITY}"
            ));
        }
        if density > MAX_DENSITY {
            return Err(format!(
                "Circle density too large. Max density is {MAX_DENSITY}"
            ));
        }

        let restitution = restitution.clamp(0.0, 1.0);

        let area = radius.powi(2) * std::f32::consts::PI;
        match body_type {
            BodyType::Static => Ok(Body::Static(StaticBody {
                data: CommonBody {
                    shape: Shape::Circle(Circle::new(
                        position.x,
                        position.y,
                        radius,
                        fill_color,
                        hitbox_color,
                    )),
                    density,
                    mass: area * density,
                    restitution,
                    area,
                },
            })),
            BodyType::Dynamic => Ok(Body::Dynamic(DynamicBody {
                data: CommonBody {
                    shape: Shape::Circle(Circle::new(
                        position.x,
                        position.y,
                        radius,
                        fill_color,
                        hitbox_color,
                    )),
                    density,
                    mass: area * density,
                    restitution,
                    area,
                },
                linear_velocity: Vec2f::zeros(),
                rotation_velocity: 0.0,
            })),
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
