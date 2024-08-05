//a Imports
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

mod inner;
use inner::Inner;
// pub mod shader;

// mod base_shader;
mod model;
mod objects;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (
        #[allow(unused_unsafe)]
        unsafe { $crate::log(&format_args!($($t)*).to_string())}
    )
}

//a CanvasWebgl - the external interface
//tp CanvasWebgl
/// A paint module that is attached to a Canvas element in an HTML
/// document, which uses mouse events etc to provide a simple paint
/// program
#[wasm_bindgen]
pub struct CanvasWebgl {
    inner: Rc<Inner>,
}

//ip CanvasWebgl
#[wasm_bindgen]
impl CanvasWebgl {
    //fp new
    /// Create a new CanvasWebgl attached to a Canvas HTML element,
    /// adding events to the canvas that provide the paint program
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Result<CanvasWebgl, JsValue> {
        console_error_panic_hook::set_once();
        let inner = Inner::new(canvas)?;
        Ok(Self { inner })
    }

    //mp shutdown
    /// Shut down the CanvasWebgl, removing any event callbacks for the canvas
    pub fn shutdown(&self) -> Result<(), JsValue> {
        self.inner.shutdown()
    }

    //mp add_file
    pub fn add_file(&mut self, filename: &str, data: JsValue) -> Result<(), JsValue> {
        let data = js_sys::Uint8Array::new(&data);
        let data = data.to_vec();
        Ok(std::rc::Rc::get_mut(&mut self.inner)
            .unwrap()
            .add_file(filename, data))
    }

    //mp create_f
    pub fn create_f(&mut self, shader: &str, glb: &str) -> Result<(), JsValue> {
        Ok(std::rc::Rc::get_mut(&mut self.inner)
            .unwrap()
            .create_f(shader, glb, &["0"])?)
    }

    //mp fill
    /// Fill
    pub fn fill(&mut self) -> Result<(), JsValue> {
        std::rc::Rc::get_mut(&mut self.inner).unwrap().fill();
        Ok(())
    }

    //zz All done
}
