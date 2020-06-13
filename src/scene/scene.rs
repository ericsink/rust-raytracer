use crate::prelude::*;
use crate::light::Light;
use crate::geometry::Prim;
use crate::raytracer::Octree;
use crate::vec3::Vec3;

pub struct Scene {
    pub lights: Vec<Box<dyn Light+Send+Sync>>,
    pub octree: Octree<Box<dyn Prim+Send+Sync>>,
    pub background: Vec3,
}
