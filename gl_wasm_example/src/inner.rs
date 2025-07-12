//a Imports
use std::cell::RefCell;
use std::collections::HashMap;
use std::pin::Pin;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use web_sys::WebGl2RenderingContext;

use mod3d_gl::Model3DWebGL;

// use crate::shader;

//a Inner (and ClosureSet)
//ti F
struct F {
    /// base is pinned so that instantiable and instances can refer to
    /// its buffer contents
    base: Pin<Box<crate::model::Base<Model3DWebGL>>>,

    /// instantiable *must not* leak outside of F without breaking safety
    instantiable: crate::model::Instantiable<'static, Model3DWebGL>,

    /// instances *must not* leak outside of F without breaking
    /// safety, as it refers to base
    instances: RefCell<crate::model::Instances<'static, Model3DWebGL>>,

    /// State of the model
    game_state: RefCell<crate::model::GameState>,
}

//tp Inner
/// The actual CanvasArt paint structure, with canvas and rendering
/// context, state, and closures
pub struct Inner {
    /// We hold on to the canvas - this might be needed to keep
    /// control of the context, or it might just be a hangover from
    /// when this module used event listeners
    #[allow(dead_code)]
    canvas: HtmlCanvasElement,
    model3d: Model3DWebGL,
    files: HashMap<String, Vec<u8>>,
    f: Option<F>,
}

//ip Inner
impl Inner {
    //fp new
    /// Create a new Inner canvas paint structure given a Canvas element
    ///
    /// Does not add the event listeners (for no really good reason)
    pub fn new(canvas: HtmlCanvasElement) -> Result<Rc<Self>, JsValue> {
        let context = canvas
            .get_context("webgl2")?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()?;

        let model3d = Model3DWebGL::new(context);
        let files = Default::default();

        let f = None;

        let inner = Self {
            canvas,
            model3d,
            files,
            f,
        };

        Ok(inner.into())
    }

    //mp shutdown
    /// Remove all the event listeneres added (in the ClosureSet) and
    /// drop the closures
    ///
    /// This should be called prior to dropping the Inner so that it is not leaked.
    pub fn shutdown(&self) -> Result<(), JsValue> {
        Ok(())
    }

    //mp fill
    /// Fill the canvas with transparent black
    pub fn fill(&mut self) {
        self.draw();
    }

    //mp create_f
    pub fn create_f(
        &mut self,
        shader_filename: &str,
        glb: &str,
        node_names: &[&str],
    ) -> Result<(), String> {
        let scale = 0.6;
        if self.f.is_some() {
            return Err("Already created".to_string());
        }
        let shader = self
            .files
            .get(shader_filename)
            .ok_or_else(|| format!("Failed to find shader file {shader_filename}"))?;
        let shader = std::str::from_utf8(shader).map_err(|_| "Bad UTF8 for shader".to_string())?;
        let shader_program_desc: mod3d_gl::PipelineDesc =
            serde_json::from_str(shader).map_err(|e| format!("Failed to parse shader desc {e}"))?;

        let m = Box::new(crate::model::Base::new(
            &mut self.model3d,
            &self.files,
            Box::new(shader_program_desc),
            glb,
            node_names,
        )?);
        let m = Box::into_pin(m);

        let f = {
            // # Safety
            //
            // m is pinned and stored in F; m therefore lives as long as F
            //
            // Other items within F can safely refer to m, provided
            // they cannot be extracted from F.
            let m_ref = unsafe {
                std::mem::transmute::<
                    &crate::model::Base<Model3DWebGL>,
                    &'static crate::model::Base<Model3DWebGL>,
                >(&*m)
            };
            let instantiable = m_ref.make_instantiable(&mut self.model3d)?;
            let instances = m_ref.make_instances().into();
            let game_state = crate::model::GameState::new(scale).into();
            F {
                base: m,
                instantiable,
                instances,
                game_state,
            }
        };
        self.f = Some(f);
        Ok(())
    }

    pub fn add_file(&mut self, filename: &str, data: Vec<u8>) {
        self.files.insert(filename.to_string(), data);
    }

    //mp draw
    pub fn draw(&mut self) {
        // self.model3d.enable(WebGl2RenderingContext::CULL_FACE);
        self.model3d.enable(WebGl2RenderingContext::DEPTH_TEST);
        self.model3d.clear_color(0.0, 0.0, 0.0, 1.0);
        self.model3d.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        let model3d = &mut self.model3d;
        if let Some(ref mut f) = self.f {
            let base = &f.base;
            let mut game_state = f.game_state.borrow_mut();
            let instantiable = &f.instantiable;
            let mut instances = f.instances.borrow_mut();
            base.update(model3d, &mut game_state, instantiable, &mut instances);
        }
    }

    //zz All done
}
