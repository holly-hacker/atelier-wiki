#![allow(unused)]

use anyhow::{bail, Context};

use super::rgba8_image::Rgba8Image;

/// A packed image, also known as a spritesheet.
pub struct PackedImage {
    image: Rgba8Image,

    /// The dimensions of the original images
    original_dimensions: (u32, u32),

    /// The factor by which the original images are scaled down.
    scaling_factor: (u32, u32),

    /// An ordered list of the images that have been stored already.
    stored_images: Vec<String>,
}

impl PackedImage {
    pub fn new(
        original_dimensions: (u32, u32),
        new_dimensions: (u32, u32),
        total_image_count: usize,
    ) -> anyhow::Result<Self> {
        let (columns, rows) = calculate_pack_size(total_image_count);

        if original_dimensions.0 % new_dimensions.0 != 0
            || original_dimensions.1 % new_dimensions.1 != 0
        {
            bail!(
                "original dimensions must be a multiple of the new dimensions: {:?} % {:?} != 0",
                original_dimensions,
                new_dimensions
            );
        }

        let scaling_factor = (
            (original_dimensions.0 / new_dimensions.0),
            (original_dimensions.1 / new_dimensions.1),
        );
        let image = Rgba8Image::new_empty(
            new_dimensions.0 * columns as u32,
            new_dimensions.1 * rows as u32,
        );

        let ret = Self {
            image,
            original_dimensions,
            scaling_factor,
            stored_images: vec![],
        };

        debug_assert_eq!(ret.get_columns(), columns);
        debug_assert_eq!(ret.get_image_dimensions(), new_dimensions);

        Ok(ret)
    }

    /// Add a new image to the packed image.
    ///
    /// This function can return an error if the input image has incorrect dimensions or if the
    /// packed image is "full".
    pub fn add_image(&mut self, image: &Rgba8Image, name: String) -> anyhow::Result<()> {
        if image.width() != self.original_dimensions.0
            || image.height() != self.original_dimensions.1
        {
            bail!(
                "image dimensions do not match: expected {:?}, got {:?}",
                self.original_dimensions,
                (image.width(), image.height())
            );
        }

        let new_index = self.stored_images.len();
        let columns = self.get_columns();
        let (index_x, index_y) = (new_index % columns, new_index / columns);
        let (image_width, image_height) = self.get_image_dimensions();
        let (image_x, image_y) = (index_x as u32 * image_width, index_y as u32 * image_height);

        // ensure image is not out of bounds
        debug_assert!(image_x < self.image.width());
        if image_y >= self.image.height() {
            bail!(
                "image does not fit in packed image: image dimensions: {:?}, packed image dimensions: {:?}",
                (image_x, image_y),
                (self.image.width(), self.image.height())
            );
        }
        debug_assert!(image_x + image_width <= self.image.width());
        debug_assert!(image_y + image_height <= self.image.height());

        self.stored_images.push(name);

        // scale the image and blit it to the packed image
        let scaled_image = image.scale_down(self.scaling_factor);
        self.image
            .blit(image_x, image_y, &scaled_image)
            .context("blit image")?;

        Ok(())
    }

    pub fn take_image(self) -> Rgba8Image {
        self.image
    }

    fn get_columns(&self) -> usize {
        (self.image.width() / (self.original_dimensions.0 / self.scaling_factor.0)) as usize
    }

    fn get_rows(&self) -> usize {
        (self.image.height() / (self.original_dimensions.1 / self.scaling_factor.1)) as usize
    }

    /// Gets the dimension of a single image in the packed image.
    fn get_image_dimensions(&self) -> (u32, u32) {
        (
            (self.original_dimensions.0 / self.scaling_factor.0),
            (self.original_dimensions.1 / self.scaling_factor.1),
        )
    }
}

fn calculate_pack_size(count: usize) -> (usize, usize) {
    let square_width = (count as f32).sqrt().ceil() as usize;
    let square_heigth = (count as f32 / square_width as f32).ceil() as usize;

    (square_width, square_heigth)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_pack_size_test() {
        assert_eq!(calculate_pack_size(1), (1, 1));
        assert_eq!(calculate_pack_size(16), (4, 4));
        assert_eq!(calculate_pack_size(15), (5, 3));
        assert_eq!(calculate_pack_size(17), (17, 1));
    }
}
