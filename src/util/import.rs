use geometry::prims::{Triangle, TriangleVertex};
use geometry::{Mesh, Prim};
use material::materials::CookTorranceMaterial;
use raytracer::compositor::{Surface, ColorRGBA};
use std::old_io::fs::lstat;
use std::old_io::{BufferedReader, File};
use std::str::StrExt;
use vec3::Vec3;

/// This is limited to only CookTorranceMaterials, as I couldn't get a Box<Material> to clone
/// a new material for each triangle primitive in the object model.
#[allow(dead_code)]
pub fn from_obj(material: CookTorranceMaterial /*Box<Material>*/,
                flip_normals: bool, filename: &str)
                -> Mesh {

    let path = Path::new(filename);
    let fh = File::open(&path);
    let mut file = BufferedReader::new(fh);

    let start_time = ::time::get_time();
    let print_every = 2048u32;
    let mut current_line = 0;
    let mut processed_bytes = 0;

    let total_bytes = match lstat(&path) {
        Ok(stat) => stat.size,
        Err(e) => panic!("Could not open file {} (file missing?) : {}", filename, e)
    };

    let mut vertices: Vec<Vec3> = Vec::new();
    let mut normals : Vec<Vec3> = Vec::new();
    let mut triangles: Vec<Box<Prim+Send+Sync>> = Vec::new();
    let mut tex_coords: Vec<Vec<f64>> = Vec::new();

    for line_iter in file.lines() {
        let line = line_iter.unwrap();
        let tokens: Vec<&str> = line[..].words().collect();
        if tokens.len() == 0 { continue }

        match tokens[0].as_slice() {
            "v" => {
                vertices.push(Vec3 {
                    x: StrExt::parse::<f64>(tokens[1].as_slice()).unwrap(),
                    y: StrExt::parse::<f64>(tokens[2].as_slice()).unwrap(),
                    z: StrExt::parse::<f64>(tokens[3].as_slice()).unwrap()
                });
            },
            "vt" => {
                tex_coords.push(vec![
                    StrExt::parse::<f64>(tokens[1].as_slice()).unwrap(),
                    StrExt::parse::<f64>(tokens[2].as_slice()).unwrap()
                ]);
            },
            "vn" => {
                let normal_scale = if flip_normals { -1.0 } else { 1.0 };
                normals.push(Vec3 {
                    x: StrExt::parse::<f64>(tokens[1].as_slice()).unwrap() * normal_scale,
                    y: StrExt::parse::<f64>(tokens[2].as_slice()).unwrap() * normal_scale,
                    z: StrExt::parse::<f64>(tokens[3].as_slice()).unwrap() * normal_scale
                });
            },
            "f" => {
                // ["f", "1/2/3", "2/2/2", "12//4"] => [[1, 2, 3], [2, 2, 2], [12, -1u, 4]]
                let pairs: Vec<Vec<usize>> = tokens.tail().iter().map( |token| {
                    let str_tokens: Vec<&str> = token.as_slice().split('/').collect();
                    str_tokens.iter().map( |str_tok| {
                        match StrExt::parse::<usize>(*str_tok).ok() {
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

                triangles.push(box Triangle {
                    v0: TriangleVertex { pos: vertices[pairs[0][0]], n: normals[pairs[0][2]], u: u[0], v: v[0] },
                    v1: TriangleVertex { pos: vertices[pairs[1][0]], n: normals[pairs[1][2]], u: u[1], v: v[1] },
                    v2: TriangleVertex { pos: vertices[pairs[2][0]], n: normals[pairs[2][2]], u: u[2], v: v[2] },
                    material: box material.clone()
                });
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
pub fn from_ppm(filename: &str) -> Surface {
    let path = Path::new(filename);
    let mut file = BufferedReader::new(File::open(&path));

    let tex = match file.read_to_string() {
        Ok(f) => f,
        Err(e) => panic!("Could not open file {} (file missing?): {}", filename, e)
    };
    let mut tokens: Vec<&str> = tex[..].words().collect();

    tokens.remove(0); // PPM type
    let width  = StrExt::parse::<usize>(tokens.remove(0)).unwrap();
    let height = StrExt::parse::<usize>(tokens.remove(0)).unwrap();
    tokens.remove(0); // Max color value

    print!("Importing image texture {}", filename);
    println!(" {}x{}", width, height);

    let mut surface = Surface::new(width, height, ColorRGBA::new_rgb(0, 0, 0));

    let mut i = 0usize;

    for chunk in tokens[..].chunks(3) {
        let x = i % width;
        let y = i / width;
        i += 1;

        if x >= width || y >= height { break };

        surface[(x, y)] = ColorRGBA::new_rgb(
            StrExt::parse::<u8>(chunk[0]).unwrap(),
            StrExt::parse::<u8>(chunk[1]).unwrap(),
            StrExt::parse::<u8>(chunk[2]).unwrap()
        );
    }

    surface
}
