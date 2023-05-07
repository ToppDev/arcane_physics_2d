use crate::{draw::Color, math::Vec2f, world::validate_body_parameters};

use super::{
    shapes::{polygon::Polygon, rectangle::calc_rect_vertices, Shape},
    Body, CommonBody, DynamicBody, StaticBody,
};

impl Body {
    pub fn new_static_rectangle(
        center: Vec2f,
        width: f32,
        height: f32,
        rotation: f32,
        fill_color: Color,
        hitbox_color: Option<Color>,
        density: f32,
        restitution: f32,
    ) -> Result<Self, String> {
        let area = width * height;
        let restitution = validate_body_parameters(area, density, restitution)?;

        let points = calc_rect_vertices(center.x, center.y, width, height, rotation);
        Ok(Body::Static(StaticBody {
            data: CommonBody {
                shape: Shape::Polygon(Polygon::new(&points, fill_color, hitbox_color)),
                density,
                mass: area * density,
                restitution,
                area,
            },
        }))
    }
    pub fn new_dynamic_rectangle(
        center: Vec2f,
        width: f32,
        height: f32,
        rotation: f32,
        fill_color: Color,
        hitbox_color: Option<Color>,
        density: f32,
        restitution: f32,
    ) -> Result<Self, String> {
        let area = width * height;
        let restitution = validate_body_parameters(area, density, restitution)?;

        let points = calc_rect_vertices(center.x, center.y, width, height, rotation);
        Ok(Body::Dynamic(DynamicBody {
            data: CommonBody {
                shape: Shape::Polygon(Polygon::new(&points, fill_color, hitbox_color)),
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
