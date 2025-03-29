use std::io::Write;

static IMAGE_DATA: &[u8] = include_bytes!("../static/test_image.bin");

fn main() {
    let mut image = nimage::Image::from_raw_data(IMAGE_DATA, 320, 240, nimage::PixelFormat::RGB);

    image.scale(640, 480);

    {
        let mut file = std::fs::File::create("out.bin").unwrap();

        file.write(image.data()).unwrap();
    }

    println!("ok");
}
