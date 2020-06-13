use std::io::{self, Write};
use crate::Surface;

pub fn to_ppm(surface: &Surface) -> io::Result<()> {
    let header = format!(
        "P3 {} {} {}\n", surface.width, surface.height,
        u8::max_value());

    let stdout = io::stdout();
    let mut f = stdout.lock();

    f.write_all(header.as_bytes())?;
    for pixel in &surface.buffer {
        f.write_all(format!("{} {} {} ", pixel.r, pixel.g, pixel.b).as_bytes())?;
    }
    Ok(())
}
