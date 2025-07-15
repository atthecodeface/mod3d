//a Imports
use crate::types::Event;

//a SdlWindow
//tp SdlWindow
pub struct SdlWindow {
    #[allow(dead_code)]
    sdl: sdl2::Sdl,
    #[allow(dead_code)]
    window: sdl2::video::Window,
    #[allow(dead_code)]
    gl_context: sdl2::video::GLContext,
    event_pump: sdl2::EventPump,
}

//ip SdlWindow
impl SdlWindow {
    //cp new
    pub fn new() -> Result<Self, anyhow::Error> {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 1);

        let window = video_subsystem
            .window("Game", 900, 700)
            .opengl()
            .resizable()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().map_err(anyhow::Error::msg)?;
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
        let event_pump = sdl.event_pump().unwrap();
        Ok(SdlWindow {
            sdl,
            window,
            gl_context,
            event_pump,
        })
    }

    //mp prepare_viewport
    pub fn prepare_viewport(&self) {
        unsafe {
            let (w, h) = self.window.drawable_size();
            let w = w as i32;
            let h = h as i32;
            gl::Viewport(0, 0, w, h);
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }

        mod3d_gl::opengl_utils::check_errors().unwrap();
        // These are not flags
        // unsafe { gl::Enable(gl::CULL_FACE) };
        unsafe { gl::Enable(gl::DEPTH_TEST) };
        mod3d_gl::opengl_utils::check_errors().unwrap();
    }

    //mp resize_viewport
    pub fn resize_viewport(&self, x: isize, y: isize, w: usize, h: usize) {
        unsafe { gl::Viewport(x as i32, y as i32, w as i32, h as i32) };
    }

    //mp event_poll
    pub fn event_poll(&mut self) -> Event {
        while let Some(e) = self.event_pump.poll_event() {
            match e {
                sdl2::event::Event::Quit { .. } => {
                    return Event::Quit;
                }
                sdl2::event::Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    let w = w.max(0) as usize;
                    let h = h.max(0) as usize;
                    return Event::ResizeWindow(w, h);
                }
                _ => (),
            }
        }
        Event::None
    }

    //mp clear_framebuffer
    pub fn clear_framebuffer(&self) {
        mod3d_gl::opengl_utils::check_errors().unwrap();

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    //mp swap_framebuffer
    pub fn swap_framebuffer(&self) {
        mod3d_gl::opengl_utils::check_errors().unwrap();

        self.window.gl_swap_window();
    }
}
