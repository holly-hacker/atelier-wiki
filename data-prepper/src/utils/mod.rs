pub mod images;
mod pak_index;
mod xml_reader;

use std::{path::Path, str::FromStr};

use gust_pak::common::GameVersion;
pub use pak_index::PakIndex;
pub use xml_reader::{read_xml, read_xml_shift_jis, ElementReader};

pub fn extract_game_version(path: &Path) -> Option<GameVersion> {
    if path.join("Atelier_Sophie_DX.exe").exists() {
        Some(GameVersion::A17)
    } else if path.join("Atelier_Firis_DX.exe").exists() {
        Some(GameVersion::A18)
    } else if path.join("Atelier_Lydie_and_Suelle_DX.exe").exists() {
        Some(GameVersion::A19)
    } else if path.join("Atelier_Ryza.exe").exists() {
        Some(GameVersion::A21)
    } else if path.join("Atelier_Ryza_2.exe").exists() {
        Some(GameVersion::A22)
    } else if path.join("Atelier_Sophie_2.exe").exists() {
        Some(GameVersion::A23)
    } else if path.join("Atelier_Ryza_3.exe").exists() {
        Some(GameVersion::A24)
    } else {
        None
    }
}

pub fn game_slug(game_version: GameVersion) -> &'static str {
    match game_version {
        GameVersion::A17 => "sophie",
        GameVersion::A18 => "firis",
        GameVersion::A19 => "lydiesuelle",
        GameVersion::A21 => "ryza",
        GameVersion::A22 => "ryza2",
        GameVersion::A23 => "sophie2",
        GameVersion::A24 => "ryza3",
    }
}

/// Match a pattern with a wildcard `*` against a string.
pub fn match_pattern<T>(needle: &'static str, haystack: &str) -> Option<T>
where
    T: FromStr,
{
    match_pattern_str(needle, haystack).and_then(|found| found.parse().ok())
}

pub fn match_pattern_str<'input>(
    needle: &'static str,
    haystack: &'input str,
) -> Option<&'input str> {
    let Some(index) = needle.find('*') else {
        panic!("pattern `{needle}` does not contain a `*`, which is required");
    };

    let left = &needle[..index];
    let right = &needle[index + 1..];

    if !(haystack.starts_with(left) && haystack.ends_with(right)) {
        return None;
    }

    let matched = &haystack[left.len()..haystack.len() - right.len()];

    Some(matched)
}

#[cfg(test)]
mod tests {
    use super::*;

    // NOTE: this should be a doctest, see rust-lang/rust#50784
    #[test]
    fn match_pattern_is_correct() {
        assert_eq!(match_pattern::<usize>("aa_*_bb", "aa_123_bb"), Some(123));
        assert_eq!(match_pattern::<usize>("aa_*_bb", "foo"), None);

        // failed parsing returns None
        assert_eq!(match_pattern::<usize>("aa_*", "aa_123_bb"), None);
        assert_eq!(match_pattern::<usize>("*_bbb", "aa_123_bb"), None);

        // &str has its own method
        assert_eq!(match_pattern_str("aa_*_bb", "aa_123_bb"), Some("123"));
        assert_eq!(match_pattern_str("aa_*_bb", "foo"), None);
    }
}
