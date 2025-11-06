mod tui_input;
mod utils;
use crate::audio::{AudioPlayer, AudioSource};
use crate::cli::Cli;
use ratatui::prelude::*;
use ratatui::widgets::Block;
use std::path::PathBuf;
use std::time::Duration;
use utils::verify_path_extension;

#[derive(Default)]
pub struct App {
    args: Option<Cli>,
    audio: Option<AudioSource>,
    state: Option<AppState>,
}

impl App {
    pub fn from(cli: Cli) -> Self {
        let mut app = Self::default();
        match cli.get_command() {
            crate::cli::Command::Play { path } => {
                app.add_audio(path, cli.get_debug());
                app.state = Some(AppState::default());
            }
            crate::cli::Command::Player {} => todo!("Add the player"),
        }
        app.args = Some(cli);
        app
    }
    pub fn run(self) {
        let cli = self.args.expect("[x] Could not get CLI arguments");
        match cli.clone().get_command() {
            crate::cli::Command::Play { path: _ } => {
                if let Some(mut audio) = self.audio {
                    match audio.play() {
                        Ok(mut player) => {
                            self.state
                                .expect("[x] Could not get app state")
                                .set_title(audio.get_title())
                                .set_debug(cli.get_debug())
                                .run(&mut player);
                        }
                        Err(e) => eprintln!("{e}"),
                    }
                }
            }
            crate::cli::Command::Player {} => todo!("Implement Player"),
        }
    }
    #[doc = "Check if the provided path extension is an audio file and add it to Self"]
    fn add_audio(&mut self, path: PathBuf, debug: bool) {
        let valid_path = verify_path_extension(&path).expect("[x]Invalid File Extension provided");
        debug.then(|| println!("Path transmitted : {valid_path:?}"));
        self.audio = Some(AudioSource::from(path));
    }
}

#[derive(Debug, Default, Clone)]
struct AppState {
    running: bool,
    title: String,
    current_duration: Duration,
    debug: bool,
}

impl Widget for AppState {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inner_area = area.inner(Margin {
            horizontal: 10,
            vertical: 5,
        });
        let outer_area = area;

        Block::bordered()
            .title_top("Audio Player")
            .render(outer_area, buf);

        Block::bordered()
            .title_top(format!(
                "{} - {}",
                self.title,
                self.current_duration.as_secs().to_string()
            ))
            .render(inner_area, buf);
    }
}

impl AppState {
    pub fn run(&mut self, audio_player: &mut AudioPlayer) {
        self.debug.then(|| println!("[?] Entering the main loop"));
        self.running = true;
        self.debug.then(|| println!("[?] AppState {self:?}"));
        let mut term = ratatui::init();
        while self.running {
            audio_player.is_empty().then(|| self.stop());
            self.current_duration = audio_player.get_current_duration();
            self.debug.then(|| {
                println!(
                    "[?] Current Duration : {}",
                    self.current_duration.as_secs().to_string()
                )
            });
            term.draw(|frame| {
                self.draw(frame);
            })
            .is_err()
            .then(|| self.stop());
            tui_input::handle_event(audio_player, &mut self.running)
                .is_err()
                .then(|| self.stop());
        }
        self.debug.then(|| println!("[?]Restoring terminal"));
        ratatui::restore();
        self.debug.then(|| println!("[?]Exiting main loop"));
    }
    pub fn set_debug(&mut self, debug: bool) -> &mut Self {
        self.debug = debug;
        self
    }
    pub fn set_title(&mut self, title: impl Into<String>) -> &mut Self {
        self.title = title.into();
        self
    }
    fn draw(&mut self, f: &mut ratatui::Frame) {
        self.clone().render(f.area(), f.buffer_mut());
    }
    fn stop(&mut self) {
        self.running = false
    }
}
