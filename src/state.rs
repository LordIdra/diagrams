

use std::time::Instant;

use glow::HasContext;
use glutin::dpi::PhysicalSize;
use imgui::{MouseButton, Context, Ui};
use imgui_glow_renderer::AutoRenderer;
use nalgebra_glm::{Vec2, vec2};

use crate::component_layer::ComponentLayer;

pub const INITIAL_WINDOW_SIZE: [u32; 2] = [1024, 1024];
const ZOOM_SENSITIVITY: f32 = 0.15;

pub struct State {
    last_frame: Instant,
    window_size: Vec2,
    is_right_click_dragging: bool,
    translation: Vec2,
    zoom: f32,
}

impl State {
    pub fn new() -> Self {
        State { 
            last_frame: Instant::now(), 
            window_size: vec2(INITIAL_WINDOW_SIZE[0] as f32, INITIAL_WINDOW_SIZE[1] as f32), 
            is_right_click_dragging: false,
            translation: vec2(0.0, 0.0), 
            zoom: 1.0  
        }
    }

    pub fn update_time(&mut self, imgui_context: &mut Context) {
        let now = Instant::now();
        imgui_context.io_mut().update_delta_time(now.duration_since(self.last_frame));
        self.last_frame = now;
    }

    pub fn update_input(&mut self, ui: &Ui, component_storage: &mut ComponentLayer, mouse_pos: Vec2) {
        if ui.is_mouse_clicked(MouseButton::Right) && !ui.is_any_item_hovered() {
            self.is_right_click_dragging = true;
        }

        if ui.is_mouse_released(MouseButton::Right) {
            self.is_right_click_dragging = false;
        }

        if ui.is_mouse_clicked(MouseButton::Left) && !ui.is_any_item_hovered() && component_storage.has_placer() {
            component_storage.click_placer(self.window_space_to_world_space(mouse_pos), self);
        }

        if self.is_right_click_dragging() {
            let world_coords = self.window_space_to_world_space_drag(ui.mouse_drag_delta_with_button(MouseButton::Right));
            self.translate(world_coords);
            ui.reset_mouse_drag_delta(MouseButton::Right);
        }

        let mouse_delta = ui.io().mouse_wheel;
        if mouse_delta != 0.0 {
            let world_coords = self.window_space_to_world_space_zoom(ui.io().mouse_pos);
            self.zoom(mouse_delta, world_coords);
        }
    }

    pub fn update_size(&mut self, size: PhysicalSize<u32>, ui_renderer: &mut AutoRenderer) {
        self.window_size = vec2(size.width as f32, size.height as f32);
        unsafe { 
            ui_renderer.gl_context().viewport(
                0, 0,
                self.window_size[0] as i32, self.window_size[1] as i32); 
        }
    }

    pub fn translate(&mut self, amount_world_space: Vec2) {
        self.translation -= amount_world_space / self.zoom;
    }

    pub fn get_translation(&self) -> Vec2 {
        self.translation
    }

    pub fn zoom(&mut self, amount: f32, mouse_world_position: Vec2) {
        let new_zoom = self.zoom * (1.0 + ZOOM_SENSITIVITY * amount);
        let delta_zoom = (self.zoom - new_zoom) / new_zoom;
        self.translate(mouse_world_position * delta_zoom);
        self.zoom = new_zoom;
    }

    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }

    pub fn window_space_to_world_space_drag(&self, coords: [f32; 2]) -> Vec2 {
        vec2(2.0 * (coords[0]), -2.0 * (coords[1]))
    }

    pub fn window_space_to_world_space_zoom(&self, coords: [f32; 2]) -> Vec2 {
        vec2(
             2.0 * (coords[0] - (self.window_size.x / 2.0)), 
            -2.0 * (coords[1] - (self.window_size.y / 2.0)))
    }

    pub fn window_space_to_world_space(&self, coords: Vec2) -> Vec2 {
        let coords = 2.0 * (coords - (self.window_size / 2.0)) / self.zoom;
        vec2(self.translation.x + coords.x, self.translation.y - coords.y)
    }

    pub fn world_space_to_screen_space(&self, coords: Vec2) -> Vec2 {
        vec2(coords.x / self.window_size[0], coords.y / self.window_size[1])
    }

    pub fn is_right_click_dragging(&self) -> bool {
        self.is_right_click_dragging
    }
}