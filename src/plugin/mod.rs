use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use crate::{
    geometry::{collider::Collider, collision::CollisionWith, shape::Shape},
    physics::rigid_body::{apply_velocity, RigidBody, RigidBodyType, Velocity},
    player::move_player,
};

#[derive(Default)]
pub struct ArcanePhysicsPlugin2D {}

impl ArcanePhysicsPlugin2D {}

impl Plugin for ArcanePhysicsPlugin2D {
    fn build(&self, app: &mut App) {
        app.register_type::<Velocity>()
            .add_event::<CollisionEvent>()
            // add our system to the fixed timestep schedule
            .add_systems(
                (
                    collision_reset.before(check_for_collisions),
                    apply_velocity.before(check_for_collisions),
                    move_player
                        .before(check_for_collisions)
                        .after(apply_velocity),
                    check_for_collisions,
                ), // .in_schedule(CoreSchedule::FixedUpdate),
            )
            // configure our fixed timestep schedule to run twice a second
            .insert_resource(FixedTime::new_from_secs(1.0 / 60.0));
    }
}

#[derive(Default)]
pub struct CollisionEvent;

pub fn collision_reset(mut query: Query<&mut Collider>) {
    for mut collider in &mut query {
        collider.collided = false;
    }
}

pub fn check_for_collisions(
    mut query: Query<(&mut Transform, &mut Collider, &Name, Option<&RigidBody>)>,
    mut lines: ResMut<DebugLines>,
) {
    let mut iter = query.iter_combinations_mut();
    while let Some(
        [(mut trafo1, mut collider1, name1, body1), (mut trafo2, mut collider2, name2, body2)],
    ) = iter.fetch_next()
    {
        if let Some(collision) = match &collider1.shape {
            Shape::Circle(circle) => match &collider2.shape {
                Shape::Circle(other) => circle.collides(other, &trafo1, &trafo2, &mut lines),
                Shape::ConvexPolygon(other) => circle.collides(other, &trafo1, &trafo2, &mut lines),
            },
            Shape::ConvexPolygon(polygon) => match &collider2.shape {
                Shape::Circle(other) => polygon.collides(other, &trafo1, &trafo2, &mut lines),
                Shape::ConvexPolygon(other) => {
                    polygon.collides(other, &trafo1, &trafo2, &mut lines)
                }
            },
        } {
            // println!(
            //     "Collision for body {name1} to {name2} with depth {} in direction {:?} occured",
            //     collision.depth, collision.normal
            // );
            collider1.collided = true;
            collider2.collided = true;
            let type1 = match body1 {
                Some(body) => &body.body_type,
                None => &RigidBodyType::Fixed,
            };
            let type2 = match body2 {
                Some(body) => &body.body_type,
                None => &RigidBodyType::Fixed,
            };
            match (type1, type2) {
                (RigidBodyType::Dynamic, RigidBodyType::Dynamic) => {
                    trafo1.translation -= (collision.normal * collision.depth / 2.0).extend(0.0);
                    trafo2.translation += (collision.normal * collision.depth / 2.0).extend(0.0);
                    // println!(
                    //     "  Moving body '{name1}' in direction {} with depth {}",
                    //     -collision.normal,
                    //     collision.depth / 2.0
                    // );
                    // println!(
                    //     "  Moving body '{name2}' in direction {} with depth {}",
                    //     collision.normal,
                    //     collision.depth / 2.0
                    // );
                }
                (RigidBodyType::Dynamic, RigidBodyType::Fixed) => {
                    trafo1.translation -= (collision.normal * collision.depth).extend(0.0);
                    // println!(
                    //     "  Moving body '{name1}' in direction {} with depth {}",
                    //     -collision.normal, collision.depth
                    // );
                }
                (RigidBodyType::Fixed, RigidBodyType::Dynamic) => {
                    trafo2.translation += (collision.normal * collision.depth).extend(0.0);
                    // println!(
                    //     "  Moving body '{name2}' in direction {} with depth {}",
                    //     collision.normal, collision.depth
                    // );
                }
                (_, _) => (),
            }
        }
    }
}
