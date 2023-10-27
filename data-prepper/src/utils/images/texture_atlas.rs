#![allow(unused)]

use anyhow::{bail, Context};
use serde::Serialize;
use typescript_type_def::TypeDef;

use super::rgba8_image::Rgba8Image;

/// A texture atlas that stores images of the same size, with optional downscaling.
pub struct UniformTextureAtlas {
    image: Rgba8Image,

    /// The dimensions of the original images, before they are inserted.
    original_dimensions: (u32, u32),

    /// The factor by which the original images are scaled down.
    scaling_factor: (u32, u32),

    /// An ordered list of the images that have been stored already.
    image_names: Vec<String>,
}

#[derive(Serialize, TypeDef)]
/// Information about a texture atlas.
pub struct UniformTextureAtlasInfo {
    /// The number of columns in the texture atlas.
    pub columns: u32,
    /// The dimensions of each image.
    pub image_dimensions: (u32, u32),
    /// An ordered list of the images that are stored in this texture atlas.
    pub stored_images: Vec<String>,
}

impl UniformTextureAtlas {
    pub fn new_with_scaling(
        original_dimensions: (u32, u32),
        new_dimensions: (u32, u32),
        capacity: usize,
    ) -> anyhow::Result<Self> {
        let (columns, rows) = calculate_pack_size(capacity);

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
            image_names: vec![],
        };

        debug_assert_eq!(ret.columns(), columns);
        debug_assert_eq!(ret.image_dimensions(), new_dimensions);

        Ok(ret)
    }

    /// Add a new image to the texture atlas.
    ///
    /// This function can return an error if the input image has incorrect dimensions or if the
    /// texture atlas is "full".
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

        let new_index = self.image_names.len();
        let columns = self.columns();
        let (index_x, index_y) = (new_index % columns, new_index / columns);
        let (image_width, image_height) = self.image_dimensions();
        let (image_x, image_y) = (index_x as u32 * image_width, index_y as u32 * image_height);

        // ensure image is not out of bounds (this happens if we exceed capacity)
        debug_assert!(image_x < self.image.width());
        if image_y >= self.image.height() {
            bail!(
                "image does not fit in texture atlas: image dimensions: {:?}, texture atlas dimensions: {:?}",
                (image_x, image_y),
                (self.image.width(), self.image.height())
            );
        }
        debug_assert!(image_x + image_width <= self.image.width());
        debug_assert!(image_y + image_height <= self.image.height());

        self.image_names.push(name);

        // scale the image and blit it to the texture atlas
        let scaled_image = image.scale_down(self.scaling_factor);
        self.image
            .blit(image_x, image_y, &scaled_image)
            .context("blit image")?;

        Ok(())
    }

    pub fn into_image(self) -> Rgba8Image {
        self.image
    }

    pub fn create_info(&self) -> UniformTextureAtlasInfo {
        UniformTextureAtlasInfo {
            columns: self.columns() as u32,
            image_dimensions: self.image_dimensions(),
            stored_images: self.image_names.clone(),
        }
    }

    fn columns(&self) -> usize {
        (self.image.width() / (self.original_dimensions.0 / self.scaling_factor.0)) as usize
    }

    fn rows(&self) -> usize {
        (self.image.height() / (self.original_dimensions.1 / self.scaling_factor.1)) as usize
    }

    /// Gets the dimension of a single image in the texture atlas.
    fn image_dimensions(&self) -> (u32, u32) {
        (
            (self.original_dimensions.0 / self.scaling_factor.0),
            (self.original_dimensions.1 / self.scaling_factor.1),
        )
    }
}

fn calculate_pack_size(capacity: usize) -> (usize, usize) {
    let square_width = (capacity as f32).sqrt().ceil() as usize;
    let square_heigth = (capacity as f32 / square_width as f32).ceil() as usize;

    (square_width, square_heigth)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_pack_size_test() {
        assert_eq!(calculate_pack_size(1), (1, 1));
        assert_eq!(calculate_pack_size(16), (4, 4));
        assert_eq!(calculate_pack_size(15), (4, 4));
        assert_eq!(calculate_pack_size(17), (5, 4));
    }
}
