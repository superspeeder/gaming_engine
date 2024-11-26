use winit::application::ApplicationHandler;
use winit::error::EventLoopError;
use winit::event::StartCause;
use winit::event_loop::ActiveEventLoop;
use crate::window::apps::{Application, ApplicationEventContext};

pub use winit::window::{Window, WindowAttributes, WindowId};
pub use winit::event::{DeviceEvent, DeviceId, WindowEvent};
pub use winit::event_loop::ControlFlow;
use crate::EngineData;

pub mod apps;

#[repr(transparent)]
pub struct EventLoop(winit::event_loop::EventLoop<()>);


impl EventLoop {
    pub fn new() -> Result<EventLoop, EventLoopError> {
        winit::event_loop::EventLoop::builder().build().map(|e| EventLoop(e))
    }

    pub unsafe fn run<A: Application>(self) -> anyhow::Result<()> {
        let mut engine_data = EngineData::new(&self)?;
        let app = A::new(&mut engine_data);
        let mut container = ApplicationContainer{ engine_data: Some(engine_data), app };
        self.0.run_app(&mut container).map_err(anyhow::Error::from)

    }
}


impl std::ops::Deref for EventLoop {
    type Target = winit::event_loop::EventLoop<()>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for EventLoop {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Into<winit::event_loop::EventLoop<()>> for EventLoop {
    fn into(self) -> winit::event_loop::EventLoop<()> {
        self.0
    }
}

struct ApplicationContainer<A: Application> {
    engine_data: Option<EngineData>,
    app: A,
}

impl <A: Application> std::ops::Deref for ApplicationContainer<A> {
    type Target = A;

    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

impl <A: Application> std::ops::DerefMut for ApplicationContainer<A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.app
    }
}


impl<A: Application> ApplicationHandler for ApplicationContainer<A> {
    fn new_events(&mut self, active_event_loop: &ActiveEventLoop, _cause: StartCause) {
        let mut engine_data = self.engine_data.take().unwrap();
        self.on_new_events(ApplicationEventContext { active_event_loop, engine_data: &mut engine_data });
        self.engine_data = Some(engine_data);
    }

    fn resumed(&mut self, active_event_loop: &ActiveEventLoop) {
        let mut engine_data = self.engine_data.take().unwrap();
        self.on_resumed(ApplicationEventContext { active_event_loop, engine_data: &mut engine_data });
        self.engine_data = Some(engine_data);
    }

    fn user_event(&mut self, active_event_loop: &ActiveEventLoop, _event: ()) {
        let mut engine_data = self.engine_data.take().unwrap();
        self.on_user_event(ApplicationEventContext { active_event_loop, engine_data: &mut engine_data });
        self.engine_data = Some(engine_data);
    }

    fn window_event(&mut self, active_event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        let mut engine_data = self.engine_data.take().unwrap();
        self.on_window_event(window_id, event, ApplicationEventContext { active_event_loop, engine_data: &mut engine_data });
        self.engine_data = Some(engine_data);
    }

    fn device_event(&mut self, active_event_loop: &ActiveEventLoop, device_id: DeviceId, event: DeviceEvent) {
        let mut engine_data = self.engine_data.take().unwrap();
        self.on_device_event(device_id, event, ApplicationEventContext { active_event_loop, engine_data: &mut engine_data });
        self.engine_data = Some(engine_data);
    }

    fn about_to_wait(&mut self, active_event_loop: &ActiveEventLoop) {
        let mut engine_data = self.engine_data.take().unwrap();
        self.on_about_to_wait(ApplicationEventContext { active_event_loop, engine_data: &mut engine_data });
        self.engine_data = Some(engine_data);
    }

    fn suspended(&mut self, active_event_loop: &ActiveEventLoop) {
        let mut engine_data = self.engine_data.take().unwrap();
        self.on_suspended(ApplicationEventContext { active_event_loop, engine_data: &mut engine_data });
        self.engine_data = Some(engine_data);
    }

    fn exiting(&mut self, active_event_loop: &ActiveEventLoop) {
        let mut engine_data = self.engine_data.take().unwrap();
        self.on_exiting(ApplicationEventContext { active_event_loop, engine_data: &mut engine_data });
        self.engine_data = Some(engine_data);
    }

    fn memory_warning(&mut self, active_event_loop: &ActiveEventLoop) {
        let mut engine_data = self.engine_data.take().unwrap();
        self.on_memory_warning(ApplicationEventContext { active_event_loop, engine_data: &mut engine_data });
        self.engine_data = Some(engine_data);
    }
}


#[macro_export]
macro_rules! run_app {
    ($t:ty) => (unsafe { $crate::window::EventLoop::new()?.run::<$t>().map_err(anyhow::Error::from) })
}