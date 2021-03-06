
#![deny(unused_imports)]

//extern crate core;
extern crate alloc;
extern crate num_traits;
extern crate rand;

/* maybe TODO
extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
        loop {}
}
*/

mod prelude;
mod geometry;
mod light;
mod material;
mod my_scene;
mod raytracer;
mod scene;
mod vec3;
mod mat4;

pub use raytracer::compositor::Surface;

use crate::prelude::*;

struct SceneConfig {
    size: (u32, u32),
    fov: f64,
    reflect_depth: u32,
    refract_depth: u32,
    shadow_samples: u32,
    gloss_samples: u32,
    pixel_samples: u32,
}

pub fn run(mut rng: Box<dyn rand::RngCore>) -> Surface {
    let config = SceneConfig {
        size: (512, 512),
        fov: 30.0,
        reflect_depth: 3,
        refract_depth: 6,
        shadow_samples: 16,
        gloss_samples: 8,
        pixel_samples: 2,
    };

    let scene_config = my_scene::get_scene();

    let (image_width, image_height) = config.size;
    let fov = config.fov;

    let shared_scene = scene_config.get_scene();

    let camera = 
        scene_config.get_camera(image_width, image_height, fov)
        ;

    let render_options = raytracer::RenderOptions {
        reflect_depth: config.reflect_depth,
        refract_depth: config.refract_depth,
        shadow_samples: config.shadow_samples,
        gloss_samples: config.gloss_samples,
        pixel_samples: config.pixel_samples,
    };

    let renderer = raytracer::Renderer {
        options: render_options,
    };

    let image_data = renderer.render(camera, &mut rng, &shared_scene);

    image_data
}

mod util;

fn main() {
    let start_time = std::time::Instant::now();

    let image_data = run(util::get_rng());

    let ms = start_time.elapsed().as_millis();
    eprintln!("elapsed: {}", ms);

    util::export::to_ppm(&image_data).expect("ppm write failure");
}
