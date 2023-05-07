use crate::{
    draw::Color,
    math::{calc_polygon_area, Vec2f},
    world::validate_body_parameters,
};

use super::{
    shapes::{polygon::Polygon, Shape},
    Body, CommonBody, DynamicBody, StaticBody,
};

impl Body {
    pub fn new_static_polygon(
        points: &[Vec2f],
        fill_color: Color,
        hitbox_color: Option<Color>,
        density: f32,
        restitution: f32,
    ) -> Result<Self, String> {
        let area = calc_polygon_area(points);
        let restitution = validate_body_parameters(area, density, restitution)?;

        Ok(Body::Static(StaticBody {
            data: CommonBody {
                shape: Shape::Polygon(Polygon::new(points, fill_color, hitbox_color)),
                density,
                mass: area * density,
                restitution,
                area,
            },
        }))
    }
    pub fn new_dynamic_polygon(
        points: &[Vec2f],
        fill_color: Color,
        hitbox_color: Option<Color>,
        density: f32,
        restitution: f32,
    ) -> Result<Self, String> {
        let area = calc_polygon_area(points);
        let restitution = validate_body_parameters(area, density, restitution)?;

        Ok(Body::Dynamic(DynamicBody {
            data: CommonBody {
                shape: Shape::Polygon(Polygon::new(points, fill_color, hitbox_color)),
                density,
                mass: area * density,
                restitution,
                area,
            },
            linear_velocity: Vec2f::zeros(),
            rotation_velocity: 0.0,
        }))
    }
}
