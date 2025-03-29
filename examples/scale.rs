use std::io::Write;

static IMAGE_DATA: &[u8] = include_bytes!("../static/abstract.tga");

fn main() {
    // let mut image = nimage::Image::from_raw_data(IMAGE_DATA, 320, 240, nimage::PixelFormat::RGB);
    let mut image = nimage::tga::from_tga_data(IMAGE_DATA).unwrap();

    println!("{}x{}", image.width(), image.height());

    image.scale(640, 480);

    {
        let mut file = std::fs::File::create("out.bin").unwrap();

        file.write(image.data()).unwrap();
    }

    println!("ok");
}
