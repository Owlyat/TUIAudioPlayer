use std::path::PathBuf;

const VALID_EXTENSIONS: &[&str; 5] = &["mp3", "wav", "m4a", "ogg", "flac"];

pub fn verify_path_extension(path: &PathBuf) -> Option<PathBuf> {
    let ext_str = path.extension()?.to_str()?;
    VALID_EXTENSIONS
        .iter()
        .find(|val_ext_str| val_ext_str.to_lowercase() == ext_str)
        .is_some()
        .then(|| return Some(path.clone()))?
}
