use crate::{
    body::{Drawable, Movable, Position},
    math::Vec2f,
};

pub struct Polygon {}

impl Position for Polygon {
    fn position(&self) -> Vec2f {
        todo!()
    }
}

impl Movable for Polygon {
    fn move_to(&mut self, destination: Vec2f) {
        todo!()
    }
    fn offset(&mut self, offset: Vec2f) {
        todo!()
    }
}

impl Drawable for Polygon {
    fn draw(&self) {
        todo!()
    }
}
