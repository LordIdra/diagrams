use imgui::{StyleColor, StyleVar, WindowFlags, Condition, Ui, ColorStackToken};

use crate::{icons, component_layer::ComponentLayer, components::ComponentType};

const WINDOW_BG: [f32; 4] = [0.2, 0.2, 0.2, 0.7];
const COMPONENT_BUTTON_NORMAL: [f32; 4] = [0.3, 0.3, 0.3, 1.0];
const COMPONENT_BUTTON_HOVER:  [f32; 4] = [0.5, 0.4, 0.2, 1.0];
const COMPONENT_BUTTON_ACTIVE: [f32; 4] = [0.8, 0.5, 0.2, 1.0];

fn component_button(ui: &Ui, component_storage: &mut ComponentLayer, component_type: ComponentType, icon: char) {
    let _t_button_normal: ColorStackToken;
    let _t_button_hover: ColorStackToken;
    if component_storage.get_placer_type() == component_type {
        _t_button_normal = ui.push_style_color(StyleColor::Button, COMPONENT_BUTTON_ACTIVE);
        _t_button_hover = ui.push_style_color(StyleColor::ButtonHovered, COMPONENT_BUTTON_ACTIVE);
    }

    if ui.button(icon.to_string()) {
        component_storage.set_placer(component_type);
    }
}

fn draw_components(ui: &Ui, component_storage: &mut ComponentLayer) {
    let _t_window_bg = ui.push_style_color(StyleColor::WindowBg, WINDOW_BG);
    let _t_button_normal= ui.push_style_color(StyleColor::Button, COMPONENT_BUTTON_NORMAL);
    let _t_button_hovered = ui.push_style_color(StyleColor::ButtonHovered, COMPONENT_BUTTON_HOVER);
    let _t_button_active = ui.push_style_color(StyleColor::ButtonActive, COMPONENT_BUTTON_ACTIVE);
    let _t_window_border_size = ui.push_style_var(StyleVar::WindowBorderSize(0.0));

    ui.window("components")
        .flags(WindowFlags::NO_TITLE_BAR | WindowFlags::NO_RESIZE | WindowFlags::NO_MOVE)
        .position([20.0, 20.0], Condition::Always)
        .build(|| {
            let horizontal_spacing = 4.0;

            component_button(ui, component_storage, ComponentType::None, icons::ICON_CURSOR_DEFAULT);
            
            ui.same_line_with_spacing(0.0, horizontal_spacing);
            component_button(ui, component_storage, ComponentType::Line, icons::ICON_VECTOR_LINE);

            ui.same_line_with_spacing(0.0, horizontal_spacing);
            component_button(ui, component_storage, ComponentType::Rect, icons::ICON_RECTANGLE_OUTLINE);

            ui.same_line_with_spacing(0.0, horizontal_spacing);
            component_button(ui, component_storage, ComponentType::Circle, icons::ICON_CIRCLE_OUTLINE);

            component_button(ui, component_storage, ComponentType::Triangle, icons::ICON_TRIANGLE_OUTLINE);
    });
}

pub fn draw(ui: &Ui, component_storage: &mut ComponentLayer) {
    ui.show_demo_window(&mut true);
    draw_components(ui, component_storage);
}