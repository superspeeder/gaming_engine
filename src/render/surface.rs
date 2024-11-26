use std::rc::Rc;
use ash::vk;
use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::window::Window;
use crate::render::VulkanInstanceData;

pub struct SurfaceData {
    vulkan_instance_data: Rc<VulkanInstanceData>,
    surface: vk::SurfaceKHR,
}

impl SurfaceData {
    pub unsafe fn new(vulkan_instance_data: Rc<VulkanInstanceData>, window: &Window) -> anyhow::Result<Self> {
        let surface = ash_window::create_surface(&vulkan_instance_data.entry, &vulkan_instance_data.instance, window.display_handle().unwrap().as_raw(), window.window_handle().unwrap().as_raw(), None)?;

        Ok(Self {
            vulkan_instance_data,
            surface,
        })
    }
}

impl Drop for SurfaceData {
    fn drop(&mut self) {
        unsafe { self.vulkan_instance_data.surface_fn.destroy_surface(self.surface, None) };
    }
}