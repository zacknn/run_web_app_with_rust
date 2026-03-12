use crate::app::App;
use crate::bash;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_key(key: KeyEvent, app: &mut App) -> bool {
    match key.code {
        KeyCode::Char('q') => return true,

        KeyCode::Down => app.move_down(),
        KeyCode::Up => app.move_up(),

        KeyCode::Enter => {
            if let Some(url) = app.selected_url() {
                bash::launch_app(url);
            }
        }
        _ => {}
    }

    false
}
