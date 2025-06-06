/// This module provides functions to work with TGA images.

use alloc::vec::Vec;

use tinytga::Bpp::Bits32;
use tinytga::{ImageOrigin, RawTga};

use crate::{Image, PixelFormat};

/// Make an image out of raw TGA data.
///
/// `None` is returned if any error is present
pub fn from_tga_data(data: &[u8]) -> Option<Image> {
    let im = RawTga::from_slice(data);

    match im {
        Ok(image) => {
            let bpp = 8 * (image.image_data_bpp() as u32 + 1);
            let size = image.pixels().count() * bpp as usize;
            let mut pixels = Vec::with_capacity(size);

            for i in image.pixels() {
                let r = (i.color >> 16) as u8;
                let g = (i.color >> 8) as u8;
                let b = i.color as u8;

                pixels.extend([r, g, b]);

                if bpp == 32 {
                    let a = (i.color >> 24) as u8;
                    pixels.push(a);
                }
            }

            let mut out = Image {
                width: image.size().width as usize,
                height: image.size().height as usize,
                pixel_format: if image.image_data_bpp() == Bits32 {
                    PixelFormat::RGBA
                } else {
                    PixelFormat::RGB
                },
                data: pixels,
            };

            match image.image_origin() {
                ImageOrigin::TopLeft => {},
                ImageOrigin::BottomLeft => {
                    out.flip_vertically();
                }
                _ => todo!("Implement image translation for `{:?}`", image.image_origin())
            }

            Some(out)
        }
        Err(_) => None,
    }
}
