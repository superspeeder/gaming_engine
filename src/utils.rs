use std::ffi::{c_char, CStr};
pub use ash::vk::{Extent2D, Extent3D, Offset2D, Offset3D, Rect2D};

pub unsafe fn c_string_to_readable<'a>(c_string: *const c_char) -> anyhow::Result<&'a str> {
    Ok(CStr::from_ptr(c_string).to_str()?)
}

pub unsafe fn c_string_slice_to_readable<'a>(slice: &'a [*const c_char]) -> anyhow::Result<Vec<&'a str>> {
    let mut vec: Vec<&'a str> = Vec::with_capacity(slice.len());
    for c_string in slice.iter().cloned() {
       vec.push(c_string_to_readable(c_string)?)
    }

    Ok(vec)
}