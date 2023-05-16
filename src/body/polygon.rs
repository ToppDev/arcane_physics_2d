use itertools::Itertools;

use crate::{
    draw::draw_triangle,
    math::{calc_polygon_area, calc_polygon_centroid, calc_rect_vertices, Vec2f},
    physics::PhysicalProperties,
    world::validate_body_parameters,
    SHAPE_BORDER_WIDTH,
};

use super::{
    components::{BodyColor, Colored, Movable, Position, Positionable, Rotatable, Velocity},
    BodyType, Drawable, Dynamic, Static, Updatable,
};

pub struct Polygon<T: BodyType> {
    body_kinematic_type: std::marker::PhantomData<T>,
    centroid: Position,
    vertices: Vec<Vec2f>,
    velocity: Velocity,
    color: BodyColor,
    physics: PhysicalProperties,
}

impl<T: BodyType> Polygon<T> {
    pub fn new(
        points: &[Vec2f],
        color: BodyColor,
        density: f32,
        restitution: f32,
    ) -> Result<Self, String> {
        let area = calc_polygon_area(points);
        let restitution = validate_body_parameters(area, density, restitution)?;
        let mass = area * density;

        let centroid = calc_polygon_centroid(points);
        Ok(Self {
            body_kinematic_type: std::marker::PhantomData::<T>,
            centroid: Position(centroid),
            vertices: points.iter().map(|x| x - centroid).collect(),
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

    pub fn new_rect(
        center: Vec2f,
        width: f32,
        height: f32,
        rotation_deg: f32,
        color: BodyColor,
        density: f32,
        restitution: f32,
    ) -> Result<Self, String> {
        let area = width * height;
        let restitution = validate_body_parameters(area, density, restitution)?;
        let mass = area * density;

        let points = calc_rect_vertices(center.x, center.y, width, height, rotation_deg);
        Ok(Self {
            body_kinematic_type: std::marker::PhantomData::<T>,
            centroid: Position(center),
            vertices: points.iter().map(|x| x - center).collect(),
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

    pub fn vertices(&self) -> &[Vec2f] {
        &self.vertices
    }
}

impl<T: BodyType> Positionable for Polygon<T> {
    fn position(&self) -> Vec2f {
        self.centroid.0
    }
}

impl Movable for Polygon<Dynamic> {
    fn move_to(&mut self, destination: Vec2f) {
        self.centroid.0 = destination;
    }
    fn offset(&mut self, offset: Vec2f) {
        self.centroid.0 += offset;
    }
    fn linear_velocity(&self) -> &Vec2f {
        &self.velocity.linear
    }
    fn linear_velocity_mut(&mut self) -> &mut Vec2f {
        &mut self.velocity.linear
    }
}

impl Rotatable for Polygon<Dynamic> {
    fn rotate(&mut self, angle_rad: f32) {
        let rot = nalgebra::Rotation2::new(angle_rad);
        for vertex in &mut self.vertices {
            *vertex = rot * *vertex;
        }
    }
    fn rotation_velocity(&self) -> f32 {
        self.velocity.rotation
    }
    fn rotation_velocity_mut(&mut self) -> &mut f32 {
        &mut self.velocity.rotation
    }
}

impl<T: BodyType> Colored for Polygon<T> {
    fn change_fill_color(&mut self, color: crate::draw::Color) {
        self.color.fill = color;
    }
    fn change_hitbox_color(&mut self, color: crate::draw::Color) {
        self.color.hitbox = Some(color);
    }
    fn remove_hitbox_color(&mut self) {
        self.color.hitbox = None;
    }
}

impl<T: BodyType> Drawable for Polygon<T> {
    fn draw(&self) {
        if let Some(hitbox_color) = self.color.hitbox {
            self.vertices
                .iter()
                .map(|v| v + self.centroid.0)
                .circular_tuple_windows()
                .for_each(|(v1, v2, v3)| draw_triangle(v1, v2, v3, hitbox_color));
        }
        let border_width = if self.color.hitbox.is_some() {
            SHAPE_BORDER_WIDTH
        } else {
            0.0
        };
        self.vertices
            .iter()
            .map(|v| v - v.normalize() * border_width + self.centroid.0)
            .combinations(3)
            .for_each(|p| draw_triangle(p[0], p[1], p[2], self.color.fill));
    }
}

impl Updatable for Polygon<Static> {
    fn update(&mut self, _dt: f32) {}
}
impl Updatable for Polygon<Dynamic> {
    fn update(&mut self, dt: f32) {
        self.offset(self.linear_velocity() * dt);
        self.rotate(self.rotation_velocity() * dt);
    }
}
