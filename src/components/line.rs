use nalgebra_glm::{Vec2, vec2};

use crate::state::State;

use super::{Component, ComponentPreview, mouse_in_triangle};

pub struct Line {
    from: Vec2,
    to: Vec2,
}

impl Line {
    pub fn new(vertices: Vec<Vec2>, state: &State) -> Self {
        let from = state.world_space_to_screen_space(vertices[0]);
        let to = state.world_space_to_screen_space(vertices[1]);
        Self { from, to }
    }
}

impl ComponentPreview for Line {
    fn get_preview(vertices: Vec<Vec2>, state: &State) -> Vec<Box<dyn Component>> {
        match vertices.len() {
            2 => vec![Box::new(Line::new(vertices, state))],
            _ => vec![],
        }
    }
}

impl Component for Line {
    fn get_vertices(&self,) -> Vec<f32> {
        let direction = self.from - self.to;
        let perpendicular_unit = vec2(-direction.y, direction.x).normalize();
        let v1 = self.from + perpendicular_unit;
        let v2 = self.from - perpendicular_unit;
        let v3 = self.to + perpendicular_unit;
        let v4 = self.to - perpendicular_unit;
        vec![
            v1.x, v1.y,
            v2.x, v2.y,
            v3.x, v3.y,
            v2.x, v2.y,
            v3.x, v3.y,
            v4.x, v4.y,
        ]
    }

    fn hitbox_intersects(&self, mouse_pos: Vec2) -> bool {
        let direction = self.from - self.to;
        let perpendicular_unit = vec2(-direction.y, direction.x).normalize();
        let v1 = self.from + perpendicular_unit;
        let v2 = self.from - perpendicular_unit;
        let v3 = self.to + perpendicular_unit;
        let v4 = self.to - perpendicular_unit;
        mouse_in_triangle(mouse_pos, v1, v2, v3) && mouse_in_triangle(mouse_pos, v2, v3, v4)
    }
}
