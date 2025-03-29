#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "tga")]
pub mod tga;

#[derive(Copy, Clone)]
pub enum PixelFormat {
    RGBA,
    ARGB,
    BGRA,
    ABGR,
    RGB,
    BGR,
}

pub struct PixelFormatOffset {
    r: usize, g: usize, b: usize, a: Option<usize>
}

#[derive(Clone)]
pub struct Image {
    width: usize,
    height: usize,
    pixel_format: PixelFormat,
    pub(crate) data: Vec<u8>,
}

impl Image {
    pub fn new(width: usize, height: usize, pixfmt: PixelFormat) -> Self {
        Self {
            width, height, pixel_format: pixfmt, data: vec![0u8; width * height * Self::pixfmt_to_bpp(pixfmt)]
        }
    }

    fn pixfmt_to_bpp(pixfmt: PixelFormat) -> usize {
        match pixfmt {
            PixelFormat::RGBA => 4,
            PixelFormat::ARGB => 4,
            PixelFormat::BGRA => 4,
            PixelFormat::ABGR => 4,
            PixelFormat::RGB => 3,
            PixelFormat::BGR => 3,
            // _ => todo!("Support for other pixel formats"),
        }
    }

    /// To convert to `0xAARRGGBB`
    fn pixfmt_to_offset(pixfmt: PixelFormat) -> PixelFormatOffset {
        match pixfmt {
            PixelFormat::RGBA => {
                PixelFormatOffset {
                    a: Some(0), r: 24, g: 16, b: 8
                }
            },
            PixelFormat::ARGB => {
                PixelFormatOffset {
                    a: Some(24), r: 16, g: 8, b: 0
                }
            },
            PixelFormat::BGRA => {
                PixelFormatOffset {
                    a: Some(0), r: 8, g: 16, b: 24
                }
            },
            PixelFormat::ABGR => {
                PixelFormatOffset {
                    a: Some(24), r:0, g: 8, b: 16
                }
            },
            PixelFormat::RGB => {
                PixelFormatOffset {
                    a: None, r: 16, g: 8, b: 0
                }
            },
            PixelFormat::BGR => {
                PixelFormatOffset {
                    a: None, r: 0, g: 8, b: 16
                }
            },
        }
    }

    fn convert_to_universal(pixfmt: PixelFormat, color: &[u8]) -> u32 {
        let pxo = Self::pixfmt_to_offset(pixfmt);

        let mut outcolor = 0u32;

        if let Some(alpha) = pxo.a {
            outcolor |= ((color[alpha >> 3]) as u32) << 24;
        }

        outcolor |= (color[pxo.r >> 3] as u32) << 16;    // R
        outcolor |= (color[pxo.g >> 3] as u32) << 8;     // G
        outcolor |= color[pxo.b >> 3] as u32;            // B

        outcolor
    }

    /// Universal color has `0xAARRGGBB` format
    fn universal_to_preferred(pixfmt: PixelFormat, color: u32) -> [u8; 4] {
        let pxo = Self::pixfmt_to_offset(pixfmt);

        let a = (color >> 24) & 0xff;
        let r = (color >> 16) & 0xff;
        let g = (color >> 8) & 0xff;
        let b = color & 0xff;

        let (offset_a, offset_r, offset_g, offset_b) = (pxo.a.map(|x| x >> 3), pxo.r >> 3, pxo.g >> 3, pxo.b >> 3);

        let mut out = [0u8; 4];

        if let Some(offset_a) = offset_a {
            out[offset_a] = a as u8;
        }

        out[offset_r] = r as u8;
        out[offset_g] = g as u8;
        out[offset_b] = b as u8;
        
        out
    }

    pub fn from_raw_data(
        data: &[u8],
        width: usize,
        height: usize,
        pixel_format: PixelFormat,
    ) -> Self {
        Self {
            width,
            height,
            data: Vec::from(data),
            pixel_format,
        }
    }

    pub fn from_raw_data_vec(
        data: Vec<u8>,
        width: usize,
        height: usize,
        pixel_format: PixelFormat,
    ) -> Self {
        Self {
            width,
            height,
            data,
            pixel_format,
        }
    }

