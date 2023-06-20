use crate::{elf::Info, tui::state::State};
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Tabs},
    Frame,
};

/// Titles of the tabs.
pub const TAB_TITLES: &[&str] = &["Static", "Dynamic", "Strings", "Hexdump"];

/// Renders the user interface widgets.
pub fn render<B: Backend>(state: &mut State, frame: &mut Frame<'_, B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(frame.size());

    {
        frame.render_widget(
            Block::default()
                .title(format!(
                    "{} {}",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION")
                ))
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL),
            chunks[0],
        );
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
            Paragraph::new(state.analyzer.path).alignment(Alignment::Right),
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
        3 => {
            let block = Block::default().borders(Borders::ALL);
            frame.render_widget(block, chunks[1]);
        }
        _ => unreachable!(),
    }
}

/// Renders the static analysis tab.
///
/// This tab consists of:
/// - file header
/// - program headers
/// - section headers
/// - symbols
/// - dynamic symbols
/// - dynamics
/// - relocations
/// - notes
pub fn render_static_analysis<B: Backend>(state: &mut State, frame: &mut Frame<'_, B>, rect: Rect) {
    let header: Vec<Line> = state
        .analyzer
        .elf
        .info(&Info::FileHeaders)
        .items()
        .iter()
        .map(|items| {
            Line::from(vec![
                Span::styled(items[0].to_string(), Style::default().fg(Color::Cyan)),
                Span::raw(": "),
                Span::styled(items[1].to_string(), Style::default().fg(Color::White)),
            ])
        })
        .collect();
    frame.render_widget(Block::default().borders(Borders::ALL), rect);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(rect);
    {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[0]);
        frame.render_widget(
            Paragraph::new(header).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Black)),
            ),
            chunks[0],
        );
        frame.render_widget(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Black)),
            chunks[1],
        );
    }
    {
        let property = state.analyzer.elf.info(&state.selected_info);
        let headers = property.headers().unwrap_or_default();
        let title = property.title().unwrap_or_default();
        frame.render_stateful_widget(
            Table::new(
                state
                    .list
                    .items
                    .iter()
                    .map(|items| Row::new(items.iter().map(|v| Cell::from(Span::raw(v))))),
            )
            .block(
                Block::default()
                    .title(title)
                    .title_alignment(Alignment::Left)
                    .borders(Borders::all()),
            )
            .highlight_style(Style::default().fg(Color::Green))
            .header(Row::new(headers.to_vec()))
            .widths(
                &[Constraint::Percentage(
                    (100 / headers.len()).try_into().unwrap_or_default(),
                )]
                .repeat(headers.len()),
            ),
            chunks[1],
            &mut state.list.state,
        );
    }
}
