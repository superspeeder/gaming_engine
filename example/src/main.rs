use gaming::window::apps::{Application, ApplicationEventContext, LoopMode};
use gaming::window::{WindowAttributes, WindowEvent, WindowId};
use gaming::{run_app, EngineData};
use log::info;

#[derive(Default)]
struct MyApp {
    window1: Option<WindowId>,
    window2: Option<WindowId>,
}
impl Application for MyApp {
    fn new(_engine_data: &mut EngineData) -> Self {
        Self::default()
    }

    fn on_new_events(&mut self, _event_context: ApplicationEventContext) {}

    fn on_resumed(&mut self, mut event_context: ApplicationEventContext) {
        info!("Hello!");
        event_context.set_loop_mode(LoopMode::Poll);

        self.window1 = Some(
            event_context
                .create_window(WindowAttributes::default().with_title("Hello!"))
                .expect("Failed to create window"),
        );

        self.window2 = Some(
            event_context
                .create_window(WindowAttributes::default().with_title("Hello 2!"))
                .expect("Failed to create window"),
        );
    }

    fn on_window_event(
        &mut self,
        window_id: WindowId,
        event: WindowEvent,
        mut event_context: ApplicationEventContext,
    ) {
        match event {
            WindowEvent::CloseRequested => event_context.close_window(window_id),
            _ => (),
        }
    }
}

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init_timed();
    run_app!(MyApp)
}
