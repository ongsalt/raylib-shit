use std::ffi::CString;

use raylib::{ffi, texture::Image};

pub trait ImageExtension {
    fn load_image_anim(filename: &str) -> Result<Image, &'static str>;
}

impl ImageExtension for Image {
    fn load_image_anim(filename: &str) -> Result<Image, &'static str> {
        unsafe {
            let c_filename = CString::new(filename).unwrap();
            let mut frames = 5;
            let i = ffi::LoadImageAnim(c_filename.as_ptr(), &mut frames);
            if i.data.is_null() {
                return Err("no image data");
            }
            Ok(Image::from_raw(i))
        }
    }
}