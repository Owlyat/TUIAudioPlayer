use std::time::Duration;

use ratatui::crossterm::event::{KeyCode, KeyEventKind};

use crate::audio::AudioPlayer;

pub fn handle_event(
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
                (key_event.code == KeyCode::Enter && key_event.kind == KeyEventKind::Press).then(
                    || {
                        if audio_player.is_paused() {
                            audio_player.play()
                        } else {
                            audio_player.pause();
                        }
                    },
                );
            }
            ratatui::crossterm::event::Event::Mouse(mouse_event) => {}
            ratatui::crossterm::event::Event::Paste(_) => {}
            ratatui::crossterm::event::Event::Resize(_, _) => {}
        }
    }
    Ok(())
}
