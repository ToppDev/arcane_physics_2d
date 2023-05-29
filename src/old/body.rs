use macroquad::prelude::*;

use crate::{
    collision::{CollisionResponse, CollisionWith},
    math::{calc_polygon_area, calc_polygon_centroid, calc_rect_vertices},
    physics::PhysicalProperties,
    world::validate_body_parameters,
};

use self::{
    circle::Circle,
    components::{BodyColor, Colored, Movable, Position, Positionable, Rotatable, Velocity},
    polygon::Polygon,
};

pub mod circle;
pub mod components;
pub mod polygon;

#[derive(Clone, Copy)]
pub enum RigidBodyType {
    /// Affected by all external forces.
    Dynamic,
    /// Not affected by external forces. Fixed in place.
    Fixed,
    /// Not affected by external forces, but moved by user.
    Kinematic,
}

pub struct Body {
    shape: Shape,
    velocity: Velocity,
    color: BodyColor,
    body_type: RigidBodyType,
    physics: PhysicalProperties,
}

impl Body {
    pub fn body_type(&self) -> RigidBodyType {
        self.body_type
    }

    pub fn new_circle(
        position: Vec2,
        radius: f32,
        color: BodyColor,
        body_type: RigidBodyType,
        density: f32,
        restitution: f32,
    ) -> Result<Self, String> {
        let area = radius.powi(2) * std::f32::consts::PI;
        let restitution = validate_body_parameters(area, density, restitution)?;
        let mass = area * density;

        Ok(Self {
            shape: Shape::Circle(Circle::new(Position(position), radius)),
            velocity: Velocity {
                linear: Vec2::ZERO,
                rotation: 0.0,
            },
            color,
            body_type,
            physics: PhysicalProperties {
                density,
                mass,
                restitution,
                area,
            },
        })
    }
    pub fn new_polygon(
        points: &[Vec2],
        color: BodyColor,
        body_type: RigidBodyType,
        density: f32,
        restitution: f32,
    ) -> Result<Self, String> {
        let area = calc_polygon_area(points);
        let restitution = validate_body_parameters(area, density, restitution)?;
        let mass = area * density;

        let centroid = calc_polygon_centroid(points);
        Ok(Self {
            shape: Shape::Polygon(Polygon::new(
                Position(centroid),
                points.iter().map(|&x| x - centroid).collect(),
            )),
            velocity: Velocity {
                linear: Vec2::ZERO,
                rotation: 0.0,
            },
            color,
            body_type,
            physics: PhysicalProperties {
                density,
                mass,
                restitution,
                area,
            },
        })
    }

    pub fn new_rect(
        center: Vec2,
        width: f32,
        height: f32,
        rotation_deg: f32,
        color: BodyColor,
        body_type: RigidBodyType,
        density: f32,
        restitution: f32,
    ) -> Result<Self, String> {
        let area = width * height;
        let restitution = validate_body_parameters(area, density, restitution)?;
        let mass = area * density;

        let points = calc_rect_vertices(center.x, center.y, width, height, rotation_deg);

        Ok(Self {
            shape: Shape::Polygon(Polygon::new(
                Position(center),
                points.iter().map(|x| *x - center).collect(),
            )),
            velocity: Velocity {
                linear: Vec2::ZERO,
                rotation: 0.0,
            },
            color,
            body_type,
            physics: PhysicalProperties {
                density,
                mass,
                restitution,
                area,
            },
        })
    }
}

pub enum Shape {
    Circle(Circle),
    Polygon(Polygon),
}
impl Positionable for Body {
    fn position(&self) -> Vec2 {
        match &self.shape {
            Shape::Circle(circle) => circle.position(),
            Shape::Polygon(polygon) => polygon.position(),
        }
    }
}

impl Movable for Body {
    fn move_to(&mut self, destination: Vec2) {
        match &mut self.shape {
            Shape::Circle(circle) => circle.move_to(destination),
            Shape::Polygon(polygon) => polygon.move_to(destination),
        }
    }
    fn offset(&mut self, offset: Vec2) {
        match &mut self.shape {
            Shape::Circle(circle) => circle.offset(offset),
            Shape::Polygon(polygon) => polygon.offset(offset),
        }
    }
    fn linear_velocity(&self) -> &Vec2 {
        &self.velocity.linear
    }
    fn linear_velocity_mut(&mut self) -> &mut Vec2 {
        &mut self.velocity.linear
    }
}

impl Rotatable for Body {
    fn rotate(&mut self, angle_rad: f32) {
        if let Shape::Polygon(polygon) = &mut self.shape {
            polygon.rotate(angle_rad);
        }
    }
    fn rotation_velocity(&self) -> f32 {
        self.velocity.rotation
    }
    fn rotation_velocity_mut(&mut self) -> &mut f32 {
        &mut self.velocity.rotation
    }
}

impl Colored for Body {
    fn change_fill_color(&mut self, color: Color) {
        self.color.fill = color;
    }
    fn change_hitbox_color(&mut self, color: Color) {
        self.color.hitbox = Some(color);
    }
    fn remove_hitbox_color(&mut self) {
        self.color.hitbox = None;
    }
}

impl Body {
    pub fn draw(&self) {
        match &self.shape {
            Shape::Circle(circle) => circle.draw(&self.color),
            Shape::Polygon(polygon) => polygon.draw(&self.color),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.offset(self.velocity.linear * dt);
        self.rotate(self.velocity.rotation * dt);
    }
}

impl CollisionWith<Body> for Body {
    fn collides(&self, other: &Body) -> Option<CollisionResponse> {
        self.shape.collides(&other.shape)
    }
}

impl CollisionWith<Shape> for Shape {
    fn collides(&self, other: &Shape) -> Option<CollisionResponse> {
        match self {
            Shape::Circle(circle) => match other {
                Shape::Circle(other_circle) => circle.collides(other_circle),
                Shape::Polygon(other_polygon) => circle.collides(other_polygon),
            },
            Shape::Polygon(polygon) => match other {
                Shape::Circle(other_circle) => polygon.collides(other_circle),
                Shape::Polygon(other_polygon) => polygon.collides(other_polygon),
            },
        }
    }
}
