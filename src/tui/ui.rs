use crate::tui::state::State;
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
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
                "{} {} ",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            ))
            .alignment(Alignment::Right),
            chunks[1],
        )
    }
    match state.tab_index {
        0 => {
            render_static_analysis(state, frame, chunks[1]);
        }
        1 => {
            let block = Block::default().borders(Borders::ALL);
            frame.render_widget(block, chunks[1]);
        }
        2 => {
            let block = Block::default().borders(Borders::ALL);
            frame.render_widget(block, chunks[1]);
        }
        _ => unreachable!(),
    }
}

/// Renders the static analysis tab.
pub fn render_static_analysis<B: Backend>(state: &mut State, frame: &mut Frame<'_, B>, rect: Rect) {
    let header: Vec<Line> = state
        .analyzer
        .get_headers()
        .iter()
        .map(|header| {
            Line::from(vec![
                Span::styled(header.name.to_string(), Style::default().fg(Color::Cyan)),
                Span::raw(": "),
                Span::styled(header.value.to_string(), Style::default().fg(Color::White)),
            ])
        })
        .collect();
    frame.render_widget(
        Paragraph::new(header).block(Block::default().borders(Borders::ALL)),
        rect,
    );
}
