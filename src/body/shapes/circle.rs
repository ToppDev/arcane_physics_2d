use crate::{
    body::{Drawable, Movable, Position},
    draw::*,
    math::Vec2f,
    SHAPE_BORDER_WIDTH,
};

pub struct Circle {
    pos: Vec2f,
    radius: f32,
    fill_color: Color,
    hitbox_color: Option<Color>,
}

impl Circle {
    pub fn new(
        x: f32,
        y: f32,
        radius: f32,
        fill_color: Color,
        border_color: Option<Color>,
    ) -> Self {
        Self {
            pos: Vec2f::new(x, y),
            radius,
            fill_color,
            hitbox_color: border_color,
        }
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl Position for Circle {
    fn position(&self) -> Vec2f {
        self.pos
    }
}

impl Movable for Circle {
    fn move_to(&mut self, destination: Vec2f) {
        self.pos = destination;
    }

    fn offset(&mut self, offset: Vec2f) {
        self.pos += offset;
    }
}

impl Drawable for Circle {
    fn draw(&self) {
        if let Some(hitbox_color) = self.hitbox_color {
            draw_circle(self.pos.x, self.pos.y, self.radius, hitbox_color);
        }

        draw_circle(
            self.pos.x,
            self.pos.y,
            self.radius - SHAPE_BORDER_WIDTH,
            self.fill_color,
        );
    }
}
