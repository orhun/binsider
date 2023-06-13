use crate::tui::state::State;
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

/// Titles of the tabs.
pub const TAB_TITLES: &[&str] = &["Static", "Dynamic", "Hexdump"];

/// Renders the user interface widgets.
pub fn render<B: Backend>(state: &mut State, frame: &mut Frame<'_, B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(frame.size());

    {
        frame.render_widget(Block::default().borders(Borders::ALL), chunks[0]);
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[0]);
        let tabs = Tabs::new(TAB_TITLES.iter().map(|v| Line::from(*v)).collect())
            .select(state.tab_index)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Black),
            );
        frame.render_widget(tabs, chunks[0]);
        frame.render_widget(
            Paragraph::new(format!(
                "{} {}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            ))
            .alignment(Alignment::Right),
            chunks[1],
        )
    }
    let inner = match state.tab_index {
        0 => Block::default().title("Inner 0").borders(Borders::ALL),
        1 => Block::default().title("Inner 1").borders(Borders::ALL),
        2 => Block::default().title("Inner 2").borders(Borders::ALL),
        _ => unreachable!(),
    };
    frame.render_widget(inner, chunks[1]);
}
