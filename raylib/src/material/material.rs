use vec3::Vec3;

/// TODO: Move specular/transmissive properties into traits
pub trait Material {
    fn sample(&self, n: Vec3, i: Vec3, l: Vec3, u: f64, v: f64) -> Vec3;
    fn is_reflective(&self) -> bool;
    fn is_refractive(&self) -> bool;
    fn global_specular(&self, color: &Vec3) -> Vec3;
    fn global_transmissive(&self, color: &Vec3) -> Vec3;
    fn transmission(&self) -> Vec3;
    fn ior(&self) -> f64;
    fn is_glossy(&self) -> bool;
    fn glossiness(&self) -> f64;
}
