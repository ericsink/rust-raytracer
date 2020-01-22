use rand::{RngCore};

pub mod export;

#[cfg(target_os = "wasi")]
pub fn get_rng() -> Box<dyn RngCore> {
    Box::new(wasi_rng::WasiRng)
}

#[cfg(not(target_os = "wasi"))]
pub fn get_rng() -> Box<dyn RngCore> {
    Box::new(rand::thread_rng())
}

