use crate::{draw::Color, math::Vec2f, world::validate_body_parameters};

use super::{
    shapes::{circle::Circle, Shape},
    Body, CommonBody, DynamicBody, StaticBody,
};

impl Body {
    pub fn new_static_circle(
        position: Vec2f,
        radius: f32,
        fill_color: Color,
        hitbox_color: Option<Color>,
        density: f32,
        restitution: f32,
    ) -> Result<Self, String> {
        let area = radius.powi(2) * std::f32::consts::PI;
        let restitution = validate_body_parameters(area, density, restitution)?;

        Ok(Body::Static(StaticBody {
            data: CommonBody {
                shape: Shape::Circle(Circle::new(
                    position.x,
                    position.y,
                    radius,
                    fill_color,
                    hitbox_color,
                )),
                density,
                mass: area * density,
                restitution,
                area,
            },
        }))
    }
    pub fn new_dynamic_circle(
        position: Vec2f,
        radius: f32,
        fill_color: Color,
        hitbox_color: Option<Color>,
        density: f32,
        restitution: f32,
    ) -> Result<Self, String> {
        let area = radius.powi(2) * std::f32::consts::PI;
        let restitution = validate_body_parameters(area, density, restitution)?;

        Ok(Body::Dynamic(DynamicBody {
            data: CommonBody {
                shape: Shape::Circle(Circle::new(
                    position.x,
                    position.y,
                    radius,
                    fill_color,
                    hitbox_color,
                )),
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
