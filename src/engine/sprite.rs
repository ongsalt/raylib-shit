use std::slice;
use raylib::prelude::*;
use crate::utils::ImageExtension;

pub struct Sprite {
    image: Image,
    pub texture: Texture2D,
    pub position: Vector2,
    pub is_gif: bool,
    pub scale: f32,
    pub rotation: f32,

    texture_size: i32,
    frame_offset: usize,
    frame_count: u64,
}

impl Sprite {
    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        image_filename: &str,
        is_gif: bool,
    ) -> Self {
        let image = Image::load_image_anim(image_filename).expect("File not found");
        let texture = rl.load_texture_from_image(&thread, &image).unwrap();

        Self {
            texture_size: image.width * image.height * 4,
            is_gif,
            image,
            scale: 1.,
            rotation: 0.,
            texture,
            position: Vector2::zero(),
            frame_count: 0,
            frame_offset: 0,
        }
    }

    pub fn draw<'a>(&mut self, d: &'a mut RaylibMode2D<RaylibDrawHandle>) {
        if self.is_gif {
            self.update_gif_frame();
        }
        d.draw_texture_ex(&self.texture, self.position, self.rotation, self.scale, Color::WHITE);
    }

    fn update_gif_frame(&mut self) {
        self.frame_count += 1;
        if self.frame_count % 3 == 0 {
            self.frame_offset += 1;
            if self.frame_offset >= 24 {
                self.frame_offset = 0
            }
            unsafe {
                let data_ptr = self
                    .image
                    .data()
                    .offset(self.frame_offset as isize * self.texture_size as isize)
                    as *const u8;
                let pixels = slice::from_raw_parts::<u8>(data_ptr, self.texture_size as usize);
                self.texture.update_texture(&pixels);
            }
        }
    }
}
