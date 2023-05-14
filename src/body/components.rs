use crate::{draw::Color, math::Vec2f};

pub struct Position(pub Vec2f);

pub trait Positionable {
    fn position(&self) -> Vec2f;
}

pub struct BodyColor {
    pub fill: Color,
    pub hitbox: Option<Color>,
}

pub struct Velocity {
    /// Linear velocity in [m/s]
    pub linear: Vec2f,
    /// Rotation velocity in [rad/s] for counter-clockwise rotation as positive direction
    pub rotation: f32,
}
pub trait Movable {
    fn move_to(&mut self, destination: Vec2f);
    fn offset(&mut self, offset: Vec2f);
    fn linear_velocity(&self) -> &Vec2f;
    fn linear_velocity_mut(&mut self) -> &mut Vec2f;
}
pub trait Rotatable {
    fn rotate(&mut self, angle_rad: f32);
    fn rotation_velocity(&self) -> f32;
    fn rotation_velocity_mut(&mut self) -> &mut f32;
}
pub trait Colored {
    fn change_fill_color(&mut self, color: Color);
    fn change_hitbox_color(&mut self, color: Color);
    fn remove_hitbox_color(&mut self);
}
