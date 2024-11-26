use std::collections::HashMap;
use std::rc::Rc;
use winit::raw_window_handle::HasDisplayHandle;
use winit::window::{Window, WindowId};
use crate::render::surface::SurfaceData;
use crate::render::VulkanInstanceData;
use crate::window::EventLoop;

pub mod window;
pub mod render;
pub mod utils;

pub struct WindowData {
    pub window: Window,
    pub surface_data: SurfaceData,
}

pub struct EngineData {
    windows: HashMap<WindowId, WindowData>,
    vulkan_instance_data: Rc<VulkanInstanceData>,
}

impl EngineData {
    pub unsafe fn new(event_loop: &EventLoop) -> anyhow::Result<Self> {
        Ok(Self {
            windows: HashMap::new(),
            vulkan_instance_data: Rc::new(VulkanInstanceData::new(event_loop.display_handle()?)?),
        })
    }

    pub(crate) unsafe fn register_new_window(&mut self, window: Window) -> anyhow::Result<WindowId> {
        let id = window.id();
        let surface_data = SurfaceData::new(self.vulkan_instance_data.clone(), &window)?;
        _ = self.windows.insert(id, WindowData { window, surface_data });
        Ok(id)
    }

    pub(crate) fn close_window(&mut self, id: WindowId) {
        self.windows.remove(&id);
    }

    pub(crate) fn window_count(&self) -> usize {
        self.windows.len()
    }
}