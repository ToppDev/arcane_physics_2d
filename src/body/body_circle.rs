use crate::{draw::Color, math::Vec2f, MAX_BODY_SIZE, MAX_DENSITY, MIN_BODY_SIZE, MIN_DENSITY};

use super::{
    shapes::{circle::Circle, Shape},
    Body, CommonBody, DynamicBody, StaticBody,
};

impl Body {
    fn validate_circle_parameters(
        radius: f32,
        density: f32,
        restitution: f32,
    ) -> Result<(f32, f32), String> {
        let area = radius.powi(2) * std::f32::consts::PI;
        if area < MIN_BODY_SIZE {
            let min_radius = (MIN_BODY_SIZE / std::f32::consts::PI).sqrt();
            return Err(format!(
                "Circle radius too small. Min circle radius is {min_radius}"
            ));
        }
        if area > MAX_BODY_SIZE {
            let max_radius = (MAX_BODY_SIZE / std::f32::consts::PI).sqrt();
            return Err(format!(
                "Circle radius too large. Max circle radius is {max_radius}"
            ));
        }
        if density < MIN_DENSITY {
            return Err(format!("Density too small. Min density is {MIN_DENSITY}"));
        }
        if density > MAX_DENSITY {
            return Err(format!("Density too large. Max density is {MAX_DENSITY}"));
        }

        let restitution = restitution.clamp(0.0, 1.0);

        Ok((area, restitution))
    }

    pub fn new_static_circle(
        position: Vec2f,
        radius: f32,
        fill_color: Color,
        hitbox_color: Option<Color>,
        density: f32,
        restitution: f32,
    ) -> Result<Self, String> {
        let (area, restitution) = Body::validate_circle_parameters(radius, density, restitution)?;

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
        let (area, restitution) = Body::validate_circle_parameters(radius, density, restitution)?;

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
