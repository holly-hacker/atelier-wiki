use std::{
    collections::HashMap,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use anyhow::Context;
use gust_pak::{common::GameVersion, GustPak, PakEntry};
use rayon::prelude::*;
use tracing::{debug, trace};

pub struct PakIndex {
    /// Map whose key contains an index into the file vec, and the pak entry.
    pub map: HashMap<String, (usize, PakEntry)>,
    game_version: GameVersion,
    files: Vec<FileInfo>,
}

#[derive(Debug)]
struct FileInfo {
    /// The original filesystem path to the file
    path: PathBuf,
    /// A handle to the open file
    file: File,
    /// The index in the file where the data starts
    data_start: u64,
}

impl PakIndex {
    pub fn read(pak_dir: &Path, game_version: GameVersion) -> anyhow::Result<Self> {
        let data_dir = pak_dir.read_dir().context("read data dir")?;

        // only select the pak files
        let pak_files = data_dir
            .map(|d| d.unwrap())
            .filter(|entry| {
                entry.file_type().unwrap().is_file()
                    && entry.path().extension() == Some("PAK".as_ref())
            })
            .collect::<Vec<_>>();

        let mut map = HashMap::new();
        let mut files = vec![];
        let mut duplicate_count = 0;

        debug!("Reading pak files");
        let mut indices = pak_files
            .par_iter()
            .map(|pak_file| {
                let pak_file_path = pak_file.path();
                debug!(?pak_file_path, "Reading pak file");
                let mut file = File::open(&pak_file_path).context("open pak file")?;

                let index =
                    GustPak::read_index(&mut file, game_version).context("read pak file index")?;

                Ok((index, file, pak_file_path))
            })
            .collect::<anyhow::Result<Vec<_>>>()
            .context("read indices")?;

        // NOTE: the actual insertion must be in file order, because some files will overwrite others
        // this is important if we paralellize this in the future
        debug!("Sorting pak files");
        indices.sort_by(|(_, _, a), (_, _, b)| a.cmp(b));

        debug!("Reading pak entries into list");
        for (i, (index, file, pak_file_path)) in indices.into_iter().enumerate() {
            for pak_entry in index.entries.iter() {
                let owned_entry = pak_entry.into_owned();
                let file_name = owned_entry.as_ref().get_file_name().to_string();
                trace!(?pak_file_path, file_name, "Found pak entry");

                let old_value = map.insert(file_name, (i, owned_entry));
                if let Some((old_file_index, old_pak_entry)) = old_value.as_ref() {
                    duplicate_count += 1;

                    let size_diff = (pak_entry.get_file_size() as i64)
                        - (old_pak_entry.as_ref().get_file_size() as i64);
                    trace!(
                        ?old_file_index,
                        ?pak_file_path,
                        size_diff,
                        "Duplicate pak entry found"
                    );
                }
            }

            files.push(FileInfo {
                path: pak_file_path,
                file,
                data_start: index.get_data_start(),
            });
        }
        debug!("Overwrote existing entries {duplicate_count} times");

        // assert files are still sorted
        debug_assert!(files.windows(2).all(|w| w[0].path <= w[1].path));

        Ok(Self {
            map,
            game_version,
            files,
        })
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn get_file<'index>(
        &'index mut self,
        file_name: &str,
    ) -> std::io::Result<Option<impl Read + 'index>> {
        let Some((file_index, header)) = self.map.get_mut(file_name) else {
            return Ok(None);
        };

        let file = &mut self.files[*file_index];

        let data_start = file.data_start;

        Ok(Some(header.as_ref().get_reader_with_data_start(
            &mut file.file,
            data_start,
            self.game_version,
        )?))
    }
}
