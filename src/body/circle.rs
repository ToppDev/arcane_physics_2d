use macroquad::prelude::*;

use crate::SHAPE_BORDER_WIDTH;

use super::components::{BodyColor, Position};

pub struct Circle {
    position: Position,
    radius: f32,
}

impl Circle {
    pub fn new(position: Position, radius: f32) -> Self {
        Self { position, radius }
    }

    pub fn position(&self) -> Vec2 {
        self.position.0
    }
    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn move_to(&mut self, destination: Vec2) {
        self.position.0 = destination;
    }
    pub fn offset(&mut self, offset: Vec2) {
        self.position.0 += offset;
    }

    pub fn draw(&self, color: &BodyColor) {
        if let Some(hitbox_color) = color.hitbox {
            draw_circle(
                self.position.0.x,
                self.position.0.y,
                self.radius,
                hitbox_color,
            );
        }
        let border_width = if color.hitbox.is_some() {
            SHAPE_BORDER_WIDTH
        } else {
            0.0
        };

        draw_circle(
            self.position.0.x,
            self.position.0.y,
            self.radius - border_width,
            color.fill,
        );
    }
}
