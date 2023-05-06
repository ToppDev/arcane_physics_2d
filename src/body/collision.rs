use ggez::glam::Vec2;

pub struct CollisionResponse {
    pub normal: Vec2,
    pub depth: f32,
}

pub fn collsion() -> Option<CollisionResponse> {
    None
}
