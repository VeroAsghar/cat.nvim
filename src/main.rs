use image::imageops;
use image::io::Reader;
use std::io::Cursor;

use scraper::{Html, Selector};

#[tokio::main]
async fn main() {
    let body = reqwest::get("https://www.google.com/search?q=cat&source=lnms&tbm=isch")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let images = Html::parse_document(&body);
    let selector = Selector::parse("img").unwrap();

    let image_url = images
        .select(&selector)
        .nth(1)
        .unwrap()
        .value()
        .attr("src")
        .unwrap();

    let image = reqwest::get(image_url)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    let image_buf = Reader::new(Cursor::new(image))
        .with_guessed_format()
        .unwrap();
    let image = image_buf.decode().unwrap();
    let image = image.into_luma8();
    //let image = imageops::resize(&image, 24, 24, imageops::Lanczos3);
    let rows = image.rows();
    for row in rows {
        for pixel in row {
            print!("{:?}", map_brightness_to_ascii(pixel.0[0].into()));
        }
        println!();
    }
    image.save("cat.png").unwrap();
}

const CHARS: &str = r#" .:-=+*#%@"#;
const LEN: usize = CHARS.len();

fn map_brightness_to_ascii(val: usize) -> char {
    let val = val * LEN  / 255;

    match CHARS.chars().nth(val) {
        Some(c) => c,
        None => {
            eprintln!("{val}");
            panic!();
        },
    }
}
