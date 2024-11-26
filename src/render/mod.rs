pub mod surface;

use ash::vk::{ApplicationInfo, InstanceCreateInfo};
use ash::{khr, vk, Entry, Instance};
use ash_window::enumerate_required_extensions;
use log::{debug, error};
use winit::raw_window_handle::DisplayHandle;

pub struct VulkanInstanceData {
    entry: Entry,
    instance: Instance,
    surface_fn: khr::surface::Instance,
}

impl Drop for VulkanInstanceData {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}

impl VulkanInstanceData {
    pub unsafe fn new(display_handle: DisplayHandle) -> anyhow::Result<Self> {
        let entry = Entry::load()?;

        let application_info = ApplicationInfo::default().api_version(vk::API_VERSION_1_3);

        let extensions = enumerate_required_extensions(display_handle.as_raw())?;

        match crate::utils::c_string_slice_to_readable(extensions) {
            Ok(extensions_readable) => {
                debug!("Vulkan Instance Extensions: {:?}", extensions_readable)
            }
            Err(_) => error!("Unable to decode requested instance extension names."),
        }

        let create_info = InstanceCreateInfo::default()
            .enabled_extension_names(extensions)
            .application_info(&application_info);

        let instance = entry.create_instance(&create_info, None)?;

        let surface_fn = khr::surface::Instance::new(&entry, &instance);

        Ok(Self {
            entry,
            instance,
            surface_fn,
        })
    }
}
