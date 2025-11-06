use std::{path::PathBuf, time::Duration};

use lofty::{
    file::{AudioFile, TaggedFile, TaggedFileExt},
    probe::Probe,
    tag::Tag,
};

const VALID_EXTENSIONS: &[&str; 5] = &["mp3", "wav", "m4a", "ogg", "flac"];

pub fn verify_path_extension(path: &PathBuf) -> Option<PathBuf> {
    let ext_str = path.extension()?.to_str()?;
    VALID_EXTENSIONS
        .iter()
        .find(|val_ext_str| val_ext_str.to_lowercase() == ext_str)
        .is_some()
        .then(|| return Some(path.clone()))?
}

pub fn get_total_duration(path: &PathBuf) -> Duration {
    let tagged_file = get_tagged_file(path);
    tagged_file.properties().duration()
}

pub fn get_tags(path: &PathBuf) -> Tag {
    let tagged_file = get_tagged_file(path);
    let tag = match tagged_file.primary_tag() {
        Some(t) => t,
        None => tagged_file.first_tag().expect("[x] Lofty: No tags found!"),
    };
    tag.clone()
}

fn get_tagged_file(path: &PathBuf) -> TaggedFile {
    Probe::open(path)
        .expect("[x] Lofty: Could not open path")
        .read()
        .expect("[x] Lofty: Could not read path")
}
