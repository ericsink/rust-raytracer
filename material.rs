use vec3::Vec3;

/// TODO: Move specular/transmissive properties into traits
pub trait Material {
    fn sample(&self, n: Vec3, i: Vec3, l: Vec3) -> Vec3;
    fn is_specular(&self) -> bool;
    fn global_specular(&self, color: &Vec3) -> Vec3;
    fn transmission(&self) -> Vec3;
}
