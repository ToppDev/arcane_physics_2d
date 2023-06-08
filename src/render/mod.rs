use bevy::prelude::*;
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin, DebugShapes};
use itertools::Itertools;

use crate::geometry::{collider::Collider, shape::Shape};

bitflags::bitflags! {
    pub struct DebugRenderMode: u16 {
        /// Render the collider shapes
        const COLLIDER_SHAPES = 0x0001;
        /// Render normals of collider shapes
        const COLLIDER_NORMALS = 0x0002;
    }
}

impl Default for DebugRenderMode {
    fn default() -> Self {
        Self::COLLIDER_SHAPES
    }
}

pub struct ArcanePhysics2DDebugRenderPlugin {
    /// Enables the debug rendering
    pub enabled: bool,
    /// Flag to select what debug rendering is done
    pub mode: DebugRenderMode,
}

impl Default for ArcanePhysics2DDebugRenderPlugin {
    fn default() -> Self {
        Self {
            enabled: true,
            mode: DebugRenderMode::default(),
        }
    }
}

impl Plugin for ArcanePhysics2DDebugRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DebugLinesPlugin::default())
            .add_system(debug_render);
    }
}

fn debug_render(
    query: Query<(&Collider, &Transform)>,
    mut shapes: ResMut<DebugShapes>,
    mut lines: ResMut<DebugLines>,
) {
    for (collider, transform) in &query {
        let color = if collider.collided {
            Color::RED
        } else {
            Color::WHITE
        };

        shapes
            .circle()
            .position(transform.translation)
            .radius(0.1)
            .rotation(transform.rotation)
            .color(color);

        match &collider.shape {
            Shape::Circle(circle) => {
                shapes
                    .circle()
                    .position(transform.translation)
                    .radius(circle.radius() * transform.scale.x)
                    .rotation(transform.rotation)
                    .color(color);
            }
            Shape::ConvexPolygon(polygon) => {
                polygon
                    .vertices()
                    .iter()
                    .circular_tuple_windows()
                    .for_each(|(&p1, &p2)| {
                        let start = transform.transform_point(p1.extend(0.0));
                        let end = transform.transform_point(p2.extend(0.0));
                        lines.line_colored(start, end, 0.0, color);
                    });
            }
        }
    }
}
