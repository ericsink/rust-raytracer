use crate::prelude::*;
use crate::raytracer::compositor::ColorRGBA;

pub trait Texture {
    fn color(&self, u: f64, v: f64) -> ColorRGBA<f64>;
    fn clone_self(&self) -> Box<dyn Texture+Send+Sync>;
}

impl Clone for Box<dyn Texture+Send+Sync> {
    fn clone(&self) -> Box<dyn Texture+Send+Sync> {
        self.clone_self()
    }
}
