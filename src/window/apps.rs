use winit::event::{DeviceEvent, DeviceId, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::window::{WindowAttributes, WindowId};
use crate::EngineData;

pub enum LoopMode {
    Wait,
    Poll
}

pub struct ApplicationEventContext<'a> {
    pub(super) active_event_loop: &'a ActiveEventLoop,
    pub(super) engine_data: &'a mut EngineData,
}

impl ApplicationEventContext<'_> {
    pub fn active_event_loop(&self) -> &'_ ActiveEventLoop {
        self.active_event_loop
    }

    pub fn set_loop_mode(&self, loop_mode: LoopMode) {
        self.active_event_loop.set_control_flow(match loop_mode {
            LoopMode::Wait => ControlFlow::Wait,
            LoopMode::Poll => ControlFlow::Poll,
        });
    }

    pub fn create_window(&mut self, window_attributes: WindowAttributes) -> anyhow::Result<WindowId> {
        unsafe { self.engine_data.register_new_window(self.active_event_loop.create_window(window_attributes)?) }
    }

    pub fn close_window(&mut self, window: WindowId) {
        self.engine_data.close_window(window);
        if self.engine_data.window_count() == 0 {
            self.active_event_loop().exit();
        }
    }

}

// for now this is just the same as application handler but that will change *eventually*
pub trait Application {
    fn new(engine_data: &mut EngineData) -> Self;

    fn on_new_events(&mut self, _event_context: ApplicationEventContext) {}
    fn on_resumed(&mut self, _event_context: ApplicationEventContext) {}
    fn on_user_event(&mut self, _event_context: ApplicationEventContext) {}
    fn on_window_event(&mut self, _window_id: WindowId, _event: WindowEvent, _event_context: ApplicationEventContext) {}
    fn on_device_event(&mut self, _device_id: DeviceId, _event: DeviceEvent, _event_context: ApplicationEventContext) {}
    fn on_about_to_wait(&mut self, _event_context: ApplicationEventContext) {}
    fn on_suspended(&mut self, _event_context: ApplicationEventContext) {}
    fn on_exiting(&mut self, _event_context: ApplicationEventContext) {}
    fn on_memory_warning(&mut self, _event_context: ApplicationEventContext) {}
}
