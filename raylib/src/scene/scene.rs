use crate::prelude::*;
use light::Light;
use geometry::Prim;
use raytracer::Octree;
use vec3::Vec3;

pub struct Scene {
    pub lights: Vec<Box<dyn Light+Send+Sync>>,
    pub octree: Octree<Box<dyn Prim+Send+Sync>>,
    pub background: Vec3,
}
