use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use js_sys::WebAssembly;

use crate::common_funcs as cf;
use crate::shaders;

pub struct Color2D {
    program: WebGlProgram,
    rect_vertice_buffer: WebGlBuffer,
    rect_vertice_array_len: usize,
    u_color: WebGlUniformLocation,
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation
}

impl Color2D {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let program: WebGlProgram = cf::link_program(
            &gl,
            shaders::vertex::color_2d::SHADER,
            shaders::fragment::color_2d::SHADER
        ).unwrap();

        // counter clockwise triangle gives us a rectangle
        // webgl renders everything in triangles
        let vertices_rect: [f32; 12] = [
            0., 1.,//x, y
            0., 0.,
            1., 1.,
            1., 1.,
            0.,0.,
            1., 0.,
        ];

        //get this info to webgl
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

        //pointer to array
        let vertices_location = vertices_rect.as_ptr() as u32 / 4;

        //converte this array to javascript form
        let vert_array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(
                vertices_location,
                vertices_location + vertices_rect.len() as u32
            );

        let buffer_rect = gl.create_buffer().ok_or("failed to create buffer rect").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer_rect));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);

        
        Color2D{
            u_color: gl.get_uniform_location(&program, "uColor").unwrap(),
            u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
            u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),//shaders -> color_2d
            rect_vertice_array_len: vertices_rect.len(),
            rect_vertice_buffer: buffer_rect,
            program,
        }
    }


    pub fn render(
        &self,
        gl: &WebGlRenderingContext,
        bottom: f32, top: f32, left: f32, right: f32,
        canvas_height: f32, canvas_width: f32,
    ) {
        gl.use_program(Some(&self.program));
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.rect_vertice_buffer));
        gl.vertex_attrib_pointer_with_i32(
            0,//how many color gradients
            2,//how many coordinates
            GL::FLOAT,//value type
            false,//normalize
            0,//stride
            0,//offset
        );
        gl.enable_vertex_attrib_array(0);

        //shader -> uniform vec4 uColor;
        gl.uniform4f(Some(&self.u_color), 0., 0.5, 0.5, 1.0);//r g b a
        gl.uniform1f(Some(&self.u_opacity), 1.0);//opacity info

        let tx = 2.0 * left / canvas_width - 1.0;
        let ty = 2.0 * bottom / canvas_height - 1.0;
        let translation_matrix: [f32; 16] = cf::translation_matrix(tx, ty, 0.0);

        let sx = 2.0 * (right - left) / canvas_width;
        let sy = 2.0 * (top - bottom) / canvas_height;
        let scaling_matrix: [f32; 16] = cf::scaling_matrix(sx, sy, 0.0);

        let transform_mat: [f32; 16] = cf::mult_matrix_4(scaling_matrix, translation_matrix);

        //update webgl with this info
        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &transform_mat);

        gl.draw_arrays(GL::TRIANGLES, 0, (self.rect_vertice_array_len / 2) as i32);//2 here is x & y since 2D
    }
}