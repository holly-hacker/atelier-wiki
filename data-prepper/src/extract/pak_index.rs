use std::{collections::HashMap, fs::File, io::Read, path::Path};

use anyhow::Context;
use gust_pak::{common::GameVersion, GustPak, PakEntry};
use tracing::{debug, trace};

pub struct PakIndex {
    /// Map whose key contains an index into the file vec, and the pak entry.
    pub map: HashMap<String, (usize, PakEntry)>,
    game_version: GameVersion,
    files: Vec<FileInfo>,
}

struct FileInfo {
    file: File,
    data_start: u64,
}

impl PakIndex {
    pub fn read(pak_dir: &Path, game_version: GameVersion) -> anyhow::Result<Self> {
        let mut map = HashMap::new();
        let mut files = vec![];

        // TODO: ensure order is correct!
        let data_dir = pak_dir.read_dir().context("read data dir")?;

        // only select the pak files
        let pak_files = data_dir.filter(|entry| {
            let entry = entry.as_ref().unwrap();
            entry.file_type().unwrap().is_file() && entry.path().extension() == Some("PAK".as_ref())
        });

        // NOTE: the actual insertion must be in file order, because some files will overwrite others
        // this is important if we paralellize this in the future
        let mut duplicate_count = 0;
        for entry in pak_files {
            // todo: read index
            let entry = entry.context("enumerate pak file")?;
            let pak_file_path = entry.path();
            debug!(?pak_file_path, "Reading pak file");
            let mut file = File::open(&pak_file_path).context("open pak file")?;

            let index =
                GustPak::read_index(&mut file, game_version).context("read pak file index")?;

            let file_info = FileInfo {
                file,
                data_start: index.get_data_start(),
            };
            let file_index = files.len();

            for pak_entry in index.entries.iter() {
                let owned_entry = pak_entry.into_owned();
                let file_name = owned_entry.as_ref().get_file_name().to_string();
                trace!(?pak_file_path, file_name, "Found pak entry");

                let old_value = map.insert(file_name, (file_index, owned_entry));
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

            files.push(file_info);
        }
        debug!("Overwrote existing entries {duplicate_count} times");

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
