use crate::{elf::Info, tui::state::State};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, TableState, Tabs},
    Frame,
};
use unicode_width::UnicodeWidthStr;

/// Titles of the main tabs.
pub const MAIN_TABS: &[&str] = Tab::get_headers();

/// Maximum number of elements to show in table.
const TABLE_LIMIT: usize = 100;

/// Titles of the ELF info tabs.
pub const ELF_INFO_TABS: &[Info] = &[
    Info::ProgramHeaders,
    Info::SectionHeaders,
    Info::Symbols,
    Info::DynamicSymbols,
    Info::Dynamics,
    Info::Relocations,
];

/// Application tab.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Tab {
    /// Static analysis.
    StaticAnalysis = 0,
    /// Dynamic analysis.
    DynamicAnalysis = 1,
    /// String.
    Strings = 2,
    /// Hexdump.
    Hexdump = 3,
}

impl Default for Tab {
    fn default() -> Self {
        Self::StaticAnalysis
    }
}

impl Tab {
    /// Returns the available tabs.
    const fn get_headers() -> &'static [&'static str] {
        &["Static", "Dynamic", "Strings", "Hexdump"]
    }
}

impl From<usize> for Tab {
    fn from(v: usize) -> Self {
        match v {
            0 => Self::StaticAnalysis,
            1 => Self::DynamicAnalysis,
            2 => Self::Strings,
            3 => Self::Hexdump,
            _ => Self::default(),
        }
    }
}

/// Renders the user interface widgets.
pub fn render(state: &mut State, frame: &mut Frame) {
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
        let tabs = Tabs::new(MAIN_TABS.iter().map(|v| Line::from(*v)).collect())
            .select(state.tab as usize)
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
    match state.tab {
        Tab::StaticAnalysis => {
            render_static_analysis(state, frame, chunks[1]);
        }
        Tab::DynamicAnalysis => {
            let block = Block::default().borders(Borders::ALL);
            frame.render_widget(block, chunks[1]);
        }
        Tab::Strings => {
            render_strings(state, frame, chunks[1]);
        }
        Tab::Hexdump => {
            let block = Block::default().borders(Borders::ALL);
            frame.render_widget(block, chunks[1]);
        }
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
pub fn render_static_analysis(state: &mut State, frame: &mut Frame, rect: Rect) {
    let headers: Vec<Line> = state
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
    let mut lines = Vec::new();
    for note in state.analyzer.elf.notes.inner.iter() {
        lines.push(Line::from(vec![Span::styled(
            note.name.to_string(),
            Style::default().fg(Color::Cyan),
        )]));
        lines.push(Line::from(
            note.header
                .iter()
                .map(|v| Span::raw(format!("{v} ")))
                .collect::<Vec<Span>>(),
        ));
        lines.push(Line::from(
            note.text
                .iter()
                .map(|v| Span::raw(format!("{v} ")))
                .collect::<Vec<Span>>(),
        ));
    }
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
            Paragraph::new(headers).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Black)),
            ),
            chunks[0],
        );
        frame.render_widget(
            Paragraph::new(lines).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Black)),
            ),
            chunks[1],
        );
    }
    {
        let area = chunks[1];
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Percentage(100)].as_ref())
            .split(area);
        let tabs = Tabs::new(MAIN_TABS.iter().map(|v| Line::from(*v)).collect())
            .select(state.tab as usize)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Black),
            );
        frame.render_widget(tabs, chunks[0]);
        let selected_index = state.list.state.selected().unwrap_or_default();
        let items_len = state.list.items.len();
        let page = selected_index / TABLE_LIMIT;
        let headers = ELF_INFO_TABS[state.info_index].headers();
        let mut table_state = TableState::default();
        table_state.select(Some(selected_index % TABLE_LIMIT));
        frame.render_stateful_widget(
            Table::new(
                state
                    .list
                    .items
                    .iter()
                    .skip(page * TABLE_LIMIT)
                    .take(TABLE_LIMIT)
                    .map(|items| Row::new(items.iter().map(|v| Cell::from(Span::raw(v))))),
            )
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().fg(Color::Green))
            .header(Row::new(headers.to_vec()))
            .widths(
                &[Constraint::Percentage(
                    (100 / headers.len()).try_into().unwrap_or_default(),
                )]
                .repeat(headers.len()),
            ),
            area,
            &mut table_state,
        );
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(
                        ELF_INFO_TABS
                            .iter()
                            .map(|v| v.title().len() as u16)
                            .sum::<u16>()
                            + ((ELF_INFO_TABS.len() as u16 - 1) * 3)
                            + 2,
                    ),
                    Constraint::Percentage(100),
                ]
                .as_ref(),
            )
            .split(chunks[0]);
        frame.render_widget(Clear, chunks[1]);
        let tabs = Tabs::new(
            ELF_INFO_TABS
                .iter()
                .map(|v| Line::from(v.title()))
                .collect(),
        )
        .select(state.info_index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );
        frame.render_widget(tabs, chunks[1]);
        render_item_index(
            frame,
            rect,
            format!("{}/{}", selected_index.saturating_add(1), items_len),
        );
    }
}

