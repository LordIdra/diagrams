use nalgebra_glm::{Vec2, vec2};

use crate::state::State;

use super::{Component, ComponentPreview, mouse_in_triangle};

pub struct Rect {
    from: Vec2,
    to: Vec2,
}

impl Rect {
    pub fn new(vertices: Vec<Vec2>, state: &State) -> Self {
        let from = state.world_space_to_screen_space(vertices[0]);
        let to = state.world_space_to_screen_space(vertices[1]);
        Self { from, to }
    }
}

impl ComponentPreview for Rect {
    fn get_preview(vertices: Vec<Vec2>, state: &State) -> Vec<Box<dyn Component>> {
        match vertices.len() {
            2 => vec![Box::new(Rect::new(vertices, state))],
            _ => vec![],
        }
    }
}

impl Component for Rect {
    fn get_vertices(&self) -> Vec<f32> {
        let v1 = vec2(self.from.x, self.from.y);
        let v2 = vec2(self.from.x, self.to.y);
        let v3 = vec2(self.to.x, self.from.y);
        let v4 = vec2(self.to.x, self.to.y);
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
        let v1 = vec2(self.from.x, self.from.y);
        let v2 = vec2(self.from.x, self.to.y);
        let v3 = vec2(self.to.x, self.from.y);
        let v4 = vec2(self.to.x, self.to.y);
        mouse_in_triangle(mouse_pos, v1, v2, v3) && mouse_in_triangle(mouse_pos, v2, v3, v4)
    }
}