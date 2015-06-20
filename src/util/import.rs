use geometry::prims::{Triangle, TriangleVertex};
use geometry::{Mesh, Prim};
use material::materials::CookTorranceMaterial;
use png::PixelsByColorType;
use raytracer::compositor::{Surface, ColorRGBA};
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use vec3::Vec3;

/// This is limited to only CookTorranceMaterials, as I couldn't get a Box<Material> to clone
/// a new material for each triangle primitive in the object model.
#[allow(dead_code)]
pub fn from_obj(material: CookTorranceMaterial /*Box<Material>*/,
                flip_normals: bool, filename: &str)
                -> Mesh {

    let file = File::open(&filename).ok().expect("Couldn't open file");
    let total_bytes = file.metadata().ok().expect("Couldn't load metadata").len();

    let file = BufReader::new(file);

    let start_time = ::time::get_time();
    let print_every = 2048u32;
    let mut current_line = 0;
    let mut processed_bytes = 0;

    let mut vertices: Vec<Vec3> = Vec::new();
    let mut normals : Vec<Vec3> = Vec::new();
    let mut triangles: Vec<Box<Prim+Send+Sync>> = Vec::new();
    let mut tex_coords: Vec<Vec<f64>> = Vec::new();

    for line_iter in file.lines() {
        let line = line_iter.unwrap();
        let tokens: Vec<&str> = line[..].split_whitespace().collect();
        if tokens.len() == 0 { continue }

        match tokens[0] {
            "v" => {
                vertices.push(Vec3 {
                    x: tokens[1].parse().unwrap(),
                    y: tokens[2].parse().unwrap(),
                    z: tokens[3].parse().unwrap()
                });
            },
            "vt" => {
                tex_coords.push(vec![
                    tokens[1].parse().unwrap(),
                    tokens[2].parse().unwrap()
                ]);
            },
            "vn" => {
                let normal_scale = if flip_normals { -1.0 } else { 1.0 };
                normals.push(Vec3 {
                    x: tokens[1].parse::<f64>().unwrap() * normal_scale,
                    y: tokens[2].parse::<f64>().unwrap() * normal_scale,
                    z: tokens[3].parse::<f64>().unwrap() * normal_scale
                });
            },
            "f" => {
                // ["f", "1/2/3", "2/2/2", "12//4"] => [[1, 2, 3], [2, 2, 2], [12, -1u, 4]]
                let pairs: Vec<Vec<usize>> = tokens.tail().iter().map( |token| {
                    let str_tokens: Vec<&str> = token.split('/').collect();
                    str_tokens.iter().map( |str_tok| {
                        match str_tok.parse::<usize>().ok() {
                            Some(usize_tok) => usize_tok - 1,
                            None => !0 // No data available/not supplied
                        }
                    }).collect()
                }).collect();

                // If no texture coordinates were supplied, default to zero.
                // We store nothing supplied as !0
                let (u, v) = if pairs[0][1] != !0 {
                    (vec![
                        tex_coords[pairs[0][1]][0],
                        tex_coords[pairs[1][1]][0],
                        tex_coords[pairs[2][1]][0]
                    ],
                    vec![
                        tex_coords[pairs[0][1]][1],
                        tex_coords[pairs[1][1]][1],
                        tex_coords[pairs[2][1]][1]
                    ])
                } else {
                    (vec![0.0, 0.0, 0.0],
                     vec![0.0, 0.0, 0.0])
                };

                triangles.push(Box::new(Triangle {
                    v0: TriangleVertex { pos: vertices[pairs[0][0]], n: normals[pairs[0][2]], u: u[0], v: v[0] },
                    v1: TriangleVertex { pos: vertices[pairs[1][0]], n: normals[pairs[1][2]], u: u[1], v: v[1] },
                    v2: TriangleVertex { pos: vertices[pairs[2][0]], n: normals[pairs[2][2]], u: u[2], v: v[2] },
                    material: Box::new(material.clone()),
                }));
            },
            _ => {}
        }

        current_line += 1;
        processed_bytes += line.as_bytes().len();
        if current_line % print_every == 0 {
            ::util::print_progress("Bytes", start_time.clone(), processed_bytes, total_bytes as usize);
        }
    }

    // Cheat the progress meter
    ::util::print_progress("Bytes", start_time, total_bytes as usize, total_bytes as usize);

    Mesh {
        triangles: triangles
    }
}

#[allow(dead_code)]
pub fn from_png<P: AsRef<Path>>(path: P) -> Result<Surface, String> {
    let image = try!(::png::load_png(path));
    let mut surface = Surface::new(image.width as usize, image.height as usize, ColorRGBA::black());

    match image.pixels {
        PixelsByColorType::K8(ref bytes) => {
            assert_eq!((image.width * image.height) as usize, bytes.len());
            for (&k, pixel) in bytes.iter().zip(surface.iter_pixels_mut()) {
                *pixel = ColorRGBA::new_rgb(k, k, k);
            }
        },
        PixelsByColorType::KA8(ref bytes) => {
            assert_eq!((2 * image.width * image.height) as usize, bytes.len());
            for (value, pixel) in bytes.chunks(2).zip(surface.iter_pixels_mut()) {
                *pixel = ColorRGBA::new_rgba(value[0], value[0], value[0], value[1]);
            }
        },
        PixelsByColorType::RGB8(ref bytes) => {
            assert_eq!((3 * image.width * image.height) as usize, bytes.len());
            for (value, pixel) in bytes.chunks(3).zip(surface.iter_pixels_mut()) {
                *pixel = ColorRGBA::new_rgb(value[0], value[1], value[2]);
            }
        },
        PixelsByColorType::RGBA8(ref bytes) => {
            assert_eq!((4 * image.width * image.height) as usize, bytes.len());
            for (value, pixel) in bytes.chunks(4).zip(surface.iter_pixels_mut()) {
                *pixel = ColorRGBA::new_rgba(value[0], value[1], value[2], value[3]);
            }
        },
    }
    Ok(surface)
}
