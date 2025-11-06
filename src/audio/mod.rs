use std::{fs::File, io::BufReader, path::PathBuf, time::Duration};

pub struct AudioSource {
    title: String,
    file: File,
    player: Option<AudioPlayer>,
}

impl AudioSource {
    pub fn from(path: PathBuf) -> Self {
        Self {
            title: path
                .to_str()
                .expect("[x] Could not convert title to string")
                .to_string(),
            file: File::open(path.clone()).expect(&format!(
                "[x] Error while opening file {}",
                path.to_string_lossy()
            )),
            player: None,
        }
    }
    pub fn play(&mut self) -> Result<AudioPlayer, rodio::PlayError> {
        let stream_handle =
            rodio::OutputStreamBuilder::open_default_stream().expect("Could not use audio device");
        let sink = rodio::play(
            stream_handle.mixer(),
            BufReader::new(self.file.try_clone().expect("Could not clone file")),
        )?;
        Ok(AudioPlayer::from(stream_handle, sink))
    }
    pub fn get_title(&self) -> &str {
        &self.title
    }
}

pub struct AudioPlayer {
    handle: rodio::OutputStream,
    sink: rodio::Sink,
}

impl AudioPlayer {
    fn from(h: rodio::OutputStream, s: rodio::Sink) -> Self {
        Self { handle: h, sink: s }
    }
    pub fn is_empty(&self) -> bool {
        self.sink.empty()
    }
    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }
    pub fn get_current_duration(&self) -> Duration {
        self.sink.get_pos()
    }
    pub fn pause(&mut self) {
        self.sink.pause();
    }
    pub fn play(&mut self) {
        self.sink.play();
    }
}
