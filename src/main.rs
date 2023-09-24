use component_layer::ComponentLayer;
use glutin::{event_loop::EventLoop, WindowedContext, event::{Event, WindowEvent}};
use imgui::{FontAtlas, FontSource, FontId, FontConfig, FontGlyphRanges, Context};
use imgui_glow_renderer::AutoRenderer;
use imgui_winit_support::WinitPlatform;
use nalgebra_glm::vec2;
use state::{State, INITIAL_WINDOW_SIZE};
use triangle_renderer::TriangleRenderer;

pub type Window = WindowedContext<glutin::PossiblyCurrent>;

const MDI_RANGE: [u32; 3] = [icons::ICON_MIN as u32, icons::ICON_MAX as u32, 0];


mod component_layer;
mod icons;
mod placer;
mod components;
mod ui;
mod state;
mod triangle_renderer;
mod underlay_layer;

fn init_window(event_loop: &EventLoop<()>) -> Window {
    let window = glutin::window::WindowBuilder::new()
        .with_title("Oh no")
        .with_inner_size(glutin::dpi::LogicalSize::new(INITIAL_WINDOW_SIZE[0], INITIAL_WINDOW_SIZE[1]));
    let window = glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(16)
        .build_windowed(window, event_loop)
        .expect("could not create window");
    unsafe { window.make_current().expect("could not make window context current") }
}

fn init_winit(window: &Window, imgui_context: &mut Context) -> WinitPlatform {
    let mut winit_platform = WinitPlatform::init(imgui_context);
    winit_platform.attach_window(imgui_context.io_mut(), window.window(), imgui_winit_support::HiDpiMode::Rounded);
    imgui_context.set_ini_filename(None);
    imgui_context.io_mut().font_global_scale = (1.0 / winit_platform.hidpi_factor()) as f32;
    winit_platform
}

fn init_fonts(fonts: &mut FontAtlas) -> FontId {
    fonts.add_font(&[
        FontSource::TtfData { 
            data: include_bytes!("../resources/fonts/Inter-Regular.otf"), 
            size_pixels: 16.0, 
            config: None,
        },
        FontSource::TtfData { 
            data: include_bytes!("../resources/fonts/material-design-icons.ttf"), 
            size_pixels: 16.0, 
            config: Some(FontConfig { 
                glyph_ranges: FontGlyphRanges::from_slice(&MDI_RANGE),
                ..Default::default()
            }),
        }
    ])
}

fn init_glow_context(window: &Window) -> glow::Context {
    unsafe { glow::Context::from_loader_function(|s| window.get_proc_address(s).cast()) }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = init_window(&event_loop);
    let mut imgui_context = imgui::Context::create();
    let mut winit_platform = init_winit(&window, &mut imgui_context);
    let font = init_fonts(imgui_context.fonts());
    let mut ui_renderer = AutoRenderer::initialize(init_glow_context(&window), &mut imgui_context).expect("failed to create renderer");
    let mut triangle_renderer = TriangleRenderer::new(ui_renderer.gl_context());
    let mut state = State::new();
    let mut component_storage = ComponentLayer::new();

    event_loop.run(move |e, _, control_flow| {
        match e {
            Event::NewEvents(_) => state.update_time(&mut imgui_context),

            Event::MainEventsCleared => {
                winit_platform.prepare_frame(imgui_context.io_mut(), window.window()).unwrap();
                window.window().request_redraw();
            }

            Event::RedrawRequested(_) => {
                let mouse_pos = vec2(imgui_context.io().mouse_pos[0], imgui_context.io().mouse_pos[1]);
                {
                    let ui = imgui_context.new_frame();
                    state.update_input(ui, &mut component_storage, mouse_pos);
                    let _t_font = ui.push_font(font);
                    ui::draw(ui, &mut component_storage);
                    component_storage.update_io(ui, &state);
                    winit_platform.prepare_render(ui, window.window());
                    component_storage.draw(ui, &state, &mut triangle_renderer);
                }
                ui_renderer.render(imgui_context.render()).expect("error rendering imgui");
                window.swap_buffers().unwrap();
            },

            Event::WindowEvent { .. } => {
                winit_platform.handle_event(imgui_context.io_mut(), window.window(), &e);
                if let Event::WindowEvent { event, .. } = e {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = glutin::event_loop::ControlFlow::Exit,
                        WindowEvent::Resized(size) => state.update_size(size, &mut ui_renderer),
                        _ => ()
                    }
                }
            }
            e => winit_platform.handle_event(imgui_context.io_mut(), window.window(), &e),
        }
    });
}
