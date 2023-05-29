use bevy::prelude::Vec2;
use itertools::Itertools;

pub struct ConvexPolygon {
    vertices: Vec<Vec2>,
    normals: Vec<Vec2>,
}

impl ConvexPolygon {
    pub fn new(vertices: Vec<Vec2>) -> Self {
        let normals: Vec<Vec2> = vertices
            .iter()
            .circular_tuple_windows()
            .map(|(&p1, &p2)| (p2 - p1).perp())
            .collect();
        Self { vertices, normals }
    }

    pub fn vertices(&self) -> &Vec<Vec2> {
        &self.vertices
    }

    pub fn normals(&self) -> &Vec<Vec2> {
        &self.normals
    }
}
