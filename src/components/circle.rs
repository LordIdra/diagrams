use std::f32::consts::PI;

use nalgebra_glm::{Vec2, vec2};

use crate::state::State;

use super::{Component, ComponentPreview};

pub struct Circle {
    center: Vec2,
    radius: f32,
}

impl Circle {
    pub fn new(vertices: Vec<Vec2>, state: &State) -> Self {
        let center = state.world_space_to_screen_space(vertices[0]);
        let to = state.world_space_to_screen_space(vertices[1]);
        let radius = (center - to).magnitude();
        Self { center, radius }
    }
}

impl ComponentPreview for Circle {
    fn get_preview(vertices: Vec<Vec2>, state: &State) -> Vec<Box<dyn Component>> {
        match vertices.len() {
            2 => vec![Box::new(Circle::new(vertices, state))],
            _ => vec![],
        }
    }
}

impl Component for Circle {
    fn get_vertices(&self) -> Vec<f32> {
        let mut vertices = vec![];
        let mut previous_position = self.center + vec2(self.radius, 0.0);
        let circumference = 2.0 * PI * self.radius;
        let sides = (circumference / 4.0) as i32;
        for i in 0..=sides {
            let angle = (i as f32 / sides as f32) * 2.0 * PI;
            let new_position = self.center + vec2(self.radius * f32::sin(angle), self.radius * f32::cos(angle));
            vertices.push(self.center.x);
            vertices.push(self.center.y);
            vertices.push(previous_position.x);
            vertices.push(previous_position.y);
            vertices.push(new_position.x);
            vertices.push(new_position.y);
            previous_position = new_position;
        }
        vertices
    }

    fn hitbox_intersects(&self, mouse_pos: Vec2) -> bool {
        (self.center - mouse_pos).magnitude() < self.radius
    }
}
