use core::slice;
use std::ffi::CString;

use raylib::{
    camera::Camera2D,
    ffi,
    math::Vector2,
    texture::{Image, RaylibTexture2D, Texture2D},
    RaylibHandle, RaylibThread,
};

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

pub trait RaylibHandleExtension {
    fn load_textures_from_image(
        &mut self,
        _: &RaylibThread,
        image: &Image,
        frames: usize,
    ) -> Result<Vec<Texture2D>, &'static str>;
}

impl RaylibHandleExtension for RaylibHandle {
    fn load_textures_from_image(
        &mut self,
        thread: &RaylibThread,
        image: &Image,
        frames: usize,
    ) -> Result<Vec<Texture2D>, &'static str> {
        let mut textures: Vec<Texture2D> = vec![];
        let frame_size = image.width() * image.height() * 4;
        for i in 0..frames {
            if let Ok(mut texture) = self.load_texture_from_image(thread, image) {
                unsafe {
                    let data_ptr =
                        image.data().offset(i as isize * frame_size as isize) as *const u8;
                    let pixels = slice::from_raw_parts::<u8>(data_ptr, frame_size as usize);
                    texture.update_texture(&pixels);
                    textures.push(texture);
                }
            } else {
                return Err("failed to load image as a texture");
            }
        }

        Ok(textures)
    }
}

pub fn is_visible(position: &Vector2, camera: &Camera2D) -> bool {
    // if point is in camera.target +- offset
    camera.target.x - camera.offset.x <= position.x
        && position.x <= camera.target.x + camera.offset.x
        && camera.target.y - camera.offset.y <= position.y
        && position.y <= camera.target.y + camera.offset.y
}
