extern crate leptess;
use image::DynamicImage;
use screenshots::Screen;
use std::io::Cursor;
use std::{thread, time::Duration};

fn main() {
    let shot = screenshot();
    ocr(shot);
}

pub fn screenshot() -> DynamicImage {
    thread::sleep(Duration::from_millis(1000));
    let screens = Screen::all();
    let mut output: Option<DynamicImage> = None;
    for screen in screens.unwrap() {
        if screen.display_info.is_primary {
            let img = screen.capture().unwrap();
            let i = image::load_from_memory(img.buffer());
            output = Some(i.unwrap());
        }
    }
    return output.unwrap();
}

pub fn ocr(img: DynamicImage) {
    let mut tess = leptess::LepTess::new(None, "eng").unwrap();
    let mut tiff_buffer = Vec::new();
    img.write_to(
        &mut Cursor::new(&mut tiff_buffer),
        image::ImageOutputFormat::Tiff,
    )
    .unwrap();
    tess.set_image_from_mem(&tiff_buffer).unwrap();
    let text = tess.get_utf8_text().unwrap();
    img.save("ocr.tiff").unwrap();
    println!("{}", text);
}
