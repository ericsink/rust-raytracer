#![deny(unused_imports)]

extern crate rand;
extern crate raylib;

mod util;

fn main() {
    let start_time = std::time::Instant::now();

    let image_data = raylib::run(util::get_rng());

    let ms = start_time.elapsed().as_millis();
    eprintln!("elapsed: {}", ms);

    util::export::to_ppm(&image_data).expect("ppm write failure");
}
