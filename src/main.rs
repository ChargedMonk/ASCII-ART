use std::env;
use ::image::{self, DynamicImage, imageops::{resize, FilterType, grayscale}};
use image::GenericImageView;

fn main() {
    // unsafe { env::set_var("RUST_BACKTRACE", "1"); }
    let ascii = ['#', '@', '$', '&', '*', 'Q', 'B', 'O', 'a', 'c', 's', 'e', 'u', 'o', 'n', 'x', ':', '*', '^', '~', '.', ',', '\'', '`'];
    let args: Vec<String> = env::args().collect();
    for k in 3..args.len() {
        let mut img = image::open(&args[k]).unwrap();
        img = DynamicImage::from(resize(&img, (&args[1]).parse::<u32>().unwrap(), (&args[2]).parse::<u32>().unwrap(), FilterType::Nearest));
        println!("Image dimensions: {:#?}, {:#?}", img.dimensions().0, img.dimensions().1);
        img = DynamicImage::from(grayscale(&img));
        let img = img.as_mut_luma8().unwrap();
        let (width, height) = img.dimensions();
        let mut ascii_img: Vec<Vec<char>> = vec![vec![' ' as char; width as usize]; height as usize];

        for pixel in img.enumerate_pixels() {
            let idx = 32 - (((pixel.2[0] as f32) / (255f32)) * 32f32).ceil() as i32;
            if idx < 24 {
                ascii_img[pixel.1 as usize][pixel.0 as usize] = ascii[idx as usize];
            } else {
                ascii_img[pixel.1 as usize][pixel.0 as usize] = ' ';
            }
        }

        for row in ascii_img.iter_mut() {
            for col in row.iter_mut() {
                print!("{col}");
            }
            println!();
        }
    }
}
