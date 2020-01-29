pub use self::light::Light;
pub mod light;

pub mod lights {
    pub use self::pointlight::PointLight;
    pub use self::spherelight::SphereLight;

    mod pointlight;
    mod spherelight;
}
