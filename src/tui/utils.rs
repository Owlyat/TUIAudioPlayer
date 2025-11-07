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

pub fn get_sample_rate(path: &PathBuf) -> String {
    let tagged_file = get_tagged_file(path);
    format!(
        "{}{}",
        tagged_file.properties().sample_rate().unwrap_or_default(),
        tagged_file
            .properties()
            .sample_rate()
            .is_some()
            .then(|| "k")
            .unwrap()
    )
}

pub fn get_tags(path: &PathBuf) -> Tag {
    let mut tagged_file = get_tagged_file(path);
    let tag = match tagged_file.primary_tag() {
        Some(t) => t,
        None => {
            if let Some(tag) = tagged_file.first_tag() {
                tag
            } else {
                let tag_type = tagged_file.primary_tag_type();
                tagged_file.insert_tag(Tag::new(tag_type));
                tagged_file
                    .primary_tag_mut()
                    .expect("[x] Lofty: Error while applying new tag on media")
            }
        }
    };
    tag.clone()
}

pub fn get_tagged_file(path: &PathBuf) -> TaggedFile {
    Probe::open(path)
        .expect("[x] Lofty: Could not open path")
        .read()
        .expect("[x] Lofty: Could not read path")
}
