use crate::app::App;
use ratatui::Frame;
use ratatui::{
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

pub fn draw(frame: &mut Frame<'_>, app: &mut App) {
    let area = frame.size();

    let items: Vec<ListItem> = if app.show_all {
        app.apps
            .iter()
            .map(|(name, url)| ListItem::new(format!("{} | {}", name, url)))
            .collect()
    } else {
        app.apps.iter().map(|(name, _)| ListItem::new(name.clone())).collect()
    };

    let mut state = ListState::default();
    if !app.apps.is_empty() {
        state.select(Some(app.selected));
    } else {
        state.select(None);
    }

    let title = if app.show_all { " WebAppMan - All Apps " } else { " WebAppMan " };

    let list = List::new(items)
        .block(Block::default().title(title).borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(list, area, &mut state);

    // If in adding mode, render input overlay in center
    if app.adding {
        let popup_area = ratatui::layout::Rect {
            x: area.x + area.width / 8,
            y: area.y + area.height / 3,
            width: area.width * 6 / 8,
            height: area.height / 3,
        };

            let name_suffix = if app.input_stage == 0 { "_" } else { "" };
            let url_suffix = if app.input_stage == 1 { "_" } else { "" };

            let text = format!("Name: {}{}\nURL:  {}{}", app.input_name, name_suffix, app.input_url, url_suffix);

            let paragraph = Paragraph::new(text)
                .block(Block::default().title(" Add App (Enter to next/commit, Esc to cancel) ").borders(Borders::ALL));

        frame.render_widget(paragraph, popup_area);
    }
}
