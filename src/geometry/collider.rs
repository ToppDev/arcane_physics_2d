use bevy::prelude::*;

use super::shape::Shape;

#[derive(Component)]
pub struct Collider {
    pub shape: Shape,
    pub collided: bool,
}

impl Collider {
    pub fn circle(radius: f32) -> Self {
        Self {
            shape: Shape::circle(radius),
            collided: false,
        }
    }

    pub fn convex_polygon(vertices: Vec<Vec2>) -> Self {
        Self {
            shape: Shape::convex_polygon(vertices),
            collided: false,
        }
    }

    pub fn rect(width: f32, height: f32) -> Self {
        Self {
            shape: Shape::rect(width, height),
            collided: false,
        }
    }

    pub fn regular_polygon(radius: f32, sides: usize) -> Self {
        Self {
            shape: Shape::regular_polygon(radius, sides),
            collided: false,
        }
    }
}
