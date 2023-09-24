use std::{cell::RefCell, rc::Rc};

use imgui::Ui;
use nalgebra_glm::Vec2;

use crate::{triangle_renderer::TriangleRenderer, components::{Component, ComponentType}, state::State, placer::Placer};

pub struct ComponentLayer {
    primitives: Vec<Rc<RefCell<dyn Component>>>,
    placer: Option<Placer>,
    selected: Option<Rc<RefCell<dyn Component>>>,
}

impl ComponentLayer {
    pub fn new() -> Self {
        Self { primitives: vec![], placer: None, selected: None }
    }

    pub fn update_io(&mut self, ui: &Ui, state: &State) {
        if ui.is_key_pressed(imgui::Key::Escape) {
            self.set_placer(ComponentType::None);
        }
    
        if ui.is_mouse_clicked(imgui::MouseButton::Left) && !ui.is_any_item_hovered() {
            let mouse_pos = state.window_space_to_world_space(Vec2::from_column_slice(&ui.io().mouse_pos));
            self.select(mouse_pos);
        }
    }

    pub fn draw(&self, ui: &Ui, state: &State, triangle_renderer: &mut TriangleRenderer) {
        triangle_renderer.prepare_render(state);
        for primitive in &self.primitives {
            triangle_renderer.data(primitive.borrow().get_vertices());
            triangle_renderer.render();
        }

        if let Some(placer) = &self.placer {
            let mouse_pos = state.window_space_to_world_space(Vec2::from_column_slice(&ui.io().mouse_pos));
            for primitive in &placer.get_preview(mouse_pos, state) {
                triangle_renderer.data(primitive.get_vertices());
                triangle_renderer.render();
            }
        }
    }

    fn set_selected_highlighted(&self, highlighted: bool) {
        if let Some(selected) = self.selected.as_ref() {
            selected.borrow_mut().set_highlighted(highlighted);
        }
    }

    pub fn select(&mut self, mouse_pos: Vec2) {
        self.set_selected_highlighted(false);
        self.selected = self.primitives.iter().rev().find(|primitive| primitive.borrow().hitbox_intersects(mouse_pos)).cloned();
        self.set_selected_highlighted(true);
    }

    pub fn set_placer(&mut self, component_type: ComponentType) {
        self.placer = if component_type == ComponentType::None {
            None
        } else {
            Some(Placer::new(component_type))
        };
    }

    pub fn get_placer_type(&self) -> ComponentType {
        if let Some(placer) = &self.placer {
            placer.get_component_type()
        } else {
            ComponentType::None
        }
    }

    pub fn has_placer(&self) -> bool {
        self.placer.is_some()
    }

    pub fn click_placer(&mut self, world_coords: Vec2, state: &State) {
        let placer = self.placer.as_mut().expect("Attempted to click nonexistant placer");
        placer.clicked(world_coords);
        if let Some(component) = placer.finish(state) {
            self.primitives.push(component);
        }
    }
}