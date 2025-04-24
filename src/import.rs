use crate::Image;

/// Tries to open file using various formats. Returns `None` if no format matches given data.
pub fn open(data: &[u8]) -> Option<(&'static str, Image)> {
    #[cfg(feature = "png")]
    {
        if let Some(image) = crate::png::from_png_data(data) {
            return Some(("png", image));
        }
    }

    #[cfg(feature = "tga")]
    {
        if let Some(image) = crate::tga::from_tga_data(data) {
            return Some(("tga", image));
        }
    }

    None
}