    /// Fills whole image with given color
    /// 
    /// Color in `0xAARRGGBB` format
    pub fn fill(&mut self, color: u32) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_pixel(x, y, color);
            }
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn bytes_per_pixel(&self) -> usize {
        Self::pixfmt_to_bpp(self.pixel_format)
    }

    pub fn size(&self) -> usize {
        self.width * self.height * Self::pixfmt_to_bpp(self.pixel_format)
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub fn data_mut(&mut self) -> &mut[u8] {
        self.data.as_mut_slice()
    }

    #[inline]
    fn pixel_index(width: usize, x: usize, y: usize) -> usize {
        width * y + x
    }

    /// Output format is `0xAARRGGBB`
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<u32> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let idx = Self::pixel_index(self.width, x, y);
        let data = &self.data()[idx * self.bytes_per_pixel()..];
        let color = Self::convert_to_universal(self.pixel_format, data);

        Some(color)
    }

    /// Input format is `0xAARRGGBB`
    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }

        let idx = Self::pixel_index(self.width, x, y);

        let color = Self::universal_to_preferred(self.pixel_format, color);
        let bpp = self.bytes_per_pixel();
        
        let data = &mut self.data_mut()[idx * bpp..];
        
        data[0] = color[0];
        data[1] = color[1];
        data[2] = color[2];
        
        if bpp == 4 {
            data[3] = color[3];
        }
    }

    pub fn scale(&mut self, target_width: usize, target_height: usize) {
        if target_width == self.width && target_height == self.height {
            return;
        }

        let bpp = self.bytes_per_pixel();

        let mut scaled_data = vec![0u8; target_width * target_height * bpp];

        for y in 0..target_height {
            for x in 0..target_width {
                let src_x = (x as f32) * (self.width as f32 - 1.0) / (target_width as f32 - 1.0);
                let src_y = (y as f32) * (self.height as f32 - 1.0) / (target_height as f32 - 1.0);

                let x0 = src_x.floor() as usize;
                let y0 = src_y.floor() as usize;
                let x1 = (src_x.ceil() as usize).min(self.width - 1);
                let y1 = (src_y.ceil() as usize).min(self.height - 1);

                let x_weight = src_x - x0 as f32;
                let y_weight = src_y - y0 as f32;

                for c in 0..bpp {
                    let top_left =
                        self.data[Self::pixel_index(self.width, x0, y0) * bpp + c];
                    let top_right =
                        self.data[Self::pixel_index(self.width, x1, y0) * bpp + c];
                    let bottom_left =
                        self.data[Self::pixel_index(self.width, x0, y1) * bpp + c];
                    let bottom_right =
                        self.data[Self::pixel_index(self.width, x1, y1) * bpp + c];

                    let top = top_left as f32 * (1.0 - x_weight) + top_right as f32 * x_weight;
                    let bottom =
                        bottom_left as f32 * (1.0 - x_weight) + bottom_right as f32 * x_weight;

                    let value = top * (1.0 - y_weight) + bottom * y_weight;

                    scaled_data[Self::pixel_index(target_width, x, y) * bpp + c] =
                        value.round() as u8;
                }
            }
        }

        self.width = target_width;
        self.height = target_height;
        self.data = scaled_data;
    }

    pub fn get_line(&self, line: usize) -> Option<&[u8]> {
        if line >= self.height {
            return None;
        }

        let idx = Self::pixel_index(self.width, 0, line);
        let idx_end = Self::pixel_index(self.width, 0, line + 1);
        
        Some(&self.data[idx..idx_end])
    }

    pub fn flip_vertically(&mut self) {
        let bpp = self.bytes_per_pixel();
        let stride = self.width * bpp;
        let height = self.height;
        let mut buffer = vec![0u8; stride];
        
        for y in 0..height / 2 {
            let iy = height - 1 - y;
            
            let main_start = y * stride;
            let rival_start = iy * stride;
            
            buffer.copy_from_slice(&self.data[main_start..main_start + stride]);
            
            self.data.copy_within(rival_start..rival_start + stride, main_start);
            self.data[rival_start..rival_start + stride].copy_from_slice(&buffer);
        }
    }

    pub fn reverse_line(&mut self, line: usize) {
        if line >= self.height {
            return;
        }

        for i in 0..self.width / 2 {
            let rival_pixel = (self.width - 1) - i;

            let curpix = self.get_pixel(i, line);
            let endpix = self.get_pixel(rival_pixel, line);

            self.set_pixel(i, line, endpix.unwrap());
            self.set_pixel(rival_pixel, line, curpix.unwrap());
        }
    }

    pub fn flip_horizontally(&mut self) {
        for i in 0..self.height() {
            self.reverse_line(i);
        }
    }

    pub fn cut(&mut self, x: usize, y: usize, width: usize, height: usize) {
        let mut new_image = Image::new(width, height, self.pixel_format);

        for oy in 0..height {
            for ox in 0..width {
                let color = self.get_pixel(x + ox, y + oy);

                new_image.set_pixel(ox, oy, color.unwrap_or(0));
            }
        }

        *self = new_image;
    }

    pub fn scale_by_factor(&mut self, factor: f64) {
        let w = self.width() as f64 * factor;
        let h = self.height() as f64 * factor;

        self.scale(w.ceil() as _, h.ceil() as _);
    }

    fn get_column(&self, column: usize) -> Option<Vec<u8>> {
        if column >= self.width {
            return None;
        }

        let bpp = Self::pixfmt_to_bpp(self.pixel_format);

        let mut result: Vec<u8> = vec![];

        for y in 0..self.height {
            let color = self.get_pixel(column, y).unwrap();
            let channels = Self::universal_to_preferred(self.pixel_format, color);

            result.extend(&channels[..bpp]);
        }

        Some(result)
    }
    
    pub fn rotate_left(&mut self) {
        let mut buffer: Vec<u8> = Vec::with_capacity(self.size());

        for x in (0..self.width()).rev() {
            let column = self.get_column(x).unwrap();

            buffer.extend(column);
        }

        (self.width, self.height) = (self.height, self.width);

        self.data = buffer;
    }

    pub fn rotate_right(&mut self) {
        let mut buffer: Vec<u8> = Vec::with_capacity(self.size());

        for x in 0..self.width() {
            let column = self.get_column(x).unwrap();
            let column = column.chunks(self.bytes_per_pixel()).rev();

            for i in column {
                buffer.extend(i);
            }
        }

        (self.width, self.height) = (self.height, self.width);

        self.data = buffer;

        // self.flip_horizontally();
    }
}
