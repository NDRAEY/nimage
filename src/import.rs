pub fn open(data: &[u8]) -> Option<Image> {
    #[cfg(feature = "png")]
    {
        return crate::png::from_png_data(data);
    }

    #[cfg(feature = "tga")]
    {
        return crate::tga::from_tga_data(data);
    }

    None
}
