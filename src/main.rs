extern crate image;

use std::env;
use std::path::Path;
use std::fs::File;

use image::{FilterType,
            GenericImage,
            DynamicImage,
            ImageFormat,
            PNG,
            JPEG};

fn main() {
    let usage = "Usage: minart imagefile.png";

    // Check for correct usage
    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!(usage)
    };

    // Read file name and extension
    let file_splitted: Vec<String> = file.split(".").map(|s| s.to_string()).collect();;
    if file_splitted.len() < 2 { // To make sure we don't have something like '.jpg' or '.png' as files
        panic!("File type must be explicitly specified.");
    }

    let supported_file_extensions = ["png".to_owned(), "jpg".to_owned(), "jpeg".to_owned()];
    let file_extension = match file_splitted.last() {
        Some(s) => s,
        None => panic!("The file needs to have an explicit extension."),
    };

    if !supported_file_extensions.contains(&file_extension.to_lowercase()) {
        panic!("Only jpeg and png files are supported.");
    }

    let file_name = file.get(0..(file.chars().count() - 1 - file_extension.chars().count())).unwrap();

    let img = match image::open(&Path::new(&file)) {
        Ok(i) => i,
        Err(_) => panic!("No such file."),
    };

    let image_file = ImageFile {file_name: file_name,
                                file_extension: file_extension,
                                image: img};

    process_file(image_file);
}

fn process_file(file: ImageFile) {
    let dim_3x = file.image.dimensions();
    let dim_1x = (dim_3x.0 / 3, dim_3x.1 / 3);
    let dim_2x = (dim_1x.0 * 2, dim_1x.1 * 2);

    let dimensions = vec![dim_1x, dim_2x];

    let is_png = file.file_extension.to_lowercase() == "png";

    for i in 1..4 {
        let mut name = file.file_name.to_owned();
        name.push_str(&format!("-{}x", i));
        let formatted_file_name = format!("{}.{}", name, file.file_extension);
        let mut output = match File::create(formatted_file_name) {
            Ok(o) => o,
            Err(_) => panic!("Couldn't create file."),
        };
        if i == 3 {
            if is_png {
                write_file(&file.image, output, PNG);
                continue;
            }
            write_file(&file.image, output, JPEG);
            continue;
        }
        let scaled = file.image.resize(dimensions[i - 1].0, dimensions[i - 1].1, FilterType::Lanczos3);
        if is_png {
            write_file(&scaled, output, PNG);
        } else {
            write_file(&scaled, output, JPEG);
        }
    }
}

fn write_file(image: &DynamicImage, mut file: File, format: ImageFormat) {
    match image.write_to(&mut file, format) {
        Ok(_) => (),
        Err(_) => panic!("Couldn't write to file."),
    }
}

struct ImageFile<'a> {
    file_name: &'a str,
    file_extension: &'a str,
    image: DynamicImage,
}
