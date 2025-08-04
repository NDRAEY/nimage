use std::io::Write;

fn main() {
    let filename = std::env::args().skip(1).last();

    if filename.is_none() {
        eprintln!("No arguments!");
        std::process::exit(1);
    }

    let data = std::fs::read(filename.unwrap()).unwrap();

    let (_, mut image) = nimage::import::open(data.as_slice()).unwrap();

    let width = image.width();
    let height = image.height();

    let mut new_data = Vec::with_capacity(width * height * 3);
   
    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y).unwrap();

            new_data.push((pixel & 0xff) as u8);
            new_data.push(((pixel >> 8) & 0xff) as u8);
            new_data.push(((pixel >> 16) & 0xff) as u8);
        }
    }

    {
        let mut file = std::fs::File::create("out.bin").unwrap();

        file.write(&new_data).unwrap();
    }

    println!("{}x{}; ok", width, height);
}
