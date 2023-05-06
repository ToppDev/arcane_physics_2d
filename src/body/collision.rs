pub struct CollisionResponse {
    pub normal: Vec2,
    pub depth: f32,
}

pub trait CollisionWith<T> {
    pub fn collides(other: &T) -> Option<CollisionResponse>;
}
