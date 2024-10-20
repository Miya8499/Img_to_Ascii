use image::{DynamicImage, GenericImageView};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: ImgToAscii <image_path>");
        return;
    }

    // TODO -> CHECK IF FILE EXITS

    let image_path: &str = &args[1];

    image_to_ascii(&get_image(image_path), 8);
}

fn image_to_ascii(image: &DynamicImage, scale: u32) {
    let characters: Vec<char> = " .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$".chars().collect();

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

            print!("{}", get_colored_char(characters[get_scaled_index(&characters, 255, average_brightness)], pixel[0], pixel[1], pixel[2]));

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

fn get_scaled_index<T>(list: &Vec<T>, max_values: u8, index_to_scale: usize) -> usize {
    let max_index: f32 = (list.len() - 1) as f32;
    let divisor: f32 = max_values as f32 / (max_index);
    let result: f32 = index_to_scale as f32 / divisor;

    if result.fract() == 0.0 {
        return result as usize
    }

    if result.round() > max_index {
        return max_index as usize
    }

    result as usize
}

fn get_colored_char(text: char, r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{r};{g};{b}m{text}\x1b[0m")
}
