use crate::app::App;
use crate::bash;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_key(key: KeyEvent, app: &mut App) -> bool {
    // If we're in adding mode, consume most keys for text input first
    if app.adding {
        match key.code {
            KeyCode::Char(c) => {
                app.input_char(c);
            }
            KeyCode::Backspace => app.backspace(),
            KeyCode::Esc => app.cancel_add(),
            KeyCode::Tab => app.next_field(),
            KeyCode::Enter => {
                if app.input_stage == 0 {
                    app.next_field();
                } else {
                    app.commit_add();
                }
            }
            _ => {}
        }

        return false;
    }

    match key.code {
        KeyCode::Char('q') => return true,
        KeyCode::Down => app.move_down(),
        KeyCode::Up => app.move_up(),

        // Toggle full list view
        KeyCode::Char('a') => app.toggle_show_all(),

        // Reload apps from disk
        KeyCode::Char('r') => app.reload(),

        // Remove selected app
        KeyCode::Char('d') => {
            if !app.apps.is_empty() {
                if let Some((name, _)) = app.apps.get(app.selected).cloned() {
                    bash::remove_app(&name);
                    app.reload();
                }
            }
        }

        // Start adding new app
        KeyCode::Char('n') => {
            if !app.adding {
                app.start_add();
            }
        }

        KeyCode::Enter => {
            if app.adding {
                // If adding and on name field, go to url; otherwise commit
                if app.input_stage == 0 {
                    app.next_field();
                } else {
                    app.commit_add();
                }
            } else if let Some(url) = app.selected_url() {
                bash::launch_app(url);
            }
        }
        _ => {}
    }

    false
}
