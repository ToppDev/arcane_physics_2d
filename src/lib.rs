mod geometry;
mod math;
mod physics;
mod player;
mod plugin;
mod render;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;
use geometry::collider::Collider;
use plugin::ArcanePhysicsPlugin2D;
use rand::{thread_rng, Rng};
use render::ArcanePhysics2DDebugRenderPlugin;

use crate::{
    physics::rigid_body::{RigidBody, RigidBodyType, Velocity},
    player::Player,
};

pub fn entry_point() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Arcane Physics 2D".into(),
                resolution: (1280.0, 720.0).into(),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugin(ShapePlugin)
        .add_plugin(ArcanePhysicsPlugin2D::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ArcanePhysics2DDebugRenderPlugin::default())
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 1.0 / 20.0,
            ..Default::default()
        },
        ..Default::default()
    });

    const OBJECT_SIZE: f32 = 2.0; // [m]
    const COLORS: [Color; 33] = [
        Color::ALICE_BLUE,
        Color::ANTIQUE_WHITE,
        Color::AQUAMARINE,
        Color::AZURE,
        Color::BEIGE,
        Color::BISQUE,
        // Color::BLACK,
        Color::BLUE,
        Color::CRIMSON,
        Color::CYAN,
        Color::DARK_GRAY,
        Color::DARK_GREEN,
        Color::FUCHSIA,
        Color::GOLD,
        Color::GRAY,
        Color::GREEN,
        Color::INDIGO,
        Color::LIME_GREEN,
        Color::MAROON,
        Color::MIDNIGHT_BLUE,
        Color::NAVY,
        Color::OLIVE,
        // Color::ORANGE,
        Color::ORANGE_RED,
        Color::PINK,
        Color::PURPLE,
        // Color::RED,
        Color::SALMON,
        Color::SEA_GREEN,
        Color::SILVER,
        Color::TEAL,
        Color::TOMATO,
        Color::TURQUOISE,
        Color::VIOLET,
        // Color::WHITE,
        Color::YELLOW,
        Color::YELLOW_GREEN,
    ];
    const SPAWN_SIZE: (f32, f32) = (50.0, 30.0);

    let mut objects = vec![(Vec2::new(0.0, 0.0), Color::ORANGE)];

    let mut rng = thread_rng();
    for _ in 0..=1 {
        for color in COLORS {
            let mut pos_i = 0;
            let mut min_dist = 2.0 * OBJECT_SIZE;
            let pos = loop {
                pos_i += 1;
                if pos_i > 1000 {
                    min_dist /= 1.9;
                }
                let pos = Vec2::new(
                    rng.gen_range(-SPAWN_SIZE.0 / 2.0..=SPAWN_SIZE.0 / 2.0),
                    rng.gen_range(-SPAWN_SIZE.1 / 2.0..=SPAWN_SIZE.1 / 2.0),
                );

                if objects
                    .iter()
                    .all(|&(existing_pos, _)| (existing_pos - pos).length() > min_dist)
                {
                    break pos;
                }
            };
            objects.push((pos, color));
        }
    }
    objects.remove(0);

    // Player
    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(OBJECT_SIZE, OBJECT_SIZE)).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::ORANGE)),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..default()
            },
            Collider::rect(2.0, 2.0),
            RigidBody {
                body_type: RigidBodyType::Dynamic,
            },
            Player,
            Velocity::default(),
        ))
        .insert(Name::new("Player"));

    for (pos, color) in objects {
        if rng.gen_bool(0.5) {
            // Circle
            commands
                .spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes
                            .add(shape::Circle::new(OBJECT_SIZE / 2.0).into())
                            .into(),
                        material: materials.add(ColorMaterial::from(color)),
                        transform: Transform::from_translation(pos.extend(0.0)),
                        ..default()
                    },
                    Collider::circle(1.0),
                    RigidBody {
                        body_type: RigidBodyType::Dynamic,
                    },
                    Velocity::default(),
                ))
                .insert(Name::new("Circle"));
        } else {
            // Quad
            commands
                .spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes
                            .add(shape::Quad::new(Vec2::new(OBJECT_SIZE, OBJECT_SIZE)).into())
                            .into(),
                        material: materials.add(ColorMaterial::from(color)),
                        transform: Transform::from_translation(pos.extend(0.0)),
                        ..default()
                    },
                    Collider::rect(2.0, 2.0),
                    RigidBody {
                        body_type: RigidBodyType::Dynamic,
                    },
                    Velocity::default(),
                ))
                .insert(Name::new("Quad"));
        }
    }

    for sides in 3..=6 {
        commands
            .spawn((
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::RegularPolygon::new(OBJECT_SIZE / 2.0, sides).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
                    transform: Transform::from_translation(Vec3::new(
                        8.,
                        (sides as f32 - 3.0) * 2.8,
                        0.0,
                    )),
                    ..default()
                },
                Collider::regular_polygon(1.0, sides),
                RigidBody {
                    body_type: RigidBodyType::Fixed,
                },
            ))
            .insert(Name::new(format!("Polygon {sides}")));
    }

    //     [0]   _____  [1]
    //          |     \
    //          |      \ [2]
    //          |      /
    //     [4]  |_____/ [3]
    let vertices = vec![
        Vec2::new(-OBJECT_SIZE / 2.0, OBJECT_SIZE / 2.0),
        Vec2::new(0.0, OBJECT_SIZE / 2.0),
        Vec2::new(OBJECT_SIZE / 2.0, 0.0),
        Vec2::new(0.0, -OBJECT_SIZE / 2.0),
        Vec2::new(-OBJECT_SIZE / 2.0, -OBJECT_SIZE / 2.0),
    ];
    let shape = shapes::Polygon {
        points: vertices.clone(),
        closed: true,
    };

    // This is not yer working
    commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                transform: Transform::from_translation(Vec3::new(
                    -3.0 * OBJECT_SIZE,
                    2.0 * OBJECT_SIZE,
                    0.0,
                )),
                ..Default::default()
            },
            Fill::color(Color::GOLD),
            Collider::convex_polygon(vertices),
            RigidBody {
                body_type: RigidBodyType::Fixed,
            },
        ))
        .insert(Name::new("Arrow"));
}
