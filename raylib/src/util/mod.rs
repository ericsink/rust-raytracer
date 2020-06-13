use rand::{RngCore};

pub mod export;

pub fn get_rng() -> Box<dyn RngCore> {
    Box::new(rand::thread_rng())
}

