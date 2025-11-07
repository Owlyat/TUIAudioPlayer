mod tui_input;
mod utils;
use crate::audio::{AudioPlayer, AudioSource};
use crate::cli::Cli;
use lofty::tag::Accessor;
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, LineGauge, Paragraph};
use std::fmt::Debug;
use std::path::PathBuf;
use std::time::Duration;
use utils::{get_sample_rate, verify_path_extension};

#[derive(Default)]
pub struct App {
    args: Option<Cli>,
    audio: Option<AudioSource>,
    state: Option<AppStatePlay>,
}

impl App {
    pub fn from(cli: Cli) -> Self {
        let mut app = Self::default();
        match cli.get_command() {
            crate::cli::Command::Play {
                path,
                low_pass: _,
                high_pass: _,
            } => {
                app.add_audio(path, cli.get_debug());
                app.state = Some(AppStatePlay::default());
            }
            crate::cli::Command::Player {} => todo!("Add the player"),
        }
        app.args = Some(cli);
        app
    }
    pub fn run(self) {
        let cli = self.args.expect("[x] Could not get CLI arguments");
        match cli.clone().get_command() {
            crate::cli::Command::Play {
                path,
                low_pass,
                high_pass,
            } => {
                let tag = utils::get_tags(&path);
                if let Some(mut audio) = self.audio {
                    match audio.play(low_pass, high_pass) {
                        Ok(mut player) => {
                            self.state
                                .expect("[x] Could not get app state")
                                .set_title(audio.get_title())
                                .set_artist(tag.artist().unwrap_or_default())
                                .set_album(tag.album().unwrap_or_default())
                                .set_genre(tag.genre().unwrap_or_default())
                                .set_sample_rate(get_sample_rate(&path))
                                .set_total_duration(utils::get_total_duration(&path))
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
struct AppStatePlay {
    running: bool,
    title: String,
    current_duration: Duration,
    total_duration: Duration,
    artist: String,
    album: String,
    genre: String,
    sample_rate: String,
    debug: bool,
}

impl Widget for AppStatePlay {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inner_area = area.inner(Margin {
            horizontal: 1,
            vertical: 1,
        });
        let outer_area = area;

        Block::bordered()
            .title_top("[TUI Audio Player]")
            .title_alignment(Alignment::Center)
            .title_style(Style::default().fg(Color::Blue))
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::White))
            .render(outer_area, buf);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(inner_area.inner(Margin {
                horizontal: 10,
                vertical: 10,
            }));

        Block::bordered()
            .style(Style::default().fg(Color::Blue))
            .title(format!(
                "{} - {:02}:{:02}/{:02}:{:02}",
                self.title,
                (self.current_duration.as_secs() - (self.current_duration.as_secs() % 60)) / 60,
                self.current_duration.as_secs() % 60,
                (self.total_duration.as_secs() - (self.total_duration.as_secs() % 60)) / 60,
                self.total_duration.as_secs() % 60,
            ))
            .title_bottom("[Volume Up ▲ | Volume Down ▼ | Fast Forward ▶ | Rewind ◀ | Slow Down <Shift> + ▼ | Speed Up <Shift> + ▲]")
            .render(inner_area, buf);

        Paragraph::new(format!(
            "Artist: {}\nAlbum: {}\nGenre: {}\nSample Rate: {}\nTotal Duration: {:02}:{:02}",
            self.artist
                .is_empty()
                .then(|| "<None>")
                .or(Some(&self.artist))
                .unwrap(),
            self.album
                .is_empty()
                .then(|| "<None>")
                .or(Some(&self.album))
                .unwrap(),
            self.genre
                .is_empty()
                .then(|| "<None>")
                .or(Some(&self.genre))
                .unwrap(),
            self.sample_rate
                .is_empty()
                .then(|| "<None>")
                .or(Some(&self.sample_rate))
                .unwrap(),
            (self.total_duration.as_secs() - (self.total_duration.as_secs() % 60)) / 60,
            self.total_duration.as_secs() % 60,
        ))
        .style(Style::default().fg(Color::Yellow))
        .centered()
        .render(layout[0], buf);

        LineGauge::default()
            .line_set(symbols::line::THICK)
            .filled_style(Style::default().fg(Color::Blue))
            .unfilled_style(Style::default().fg(Color::Black))
            .ratio(self.current_duration.as_secs_f64() / self.total_duration.as_secs_f64())
            .render(layout[1], buf);
    }
}

impl AppStatePlay {
    pub fn run(&mut self, audio_player: &mut AudioPlayer) {
        self.debug.then(|| println!("[?] Entering the main loop"));
        self.running = true;
        self.debug.then(|| println!("[?] AppState {self:?}"));
        let mut term = ratatui::init();
        while self.running {
            audio_player.is_empty().then(|| self.stop());
            self.current_duration = audio_player.get_current_duration();
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
    pub fn set_artist(&mut self, artist: impl Into<String>) -> &mut Self {
        self.artist = artist.into();
        self
    }
    pub fn set_album(&mut self, album: impl Into<String>) -> &mut Self {
        self.album = album.into();
        self
    }
    pub fn set_genre(&mut self, genre: impl Into<String>) -> &mut Self {
        self.genre = genre.into();
        self
    }
    pub fn set_sample_rate(&mut self, sample_rate: impl Into<String>) -> &mut Self {
        self.sample_rate = sample_rate.into();
        self
    }
    fn draw(&mut self, f: &mut ratatui::Frame) {
        self.clone().render(f.area(), f.buffer_mut());
    }
    fn stop(&mut self) {
        self.running = false
    }
    pub fn set_total_duration(&mut self, d: Duration) -> &mut Self {
        self.total_duration = d;
        self
    }
}
