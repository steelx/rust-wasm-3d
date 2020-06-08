extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

mod gl_setup;
mod shaders;
mod gl_programs;
mod common_funcs;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn say_hello_from_rust() {
    log("Hello from Rust World!");
}

#[wasm_bindgen]
pub struct GameClient {
    gl: WebGlRenderingContext,
    program_color_2d: gl_programs::Color2D,
}

impl Default for GameClient {
    fn default() -> Self {
        GameClient::new()
    }
}

#[wasm_bindgen]
impl GameClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        log("inside new()");
        let gl: WebGlRenderingContext = match gl_setup::initialize_webgl_context() {
            Ok(v) => v,
            Err(_) => panic!("Failed to initialize webgl context"),
        };


        GameClient{
            program_color_2d: gl_programs::Color2D::new(&gl),
            gl,
        }
    }

    pub fn update(&mut self, _dt: f32, _width: f32, _height: f32) -> Result<(), JsValue> {
        Ok(())
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT);

        self.program_color_2d.render(&self.gl, 0., 10., 0., 10., 10., 10.);
    }
}