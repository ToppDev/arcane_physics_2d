use bevy::prelude::*;

pub enum RigidBodyType {
    /// Affected by all external forces.
    Dynamic,
    /// Not affected by external forces. Fixed in place.
    Fixed,
    /// Not affected by external forces, but moved by user.
    Kinematic,
}

#[derive(Component)]
pub struct RigidBody {
    pub body_type: RigidBodyType,
}

#[derive(Component, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct Velocity {
    /// Linear velocity
    pub lin_vel: Vec2,
    /// Angular velocity
    pub ang_vel: Vec2,
}

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<FixedTime>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.lin_vel.x * time_step.period.as_secs_f32();
        transform.translation.y += velocity.lin_vel.y * time_step.period.as_secs_f32();
    }
}
