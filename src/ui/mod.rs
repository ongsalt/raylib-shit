pub mod scenes;
pub mod overlays;
pub mod animation;
use raylib::prelude::*;

// pub type SceneRunner = Box<dyn FnMut(&mut RaylibHandle, &mut RaylibThread) -> ()>;
// pub type Scene = fn(rl: &mut RaylibHandle, thread: &RaylibThread) -> SceneRunner;

pub trait Scene {
    fn setup(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread);
    fn reset(&mut self) {}
    fn run(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {}
    fn render_texture(&self) -> &RenderTexture2D;
}

pub trait Overlay {
    fn draw(&mut self, d: &mut RaylibDrawHandle);
}

// NO ONE will get a mut ref to this
pub struct UIResources {
    pub font: Font,
    pub font_header: Font,
    pub box_blur_shader: Shader,
    pub shadow_shader: Shader,
}

impl UIResources {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let font = rl
            .load_font(thread, "assets/fonts/Inter-Regular.ttf")
            .unwrap();
        let font_header = rl
            .load_font(thread, "assets/fonts/Inter-Medium.ttf")
            .unwrap();
        // TODO: write a shader
        let box_blur_shader = rl
            .load_shader(thread, None, Some("assets/shaders/box_blur.fs"))
            .unwrap();
        let shadow_shader = rl
            .load_shader(thread, None, Some("assets/shaders/shadow.fs"))
            .unwrap();

        Self {
            font,
            font_header,
            box_blur_shader,
            shadow_shader,
        }
    }
}