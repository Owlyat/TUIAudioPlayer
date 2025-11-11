use std::time::Duration;

use lofty::tag::Accessor;
use ratatui::crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};

use crate::audio::{AudioPlayer, AudioSource};

use super::{
    AppStatePlay, PlayerSelection,
    utils::{self, get_sample_rate, verify_path_extension},
};

pub fn handle_play_event(
    audio_player: &mut AudioPlayer,
    running: &mut bool,
) -> Result<(), std::io::Error> {
    if ratatui::crossterm::event::poll(Duration::from_millis(0))? {
        let event = ratatui::crossterm::event::read()?;
        match event {
            ratatui::crossterm::event::Event::FocusGained => {}
            ratatui::crossterm::event::Event::FocusLost => {}
            ratatui::crossterm::event::Event::Key(key_event) => {
                (key_event.code == KeyCode::Char('q')).then(|| *running = false);
                play_key_input(audio_player, key_event);
            }
            ratatui::crossterm::event::Event::Mouse(_mouse_event) => {}
            ratatui::crossterm::event::Event::Paste(_) => {}
            ratatui::crossterm::event::Event::Resize(_, _) => {}
        }
    }
    Ok(())
}

pub fn play_key_input(
    audio_player: &mut AudioPlayer,
    key_event: ratatui::crossterm::event::KeyEvent,
) {
    (key_event.code == KeyCode::Enter && key_event.kind == KeyEventKind::Press).then(|| {
        if audio_player.is_paused() {
            audio_player.play()
        } else {
            audio_player.pause();
        }
    });
    (key_event.code == KeyCode::Right && key_event.kind == KeyEventKind::Press)
        .then(|| audio_player.fast_forward());
    (key_event.code == KeyCode::Left && key_event.kind == KeyEventKind::Press)
        .then(|| audio_player.rewind());
    (key_event.code == KeyCode::Up
        && key_event.kind == KeyEventKind::Press
        && key_event.modifiers == KeyModifiers::empty())
    .then(|| audio_player.higher_volume());
    (key_event.code == KeyCode::Down
        && key_event.kind == KeyEventKind::Press
        && key_event.modifiers == KeyModifiers::empty())
    .then(|| audio_player.lower_volume());
    (key_event.code == KeyCode::Up
        && key_event.kind == KeyEventKind::Press
        && key_event.modifiers == KeyModifiers::SHIFT)
        .then(|| audio_player.faster_playback());
    (key_event.code == KeyCode::Down
        && key_event.kind == KeyEventKind::Press
        && key_event.modifiers == KeyModifiers::SHIFT)
        .then(|| audio_player.slower_playback());
}

pub fn handle_player_event(
    running: &mut bool,
    file_explorer: &mut ratatui_explorer::FileExplorer,
    which: &mut PlayerSelection,
    audio_source: &mut Option<AudioSource>,
    app_player: &mut Option<AudioPlayer>,
    app_state_play: &mut AppStatePlay,
    debug: bool,
) -> Result<(), std::io::Error> {
    if ratatui::crossterm::event::poll(Duration::from_millis(0))? {
        let event = ratatui::crossterm::event::read()?;
        //
        match which {
            PlayerSelection::FileExplorer => {
                match event {
                    ratatui::crossterm::event::Event::FocusGained => {}
                    ratatui::crossterm::event::Event::FocusLost => {}
                    ratatui::crossterm::event::Event::Key(key_event) => {
                        (key_event.code == KeyCode::Char('q')
                            && key_event.kind == KeyEventKind::Press
                            && key_event.modifiers == KeyModifiers::empty())
                        .then(|| {
                            if audio_source.is_some() {
                                debug.then(|| println!("[?] Removing audio from player"));
                                *audio_source = None;
                                *app_player = None;
                                *app_state_play = AppStatePlay::default();
                                return;
                            } else {
                                debug.then(|| println!("[?] Quitting player"));
                                *running = false
                            }
                        });
                        (key_event.code == KeyCode::Tab
                            && key_event.kind == KeyEventKind::Press
                            && key_event.modifiers == KeyModifiers::empty()
                            && audio_source.is_some())
                        .then(|| {
                            debug.then(|| println!("[?]Switching tab"));
                            which.toggle()
                        });

                        (key_event.code == KeyCode::Enter
                            && key_event.kind == KeyEventKind::Press
                            && key_event.modifiers == KeyModifiers::empty())
                        .then(|| {
                            which.toggle();
                            let path = verify_path_extension(file_explorer.current().path())
                                .expect("[x] App: Invalid file selected");
                            *audio_source = Some(AudioSource::from(path.clone()));
                            debug.then(|| {
                                audio_source
                                    .is_some()
                                    .then(|| println!("[+] AudioSource created"));
                                audio_source
                                    .is_none()
                                    .then(|| println!("[x] Failed to create audio source"));
                                println!("[?] Valid Audio Source created")
                            });
                            if let Some(audio) = audio_source {
                                let tag = utils::get_tags(&path);
                                match audio.play(None, None, debug) {
                                    Ok(player) => {
                                        *app_player = Some(player);
                                        app_state_play
                                            .set_full_title(audio.get_title())
                                            .set_filename(tag.title().unwrap_or_default())
                                            .set_artist(tag.artist().unwrap_or_default())
                                            .set_album(tag.album().unwrap_or_default())
                                            .set_genre(tag.genre().unwrap_or_default())
                                            .set_sample_rate(get_sample_rate(&path))
                                            .set_total_duration(utils::get_total_duration(&path))
                                            .set_debug(debug);
                                    }
                                    Err(e) => panic!("{e}"),
                                }
                            }
                        });
                    }
                    ratatui::crossterm::event::Event::Mouse(_mouse_event) => {}
                    ratatui::crossterm::event::Event::Paste(_) => {}
                    ratatui::crossterm::event::Event::Resize(_, _) => {}
                }
                file_explorer.handle(&event)?
            }
            PlayerSelection::AudioPlayer => {
                audio_source.is_none().then(|| which.toggle());

                match event {
                    ratatui::crossterm::event::Event::Key(key_event) => {
                        (key_event.code == KeyCode::Char('q')
                            && key_event.kind == KeyEventKind::Press
                            && key_event.modifiers == KeyModifiers::empty())
                        .then(|| {
                            if audio_source.is_some() {
                                debug.then(|| println!("[?] Removing audio from player"));
                                *audio_source = None;
                                *app_player = None;
                                *app_state_play = AppStatePlay::default();
                                return;
                            } else {
                                debug.then(|| println!("[?] Quitting player"));
                                *running = false
                            }
                        });

                        (key_event.code == KeyCode::Tab
                            && key_event.kind == KeyEventKind::Press
                            && key_event.modifiers == KeyModifiers::empty())
                        .then(|| {
                            debug.then(|| println!("[?]Switching tab"));
                            which.toggle()
                        });

                        if let Some(audio_player) = app_player {
                            play_key_input(audio_player, key_event);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
