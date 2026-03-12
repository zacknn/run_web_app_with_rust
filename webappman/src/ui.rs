use crate::app::App;
use ratatui::{
    Frame,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = frame.size();

    let items: Vec<ListItem> = app
        .apps
        .iter()
        .map(|(name, _)| ListItem::new(name.clone()))
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.selected));

    let list = List::new(items)
        .block(Block::default().title(" WebAppMan ").borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(list, area, &mut state);
}
