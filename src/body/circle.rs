use crate::{
    draw::draw_circle, math::Vec2f, physics::PhysicalProperties, world::validate_body_parameters,
    SHAPE_BORDER_WIDTH,
};

use super::{
    components::{BodyColor, Movable, Position, Positionable, Rotatable, Velocity},
    BodyType, Drawable, Dynamic, Static, Updatable,
};

pub struct Circle<T: BodyType> {
    body_type: std::marker::PhantomData<T>,
    position: Position,
    radius: f32,
    velocity: Velocity,
    color: BodyColor,
    physics: PhysicalProperties,
}

impl<T: BodyType> Circle<T> {
    pub fn new(
        pos: Vec2f,
        radius: f32,
        color: BodyColor,
        density: f32,
        restitution: f32,
    ) -> Result<Self, String> {
        let area = radius.powi(2) * std::f32::consts::PI;
        let restitution = validate_body_parameters(area, density, restitution)?;
        let mass = area * density;

        Ok(Self {
            body_type: std::marker::PhantomData::<T>,
            position: Position(pos),
            radius,
            velocity: Velocity {
                linear: Vec2f::zeros(),
                rotation: 0.0,
            },
            color,
            physics: PhysicalProperties {
                density,
                mass,
                restitution,
                area,
            },
        })
    }
}

impl<T: BodyType> Positionable for Circle<T> {
    fn position(&self) -> Vec2f {
        self.position.0
    }
}

impl Movable for Circle<Dynamic> {
    fn move_to(&mut self, destination: Vec2f) {
        self.position.0 = destination;
    }
    fn offset(&mut self, offset: Vec2f) {
        self.position.0 += offset;
    }
    fn linear_velocity(&self) -> &Vec2f {
        &self.velocity.linear
    }
    fn linear_velocity_mut(&mut self) -> &mut Vec2f {
        &mut self.velocity.linear
    }
}

impl Rotatable for Circle<Dynamic> {
    fn rotate(&mut self, _angle_rad: f32) {}
    fn rotation_velocity(&self) -> f32 {
        self.velocity.rotation
    }
    fn rotation_velocity_mut(&mut self) -> &mut f32 {
        &mut self.velocity.rotation
    }
}

impl<T: BodyType> Drawable for Circle<T> {
    fn draw(&self) {
        if let Some(hitbox_color) = self.color.hitbox {
            draw_circle(
                self.position.0.x,
                self.position.0.y,
                self.radius,
                hitbox_color,
            );
        }
        let border_width = if self.color.hitbox.is_some() {
            SHAPE_BORDER_WIDTH
        } else {
            0.0
        };

        draw_circle(
            self.position.0.x,
            self.position.0.y,
            self.radius - border_width,
            self.color.fill,
        );
    }
}

impl Updatable for Circle<Static> {
    fn update(&mut self, dt: f32) {}
}
impl Updatable for Circle<Dynamic> {
    fn update(&mut self, dt: f32) {
        self.offset(self.linear_velocity() * dt);
    }
}
