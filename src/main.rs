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

    println!("dimensions {:?}", img.dimensions());
}
