use std::rc::Rc;

use glow::{Context, HasContext, MULTISAMPLE};
use nalgebra_glm::{Mat4, vec3};

use crate::state::State;

use self::{shader_program::ShaderProgram, vertex_array_object::{VertexArrayObject, VertexAttribute}};

mod shader_program;
mod vertex_array_object;

pub struct TriangleRenderer {
    gl: Rc<Context>,
    program: ShaderProgram,
    vertex_array_object: VertexArrayObject,
}

impl TriangleRenderer {
    pub fn new(gl: &Rc<Context>) -> Self {
        unsafe { gl.enable(MULTISAMPLE) }
        let program = ShaderProgram::new(gl.clone(), include_str!("../resources/shaders/geometry.vert"), include_str!("../resources/shaders/geometry.frag"));
        let vertex_array_object = VertexArrayObject::new(gl.clone(), vec![
            VertexAttribute { index: 0, count: 2 },
        ]);
        TriangleRenderer { gl: gl.clone(), program, vertex_array_object }
    }

    pub fn data(&mut self, data: Vec<f32>) {
        self.vertex_array_object.data(data);
    }

    pub fn prepare_render(&self, state: &State) {
        unsafe { self.gl.clear(glow::COLOR_BUFFER_BIT) };
        let matrix = Mat4::identity();
        let matrix = nalgebra_glm::scale(&matrix, &vec3(state.get_zoom(), state.get_zoom(), 1.0));
        let matrix = nalgebra_glm::translate(&matrix, &vec3(
            -state.world_space_to_screen_space(state.get_translation()).x,
            -state.world_space_to_screen_space(state.get_translation()).y,
            0.0));
        self.program.use_program();
        self.program.uniform_mat4("matrix", matrix.as_slice());
    }

    pub fn render(&self) {
        self.vertex_array_object.draw();
    }
}