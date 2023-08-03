mod pak_index;

use std::path::Path;

use gust_pak::common::GameVersion;

pub use pak_index::PakIndex;

pub fn extract_game_version(path: &Path) -> Option<GameVersion> {
    // currently, only detect Atelier Ryza 3. we can add more later
    if path.join("Atelier_Ryza_3.exe").exists() {
        Some(GameVersion::A24)
    } else {
        None
    }
}
