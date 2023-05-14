use crate::math::Vec2f;

pub struct CollisionResponse {
    pub normal: Vec2f,
    pub depth: f32,
}

pub trait CollisionWith<T> {
    fn collides(&self, other: &T) -> Option<CollisionResponse>;
}
