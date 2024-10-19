use image::{DynamicImage, GenericImageView};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: ImgToAscii <image_path>");
        return;
    }

    // TODO -> CHECK IF FILE EXITS
    // TODO -> ADD COLORS

    let image_path: &str = &args[1];

    image_to_ascii(&get_image(image_path), 8);
}

fn image_to_ascii(image: &DynamicImage, scale: u32) {
    let characters: Vec<char> = " .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$".chars().collect();
    let scaled_index = (255.0 / characters.len() as f32).round() as usize;

    for y in 0..image.height() {
        for x in 0..image.width() {

            if !(y % (scale * 2) == 0 && x % scale == 0) {
                continue;
            }

            let pixel = image.get_pixel(x, y);
            let mut average_brightness: usize = (pixel[0] / 3 + pixel[1] / 3 + pixel[2] / 3) as usize;

            if pixel[3] == 0 {
                average_brightness = 0;
            }

            print!("{}", characters[average_brightness / scaled_index]);

        }

        if y % (scale * 2) != 0 {
            continue;
        }

        println!();
    }

}

fn get_image(path: &str) -> DynamicImage {
    image::open(path).unwrap()
}
