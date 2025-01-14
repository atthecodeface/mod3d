use crate::types::Event;
use crate::utils::rtc::run_to_completion as rtc;

pub struct WGpuWindow {
    event_loop: winit::event_loop::EventLoop<()>,
    window: winit::window::Window,
}
impl WGpuWindow {
    pub fn new() -> Result<Self, anyhow::Error> {
        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        #[allow(unused_mut)]
        let mut builder = winit::window::WindowBuilder::new();
        let window = builder.build(&event_loop).unwrap();
        let mut size = window.inner_size();
        size.width = size.width.max(1);
        size.height = size.height.max(1);

        Ok(WGpuWindow { event_loop, window })
    }
    pub fn window(&self) -> winit::window::Window {
        self.window
    }
    pub fn prepare_viewport(&self) {}
    pub fn resize_viewport(&self, x: isize, y: isize, w: usize, h: usize) {}
    pub fn event_poll(&mut self) -> Event {
        Event::None
    }
    pub fn clear_framebuffer(&self) {}
    pub fn swap_framebuffer(&self) {}
}
