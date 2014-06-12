use ray::Ray;
use intersection::Intersection;

pub trait Prim {
    fn intersects<'a>(&'a self, ray: &Ray, t_min: f64, t_max: f64) -> Intersection<'a>;
}
