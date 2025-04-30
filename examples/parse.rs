use std::io::Write;

fn main() {
    let filename = std::env::args().skip(1).last();

    if filename.is_none() {
        eprintln!("No arguments!");
        std::process::exit(1);
    }

    let data = std::fs::read(filename.unwrap()).unwrap();

    let (_, mut image) = nimage::import::open(data.as_slice()).unwrap();

    let bpp = image.pixel_format().bits_per_pixel();

    println!("use `ffplay -f rawvideo -video_size {}x{} -pixel_format rgb{} -i out.bin`", image.width(), image.height(), bpp);
    
    {
        let mut file = std::fs::File::create("out.bin").unwrap();

        file.write(image.data()).unwrap();
    }

    println!("ok");
}
