use crate::{Image, PixelFormat};

pub fn from_png_data(data: &[u8]) -> Option<Image> {
    let rd = png::Decoder::new(data);
    let reader = rd.read_info();

    if let Err(_) = reader {
        return None;
    }

    let mut reader = reader.unwrap();
    let size = reader.output_buffer_size();

    let width = reader.info().width;
    let height = reader.info().height;

    let mut buf = vec![0; size];

    let frame_info = reader.next_frame(&mut buf).unwrap();

    let bytes = &buf[..frame_info.buffer_size()];

    let samples = frame_info.color_type.samples();

    let out = Image {
        width: width as usize,
        height: height as usize,
        pixel_format: match samples{
            4 => PixelFormat::RGBA,
            3 => PixelFormat::RGB,
            _ => todo!("Implement decoding for: {:?}", frame_info.color_type)
        },
        data: bytes.to_vec(),
    };

    Some(out)
}
