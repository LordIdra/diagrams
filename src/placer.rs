trait Increment {
    fn increment(&mut self);
}

use std::{cell::RefCell, rc::Rc};

use nalgebra_glm::Vec2;

use crate::{components::{ComponentType, Component}, state::State};

pub struct Placer {
    component_type: ComponentType,
    vertex_count: u32,
    click_stage: u32,
    vertices: Vec<Vec2>,
}

impl Placer {
    pub fn new(component_type: ComponentType) -> Self {
        Self { 
            component_type, 
            vertex_count: component_type.get_vertex_count(),
            click_stage: 0,
            vertices: vec![]
        }
    }

    pub fn get_component_type(&self) -> ComponentType {
        self.component_type
    }

    pub fn clicked(&mut self, world_position: Vec2) {
        if self.click_stage == self.vertex_count {
            self.click_stage = 0;
            self.vertices.clear();
        }

        self.click_stage += 1;
        self.vertices.push(world_position);
    }

    pub fn finish(&self, state: &State) -> Option<Rc<RefCell<dyn Component>>> {
        if self.click_stage == self.vertex_count {
            Some(self.component_type.build(self.vertices.clone(), state))
        } else {
            None
        }
    }

    pub fn get_preview(&self, mouse_pos: Vec2, state: &State) -> Vec<Box<dyn Component>> {
        let mut preview_vertices = self.vertices.clone();
        preview_vertices.push(mouse_pos);
        self.component_type.preview(preview_vertices, state)
    }
}