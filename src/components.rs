use std::{rc::Rc, cell::RefCell};

use nalgebra_glm::Vec2;

use crate::state::State;

use self::{triangle::Triangle, line::Line, circle::Circle, rect::Rect};

pub mod circle;
pub mod line;
pub mod rect;
pub mod triangle;

#[derive(PartialEq, Clone, Copy)]
pub enum ComponentType {
    None,
    Triangle,
    Line,
    Rect,
    Circle,
}

impl ComponentType {
    pub fn build(&self, vertices: Vec<Vec2>, state: &State) -> Rc<RefCell<dyn Component>> {
        match self {
            ComponentType::None => panic!(),
            ComponentType::Triangle => Rc::new(RefCell::new(Triangle::new(vertices, state))),
            ComponentType::Line => Rc::new(RefCell::new(Line::new(vertices, state))),
            ComponentType::Rect => Rc::new(RefCell::new(Rect::new(vertices, state))),
            ComponentType::Circle => Rc::new(RefCell::new(Circle::new(vertices, state))),
        }
    }

    pub fn preview(&self, vertices: Vec<Vec2>, state: &State) -> Vec<Box<dyn Component>> {
        match self {
            ComponentType::None => panic!(),
            ComponentType::Triangle => Triangle::get_preview(vertices, state),
            ComponentType::Line => Line::get_preview(vertices, state),
            ComponentType::Rect => Rect::get_preview(vertices, state),
            ComponentType::Circle => Circle::get_preview(vertices, state),
        }
    }

    pub fn get_vertex_count(&self) -> u32 {
        match self {
            ComponentType::None => panic!(),
            ComponentType::Triangle => 3,
            ComponentType::Line => 2,
            ComponentType::Rect => 2,
            ComponentType::Circle => 2,
        }
    }
}

pub trait ComponentPreview {
    fn get_preview(vertices: Vec<Vec2>, state: &State) -> Vec<Box<dyn Component>>;
}

pub trait Component {
    fn get_vertices(&self) -> Vec<f32>;
    fn hitbox_intersects(&self, mouse_pos: Vec2) -> bool;
    fn set_highlighted(&self, highlighted: bool);
}

fn sign(v1: Vec2, v2: Vec2, v3: Vec2) -> f32 {
    (v1.x - v3.x) * (v2.y - v3.y) - (v2.x - v3.x) * (v1.y - v3.y)
}

fn mouse_in_triangle(mouse_pos: Vec2, v1: Vec2, v2: Vec2, v3: Vec2) -> bool {
    let d1 = sign(mouse_pos, v1, v2);
    let d2 = sign(mouse_pos, v2, v3);
    let d3 = sign(mouse_pos, v3, v1);

    let has_negative = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
    let has_positive = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);

    !(has_negative && has_positive)
}