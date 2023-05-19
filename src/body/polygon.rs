use itertools::Itertools;
use macroquad::prelude::*;

use crate::SHAPE_BORDER_WIDTH;

use super::components::{BodyColor, Position};

pub struct Polygon {
    position: Position,
    vertices: Vec<Vec2>,
}
impl Polygon {
    pub fn new(position: Position, vertices: Vec<Vec2>) -> Self {
        Self { position, vertices }
    }

    pub fn position(&self) -> Vec2 {
        self.position.0
    }
    pub fn vertices(&self) -> &Vec<Vec2> {
        &self.vertices
    }

    pub fn move_to(&mut self, destination: Vec2) {
        self.position.0 = destination;
    }
    pub fn offset(&mut self, offset: Vec2) {
        self.position.0 += offset;
    }
    pub fn rotate(&mut self, angle_rad: f32) {
        let rot = Affine2::from_angle(angle_rad);
        for vertex in &mut self.vertices {
            *vertex = rot.transform_vector2(*vertex);
        }
    }

    pub fn draw(&self, color: &BodyColor) {
        if let Some(hitbox_color) = color.hitbox {
            self.vertices
                .iter()
                .map(|&v| v + self.position.0)
                .circular_tuple_windows()
                .for_each(|(v1, v2, v3)| draw_triangle(v1, v2, v3, hitbox_color));
        }
        let border_width = if color.hitbox.is_some() {
            SHAPE_BORDER_WIDTH
        } else {
            0.0
        };
        self.vertices
            .iter()
            .map(|&v| v - v.normalize() * border_width + self.position.0)
            .combinations(3)
            .for_each(|p| draw_triangle(p[0], p[1], p[2], color.fill));
    }
}
