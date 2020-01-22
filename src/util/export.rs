use std::io::{self, Write};
use raytracer::compositor::{Surface, Channel};

pub fn to_ppm(surface: &Surface) -> io::Result<()> {
    let channel_max: u8 = Channel::max_value();
    let header = format!(
        "P3 {} {} {}\n", surface.width, surface.height,
        channel_max);

    let stdout = io::stdout();
    let mut f = stdout.lock();

    f.write_all(header.as_bytes())?;
    for pixel in &surface.buffer {
        f.write_all(format!("{} {} {} ", pixel.r, pixel.g, pixel.b).as_bytes())?;
    }
    Ok(())
}
