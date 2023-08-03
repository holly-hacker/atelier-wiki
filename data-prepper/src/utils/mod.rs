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

/// Match a pattern with a wildcard `*` against a string.
pub fn match_pattern(needle: &'static str, haystack: &str) -> Option<usize> {
    let Some(index) = needle.find('*') else {
        panic!("pattern `{needle}` does not contain a `*`, which is required");
    };

    let left = &needle[..index];
    let right = &needle[index + 1..];

    if !(haystack.starts_with(left) && haystack.ends_with(right)) {
        return None;
    }

    let matched = &haystack[left.len()..haystack.len() - right.len()];

    matched.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    // NOTE: this should be a doctest, see rust-lang/rust#50784
    #[test]
    fn match_pattern_is_correct() {
        assert_eq!(match_pattern("aa_*_bb", "aa_123_bb"), Some(123));
        assert_eq!(match_pattern("aa_*_bb", "foo"), None);

        // failed parsing returns None
        assert_eq!(match_pattern("aa_*", "aa_123_bb"), None);
        assert_eq!(match_pattern("*_bbb", "aa_123_bb"), None);
    }
}
