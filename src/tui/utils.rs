use std::{path::PathBuf, time::Duration};

use lofty::{
    config::WriteOptions,
    file::{AudioFile, TaggedFile, TaggedFileExt},
    probe::Probe,
    tag::{Accessor, Tag, TagExt},
};

use crate::cli::Cli;

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

pub fn tag_writter(
    cli: &Cli,
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    genre: Option<String>,
    path: PathBuf,
) {
    let audio_p = verify_path_extension(&path).expect("[x] Invalid file or extension");
    let mut tagged_file = get_tagged_file(&audio_p);
    let tag = match tagged_file.primary_tag_mut() {
        Some(t) => t,
        None => {
            if let Some(t) = tagged_file.first_tag_mut() {
                t
            } else {
                let tag_type = tagged_file.primary_tag_type();
                cli.get_debug().then(|| {
                    eprintln!("[!] Lofty: No tags found, creating a new tag of type `{tag_type:?}`")
                });
                tagged_file.insert_tag(Tag::new(tag_type));
                tagged_file.primary_tag_mut().unwrap()
            }
        }
    };
    if title.is_none() && artist.is_none() && album.is_none() && genre.is_none() {
        let title =
            inquire::prompt_text("Track Title: ").expect("[x] Inquire: Failed to retrieve title");
        (!title.is_empty()).then(|| tag.set_title(title));
        let artist = inquire::prompt_text("Artist Name: ")
            .expect("[x] Inquire: Failed to retrieve Artist Name");
        (!artist.is_empty()).then(|| tag.set_artist(artist));
        let album = inquire::prompt_text("Album Name: ")
            .expect("[x] Inquire: Failed to retrieve Album name");
        (!album.is_empty()).then(|| tag.set_album(album));
        let genre = inquire::prompt_text("Genre: ").expect("[x] Inquire: Failed to retrieve Genre");
        (!genre.is_empty()).then(|| tag.set_album(genre));
        tag.save_to_path(path, WriteOptions::default())
            .expect("[x] Lofty: Failed to write the tag");
        std::process::exit(0);
    }
    if let Some(title) = title {
        tag.set_title(title.clone());
        cli.get_debug()
            .then(|| println!("[!] Title tag set to {}", title));
    }
    if let Some(artist) = artist {
        tag.set_artist(artist.clone());
        cli.get_debug()
            .then(|| println!("[!] Artist tag set to {}", artist));
    }
    if let Some(album) = album {
        tag.set_album(album.clone());
        cli.get_debug()
            .then(|| println!("[!] Album tag set to {}", album));
    }
    if let Some(genre) = genre {
        tag.set_genre(genre.clone());
        cli.get_debug()
            .then(|| println!("[!] Genre tag set to {}", genre));
    }
    tag.save_to_path(path, WriteOptions::default())
        .expect("[x] Lofty: Failed to write the tag");
    std::process::exit(0);
}
