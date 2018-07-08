extern crate image;

use std::env;
use std::path::Path;
use std::fs::File;

use image::{FilterType,
            GenericImage,
            PNG};

fn main() {
    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };

    // TODO: Check for unspecified file types

    let img = image::open(&Path::new(&file)).unwrap();

    let file_splitted = file.split(".");
    let file_type = file_splitted.last().unwrap();
    let file_name = file.get(0..(file.chars().count() - 1 - file_type.chars().count())).unwrap();

    let dim_3x = img.dimensions();
    let dim_1x = (dim_3x.0 / 3, dim_3x.1 / 3);
    let dim_2x = (dim_1x.0 * 2, dim_1x.1 * 2);

    let dimensions = vec![dim_1x, dim_2x];

    for i in 1..4 {
        let mut name = file_name.to_owned();
        name.push_str(&format!("-{}x", i));
        let formatted_file_name = format!("{}.{}", name, file_type);
        let mut output = File::create(formatted_file_name).unwrap();
        if i == 3 {
            img.write_to(&mut output, PNG).unwrap();
            continue;
        }
        let scaled = img.resize(dimensions[i - 1].0, dimensions[i - 1].1, FilterType::Lanczos3);
        scaled.write_to(&mut output, PNG).unwrap();
    }
}
