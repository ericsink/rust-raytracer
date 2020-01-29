#![cfg_attr(test, allow(dead_code))]
use ::scene::{Camera, Scene};

pub mod cornell;

pub trait SceneConfig {
    fn get_camera(&self, image_width: u32, image_height: u32, fov: f64) -> Camera;

    fn get_animation_camera(&self, image_width: u32, image_height: u32, fov: f64) -> Camera {
        self.get_camera(image_width, image_height, fov)
    }

    fn get_scene(&self) -> Scene;
}

pub fn get_scene() -> Box<dyn SceneConfig> {
    Box::new(cornell::CornelConfig)
}
