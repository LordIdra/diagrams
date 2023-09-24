use nalgebra_glm::Vec2;

use crate::state::State;

use super::{Component, line::Line, ComponentPreview, mouse_in_triangle};

pub struct Triangle {
    v1: Vec2,
    v2: Vec2,
    v3: Vec2,
}

impl Triangle {
    pub fn new(vertices: Vec<Vec2>, state: &State) -> Self {
        Self {
            v1: state.world_space_to_screen_space(vertices[0]),
            v2: state.world_space_to_screen_space(vertices[1]),
            v3: state.world_space_to_screen_space(vertices[2]),
        }
    }
}

impl ComponentPreview for Triangle {
    fn get_preview(vertices: Vec<Vec2>, state: &State) -> Vec<Box<dyn Component>> {
        match vertices.len() {
            2 => vec![Box::new(Line::new(vertices, state))],
            3 => vec![Box::new(Triangle::new(vertices, state))],
            _ => vec![],
        }
    }
}

impl Component for Triangle {
    fn get_vertices(&self) -> Vec<f32> {
        vec![
            self.v1.x, self.v1.y,
            self.v2.x, self.v2.y,
            self.v3.x, self.v3.y,
        ]
    }

    fn hitbox_intersects(&self, mouse_pos: Vec2) -> bool {
        mouse_in_triangle(mouse_pos, self.v1, self.v2, self.v3)
    }    
}