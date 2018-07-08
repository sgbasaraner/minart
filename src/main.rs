extern crate image;

use std::env;
use std::path::Path;

use image::GenericImage;

fn main() {
    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };

    let img = image::open(&Path::new(&file)).unwrap();

    let dim_3x = img.dimensions();
    let dim_1x = (dim_3x.0 / 3, dim_3x.1 / 3);
    let dim_2x = (dim_1x.0 * 2, dim_1x.1 * 2);

    println!("3x: {:?}", dim_3x);
    println!("3x: {:?}", dim_2x);
    println!("3x: {:?}", dim_1x);
}
