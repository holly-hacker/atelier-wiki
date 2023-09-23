use anyhow::{bail, Context};

pub struct Rgba8Image {
    width: u32,
    data: Vec<u8>,
}

impl Rgba8Image {
    /// Create a new transparent image with the given dimensions.
    pub fn new_empty(width: u32, height: u32) -> Self {
        if width == 0 {
            panic!("width must be greater than 0");
        }
        if height == 0 {
            panic!("height must be greater than 0");
        }
        Self {
            width,
            data: vec![0; (width * height * 4) as usize],
        }
    }

    /// Create a new image with the given RGBA8 data.
    pub fn new(width: u32, data: Vec<u8>) -> anyhow::Result<Self> {
        if width == 0 {
            bail!("width must be greater than 0");
        }
        if data.len() % 4 != 0 {
            bail!("data length must be a multiple of 4");
        }
        let pixel_count = data.len() / 4;
        if pixel_count % width as usize != 0 {
            bail!("data buffer length must be a multiple of width");
        }
        Ok(Self { width, data })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        debug_assert_eq!(
            self.data.len() % 4,
            0,
            "data length must be a multiple of 4"
        );
        let pixel_count = self.data.len() / 4;
        debug_assert_eq!(
            pixel_count % self.width as usize,
            0,
            "data length must be a multiple of width"
        );
        (pixel_count / self.width as usize) as u32
    }

    pub fn unpack(self) -> (Vec<u8>, u32) {
        (self.data, self.width)
    }

    /// Scales the image down by the given factor, using simple supersampling with uniform grid
    /// distribution. This currently only supports scaling down by integer factors.
    pub fn scale_down(&self, scale: (u32, u32)) -> Self {
        let (width, height) = (self.width(), self.height());
        let (scale_x, scale_y) = scale;

        let new_width = width / scale_x;
        let new_height = height / scale_y;

        let mut new_image = Rgba8Image::new_empty(new_width, new_height);

        // For each pixel in the scaled image...
        for y in 0..new_height {
            for x in 0..new_width {
                // Calculate the average color for the corresponding set of pixels in the original image.
                let mut r = 0;
                let mut g = 0;
                let mut b = 0;
                let mut a = 0;

                // we probably want to skip fully transparent pixels because they'll contribute
                // invalid RGB data, so instead of dividing by scale_x * scale_y, we divide by
                // the number of non-transparent pixels
                let mut num_non_transparent = 0;
                for y2 in 0..scale_y {
                    for x2 in 0..scale_x {
                        let index = ((y * scale_y + y2) * width + (x * scale_x + x2)) as usize * 4;
                        if self.data[index + 3] == 0 {
                            continue;
                        }
                        r += self.data[index] as u32;
                        g += self.data[index + 1] as u32;
                        b += self.data[index + 2] as u32;
                        a += self.data[index + 3] as u32;
                        num_non_transparent += 1;
                    }
                }

                // Set the corresponding pixel in the new image to the average color.
                // since we skip fully transparent pixels, we may have no non-transparent pixels
                if num_non_transparent != 0 {
                    let index = (y * new_width + x) as usize * 4;
                    new_image.data[index] = (r / (num_non_transparent)) as u8;
                    new_image.data[index + 1] = (g / (num_non_transparent)) as u8;
                    new_image.data[index + 2] = (b / (num_non_transparent)) as u8;
                    new_image.data[index + 3] = (a / (num_non_transparent)) as u8;
                }
            }
        }

        new_image
    }

    /// Copies a chunk of the image into a new image.
    pub fn copy_chunk(&self, x: u32, y: u32, width: u32, height: u32) -> anyhow::Result<Self> {
        if width == 0 {
            bail!("chunk width must be greater than 0");
        }
        if height == 0 {
            bail!("chunk height must be greater than 0");
        }
        if x + width > self.width {
            bail!(
                "right edge out of bounds, {} but width is {}",
                x + width,
                self.width
            );
        }
        if y + height > self.height() {
            bail!(
                "bottom edge out of bounds, {} but height is {}",
                y + height,
                self.height()
            );
        }

        let mut new_image = Rgba8Image::new_empty(width, height);

        for row in 0..height {
            let data_len = (width * 4) as usize;

            let self_data_start = (((y + row) * self.width + x) * 4) as usize;
            let self_slice = &self.data[self_data_start..(self_data_start + data_len)];

            let other_data_start = (row * width * 4) as usize;
            let other_slice = &mut new_image.data[other_data_start..(other_data_start + data_len)];

            other_slice.copy_from_slice(self_slice);
        }

        Ok(new_image)
    }

    /// Blits another image onto this image at the given coordinates.
    pub fn blit(&mut self, x: u32, y: u32, other: &Self) -> anyhow::Result<()> {
        let (width, height) = (other.width(), other.height());

        if width + x > self.width() || height + y > self.height() {
            bail!("blit out of bounds");
        }

        for y2 in 0..height {
            for x2 in 0..width {
                let index = ((y + y2) * self.width + (x + x2)) as usize * 4;
                let index2 = (y2 * width + x2) as usize * 4;
                self.data[index] = other.data[index2];
                self.data[index + 1] = other.data[index2 + 1];
                self.data[index + 2] = other.data[index2 + 2];
                self.data[index + 3] = other.data[index2 + 3];
            }
        }

        Ok(())
    }

    pub fn encode_oxipng(self, compression: u8) -> anyhow::Result<Vec<u8>> {
        let (width, height) = (self.width(), self.height());
        let (data, _) = self.unpack();

        let image_buffer = oxipng::RawImage::new(
            width,
            height,
            oxipng::ColorType::RGBA,
            oxipng::BitDepth::Eight,
            data,
        )
        .context("load raw buffer as oxipng image")?;

        let mut opts = oxipng::Options::from_preset(compression);

        // explicitly allow modifying alpha, which gives another ~7% improvement on level 1
        opts.optimize_alpha = true;

        image_buffer
            .create_optimized_png(&opts)
            .context("create optimized png")
    }

    pub fn encode_png(&self) -> anyhow::Result<Vec<u8>> {
        let (width, height) = (self.width(), self.height());

        let mut png_bytes = vec![];

        let mut encoder = png::Encoder::new(&mut png_bytes, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_adaptive_filter(png::AdaptiveFilterType::Adaptive);

        let mut writer = encoder.write_header().context("write png header")?;
        writer
            .write_image_data(&self.data)
            .context("write png data")?;

        drop(writer);

        Ok(png_bytes)
    }

    pub fn encode_webp(&self) -> anyhow::Result<Vec<u8>> {
        let (width, height) = (self.width(), self.height());

        let encoder = webp::Encoder::new(&self.data, webp::PixelLayout::Rgba, width, height);
        let encoded = encoder.encode(75.);

        Ok(encoded.to_vec())
    }
}