/// Renders the strings tab.
pub fn render_strings(state: &mut State, frame: &mut Frame, rect: Rect) {
    let left_padding = state
        .list
        .items
        .last()
        .cloned()
        .unwrap_or_default()
        .get(1)
        .map(|v| v.len())
        .unwrap_or_default();
    frame.render_stateful_widget(
        Table::new(state.list.items.iter().map(|items| {
            Row::new(vec![Cell::from(Span::raw(format!(
                "{:>p$} {}",
                items[1],
                items[0],
                p = left_padding
            )))])
        }))
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Green))
        .widths(&[Constraint::Percentage(100)]),
        rect,
        &mut state.list.state,
    );
    render_item_index(
        frame,
        rect,
        format!(
            "{}/{}",
            state.list.state.selected().map(|v| v + 1).unwrap_or(0),
            state.list.items.len()
        ),
    );
    let min_length_text = format!("Minimum length: {}", state.analyzer.strings_len);
    let selection_text_width = u16::try_from(min_length_text.width()).unwrap_or_default() + 2;
    if let Some(horizontal_area_width) = rect.width.checked_sub(selection_text_width + 2) {
        let vertical_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Min(1),
                    Constraint::Min(3),
                    Constraint::Min(rect.height),
                ]
                .as_ref(),
            )
            .split(rect);
        let horizontal_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Min(horizontal_area_width),
                    Constraint::Min(selection_text_width),
                    Constraint::Min(1),
                ]
                .as_ref(),
            )
            .split(vertical_area[1]);
        frame.render_widget(Clear, horizontal_area[1]);
        frame.render_widget(
            Paragraph::new(min_length_text).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Black)),
            ),
            horizontal_area[1],
        );
    }
}

/// Renders the text for displaying the selected index.
fn render_item_index(frame: &mut Frame, rect: Rect, selection_text: String) {
    let selection_text_width = u16::try_from(selection_text.width()).unwrap_or_default();
    if let Some(horizontal_area_width) = rect.width.checked_sub(selection_text_width + 3) {
        let vertical_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Min(rect.height.checked_sub(3).unwrap_or(rect.height)),
                    Constraint::Min(1),
                    Constraint::Min(1),
                ]
                .as_ref(),
            )
            .split(rect);
        let horizontal_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Min(horizontal_area_width),
                    Constraint::Min(selection_text_width),
                    Constraint::Min(1),
                    Constraint::Min(1),
                ]
                .as_ref(),
            )
            .split(vertical_area[1]);
        frame.render_widget(Clear, horizontal_area[1]);
        frame.render_widget(Paragraph::new(selection_text), horizontal_area[1]);
        frame.render_widget(Clear, horizontal_area[2]);
        frame.render_widget(Paragraph::new(Text::default()), horizontal_area[2]);
    }
}
