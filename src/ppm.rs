use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::colour::{blue, green, red, Colour};
pub struct PpmFile {
    image_width: u32,
    image_height: u32,
    file_name: String,
}

impl PpmFile {
    pub fn new(file_name: String, image_height: u32, image_width: u32) -> PpmFile {
        PpmFile {
            file_name,
            image_height,
            image_width,
        }
    }
}

pub fn write_to_file(ppm: PpmFile, colours: Vec<Colour>) {
    let f = File::create(ppm.file_name).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    let header = format!("P3\n{0} {1}\n255\n", ppm.image_width, ppm.image_height);

    f.write_all(header.as_bytes())
        .expect("Unable to write to file");

    for colour in colours {
        let ppm_colour = to_ppm_colour_space(&colour);

        f.write_all(&to_ppm_row(ppm_colour))
            .expect("Can't write lines to file.");
    }
}

fn to_ppm_colour_space(c: &Colour) -> (i32, i32, i32) {
    let ir = (255.999 * red(c)) as i32;
    let ig = (255.999 * green(c)) as i32;
    let ib = (255.999 * blue(c)) as i32;
    (ir, ig, ib)
}

fn to_ppm_row((r, g, b): (i32, i32, i32)) -> Vec<u8> {
    format!("{r} {g} {b}\n").as_bytes().to_vec()
}
