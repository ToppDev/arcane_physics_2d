use std::fmt;

use bevy::prelude::*;

#[cfg(test)]
use approx::AbsDiffEq;

#[derive(PartialEq, Deref, DerefMut)]
pub struct Vector2(Vec2);

#[cfg(test)]
impl AbsDiffEq for Vector2 {
    type Epsilon = f32;

    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        f32::abs_diff_eq(&self.x, &other.x, epsilon) && f32::abs_diff_eq(&self.y, &other.y, epsilon)
    }
}

impl fmt::Debug for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Vector2> for Vec2 {
    fn from(value: Vector2) -> Self {
        value.0
    }
}
impl From<Vec2> for Vector2 {
    fn from(value: Vec2) -> Self {
        Self(value)
    }
}
