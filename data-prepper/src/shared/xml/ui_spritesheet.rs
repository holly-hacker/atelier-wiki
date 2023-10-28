use anyhow::Context;
use gust_g1t::GustG1t;

use crate::utils::images::rgba8_image::Rgba8Image;
use crate::utils::{self, ElementReader, PakIndex};

/// Sprite sheet info usually found in the `gen_styles` folder, derived from the
/// `file://st01/project/A17/_Programmer/ui/uis.dtd` file.
pub struct UiSpritesheetInfo {
    #[allow(unused)]
    resolution_hd: usize,
    images: Vec<ImageOrImageList>,
}

enum ImageOrImageList {
    Image(ImageInfo),
    ImageList(ImageListInfo),
}

/// Represents an `image` tag.
struct ImageInfo {
    pub name: String,
    pub texture: String,
    pub texture_index: usize,
    pub uvwh: (u32, u32, u32, u32),
}

/// Represents an `image_list` tag which contains `image` children.
struct ImageListInfo {
    pub name: String,
    pub images: Vec<ImageInfo>,
}

impl UiSpritesheetInfo {
    pub fn read(pak_index: &mut PakIndex, path: &str) -> anyhow::Result<Self> {
        utils::read_xml(pak_index, path, Self::read_from_doc)
    }

    pub fn get_image(&self, pak_index: &mut PakIndex, name: &str) -> anyhow::Result<Rgba8Image> {
        let image_info = self
            .images
            .iter()
            .filter_map(|i| match i {
                ImageOrImageList::Image(ref i) => Some(i),
                ImageOrImageList::ImageList(_) => None,
            })
            .find(|l| l.name == name)
            .with_context(|| format!("find image with name `{name}`"))?;

        self.extract_image(pak_index, image_info)
    }

    pub fn get_image_indexed(
        &self,
        pak_index: &mut PakIndex,
        name: &str,
        index: usize,
    ) -> anyhow::Result<Rgba8Image> {
        let list = self
            .images
            .iter()
            .filter_map(|i| match i {
                ImageOrImageList::Image(_) => None,
                ImageOrImageList::ImageList(ref l) => Some(l),
            })
            .find(|l| l.name == name)
            .with_context(|| format!("find image list with name `{name}`"))?;

        let image_info = list
            .images
            .get(index)
            .with_context(|| format!("get image at index `{index}`"))?;

        self.extract_image(pak_index, image_info)
    }

    fn extract_image(
        &self,
        pak_index: &mut PakIndex,
        image_info: &ImageInfo,
    ) -> anyhow::Result<Rgba8Image> {
        // TODO: get prefix based on pak_index.game_version
        // TODO: allow checking res_{en,jp,...} as well. language parameter?
        const PREFIX: &str = r"\data\x64\res_cmn\";

        let path = format!("{PREFIX}{}", image_info.texture.replace('/', "\\"));
        let path = make_neo(&path);
        let mut file = pak_index
            .get_file(&path)
            .with_context(|| format!("get file `{}` from pak_index", path))?
            .with_context(|| format!("find file `{}` in pak_index", path))?;

        let g1t = GustG1t::read(&mut file).context("read g1t file")?;
        let texture = g1t
            .textures
            .get(image_info.texture_index)
            .with_context(|| format!("get texture {}", image_info.texture_index))?;

        let image_data = g1t
            .read_image(texture, file)
            .context("read image from g1t")?;

        let spritesheet = Rgba8Image::new(texture.width, image_data)?;

        // extract the sprite from the spritesheet
        // TODO: in ryza3, uvwh coordinates are only valid for `neo` files. other games are unchecked
        let (u, v, w, h) = image_info.uvwh;
        let image = spritesheet
            .copy_chunk(u, v, w, h)
            .context("extract image from spritesheet")?;

        Ok(image)
    }

    fn read_from_doc(document: roxmltree::Document) -> anyhow::Result<Self> {
        let root = document.root_element();

        let resolution_hd: usize = root
            .attribute("resolution_hd")
            .context("find resolution_hd on root element")?
            .parse()
            .context("parse resolution_hd")?;

        println!("children: {}", root.children().count());

        let images = root
            .children()
            .filter(|child| child.is_element())
            .map(|child| {
                Ok(match child.tag_name().name() {
                    "image" => ImageOrImageList::Image(ImageInfo::read_from_element(child)?),
                    "image_list" => {
                        ImageOrImageList::ImageList(ImageListInfo::read_from_element(child)?)
                    }
                    unk => panic!("unknown tag name: {}", unk),
                })
            })
            .collect::<anyhow::Result<_>>()?;

        Ok(Self {
            resolution_hd,
            images,
        })
    }
}

impl ImageInfo {
    fn read_from_element(element: roxmltree::Node) -> anyhow::Result<Self> {
        let reader = ElementReader(&element);

        let name = reader.read("name")?;
        let texture = reader.read("texture")?;
        let texture_index = reader.read("texture_index")?;
        let uvwh = reader.read::<String>("uvwh")?;

        let (u, vwh) = uvwh.split_once(',').context("split u,vwh")?;
        let (v, wh) = vwh.split_once(',').context("split v,wh")?;
        let (w, h) = wh.split_once(',').context("split w,h")?;
        let uvwh = (
            u.parse().context("parse `u`")?,
            v.parse().context("parse `v`")?,
            w.parse().context("parse `w`")?,
            h.parse().context("parse `h`")?,
        );

        Ok(Self {
            name,
            texture,
            texture_index,
            uvwh,
        })
    }
}

impl ImageListInfo {
    fn read_from_element(element: roxmltree::Node) -> anyhow::Result<Self> {
        let name = element
            .attribute("name")
            .context("get name attribute on image_list")?
            .to_string();

        let images = element
            .children()
            .filter(|child| child.is_element())
            .map(|child| {
                Ok(match child.tag_name().name() {
                    "image" => ImageInfo::read_from_element(child)?,
                    unk => panic!("unknown tag name: {}", unk),
                })
            })
            .collect::<anyhow::Result<_>>()?;

        Ok(Self { name, images })
    }
}

fn make_neo(path: &str) -> String {
    let mut split = path.split('\\').collect::<Vec<_>>();

    let last = split.pop().unwrap();
    let last = format!("neo\\neo_{last}");
    split.push(&last);

    split.join("\\")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_make_neo() {
        assert_eq!(
            make_neo(r"\data\x64\res_cmn\ui\a24_icons.g1t"),
            r"\data\x64\res_cmn\ui\neo\neo_a24_icons.g1t"
        );
    }
}
