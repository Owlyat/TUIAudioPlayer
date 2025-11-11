use std::{fs::File, io::BufReader, path::PathBuf, time::Duration};

use rodio::{
    Source,
    cpal::{self, traits::HostTrait},
};

pub struct AudioSource {
    title: String,
    file: File,
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
        }
    }
    pub fn play(
        &mut self,
        low_pass: Option<u32>,
        high_pass: Option<u32>,
        debug: bool,
    ) -> Result<AudioPlayer, rodio::PlayError> {
        let default_device = cpal::default_host()
            .default_output_device()
            .expect("[x] Rodio: Could not find output device");
        let mut stream_handle = rodio::OutputStreamBuilder::from_device(default_device)
            .expect("[x] Rodio: Could not user output device")
            .open_stream_or_fallback()
            .expect("Could not use audio device");
        let sink = rodio::Sink::connect_new(stream_handle.mixer());
        let decoder = rodio::Decoder::new(BufReader::new(
            self.file.try_clone().expect("[x] Could not clone file"),
        ))?;
        if let Some(low_pass) = low_pass {
            let src = decoder.low_pass(low_pass);
            if high_pass.is_some() {
                sink.append(src.high_pass(high_pass.unwrap()));
                return Ok(AudioPlayer::from(stream_handle, sink));
            }
            sink.append(src);
        } else {
            if high_pass.is_some() {
                sink.append(decoder.high_pass(high_pass.unwrap()));
                return Ok(AudioPlayer::from(stream_handle, sink));
            }

            sink.append(decoder);
        }
        stream_handle.log_on_drop(debug);
        Ok(AudioPlayer::from(stream_handle, sink))
    }
    pub fn get_title(&self) -> &str {
        &self.title
    }
}

pub struct AudioPlayer {
    #[allow(dead_code)]
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
    pub fn fast_forward(&mut self) {
        let _ = self
            .sink
            .try_seek(self.sink.get_pos() + Duration::from_secs(5));
    }
    pub fn rewind(&mut self) {
        let _ = self.sink.try_seek(Duration::from_secs(0));
    }
    pub fn faster_playback(&mut self) {
        self.sink
            .set_speed((self.sink.speed() + 0.1).clamp(0.1, 2.0));
    }
    pub fn slower_playback(&mut self) {
        self.sink
            .set_speed((self.sink.speed() - 0.1).clamp(0.1, 2.0));
    }
    pub fn higher_volume(&mut self) {
        self.sink
            .set_volume((self.sink.volume() + 0.1).clamp(0.0, 2.0));
    }
    pub fn lower_volume(&mut self) {
        self.sink
            .set_volume((self.sink.volume() - 0.1).clamp(0.0, 2.0));
    }
}
