use crate::{elf::Info, tui::state::State};
use ansi_to_tui::IntoText;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        Block, Cell, Clear, Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table,
        TableState, Tabs, Wrap,
    },
    Frame,
};
use tui_input::Input;
use tui_popup::Popup;
use unicode_width::UnicodeWidthStr;

/// Titles of the main tabs.
pub const MAIN_TABS: &[&str] = Tab::get_headers();

/// Header for the strings table.
const STRINGS_HEADERS: &[&str] = &["Location", "String"];

/// Maximum number of elements to show in table/list.
const LIST_LIMIT: usize = 100;

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
            Block::bordered()
                .title(vec![
                    env!("CARGO_PKG_NAME").bold(),
                    "-".fg(Color::Rgb(100, 100, 100)),
                    env!("CARGO_PKG_VERSION").into(),
                ])
                .title_alignment(Alignment::Center),
            chunks[0],
        );
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[0]);
        let tabs = Tabs::new(MAIN_TABS.iter().map(|v| Line::from(*v)))
            .select(state.tab as usize)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::White),
            );
        frame.render_widget(tabs, chunks[0]);
        frame.render_widget(
            Paragraph::new(state.analyzer.path.italic()).alignment(Alignment::Right),
            chunks[1],
        )
    }
    match state.tab {
        Tab::StaticAnalysis => {
            render_static_analysis(state, frame, chunks[1]);
        }
        Tab::DynamicAnalysis => {
            render_dynamic_analysis(state, frame, chunks[1]);
        }
        Tab::Strings => {
            render_strings(state, frame, chunks[1]);
        }
        Tab::Hexdump => {
            state.analyzer.heh.render_frame(frame, chunks[1]);
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
                Span::raw(": ").fg(Color::Rgb(100, 100, 100)),
                Span::styled(items[1].to_string(), Style::default().fg(Color::White)),
            ])
        })
        .collect();
    let mut notes = Vec::new();
    for note in state.analyzer.elf.notes.inner.iter() {
        notes.push(Line::from(vec![
            "Notes in ".cyan(),
            note.name.to_string().cyan().italic(),
        ]));
        notes.push(Line::from(
            note.header
                .iter()
                .map(|v| Span::raw(format!("{v} ")))
                .collect::<Vec<Span>>(),
        ));
        notes.push(Line::from(
            note.text
                .iter()
                .map(|v| Span::raw(format!("{v} ")))
                .collect::<Vec<Span>>(),
        ));
    }
    frame.render_widget(Block::bordered(), rect);
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
            Paragraph::new(headers)
                .block(
                    Block::bordered()
                        .title(vec![
                            "|".fg(Color::Rgb(100, 100, 100)),
                            "File Headers".white().bold(),
                            "|".fg(Color::Rgb(100, 100, 100)),
                        ])
                        .border_style(Style::default().fg(Color::Rgb(100, 100, 100))),
                )
                .wrap(Wrap { trim: true }),
            chunks[0],
        );
        frame.render_widget(
            Paragraph::new(notes)
                .block(
                    Block::bordered()
                        .title(vec![
                            "|".fg(Color::Rgb(100, 100, 100)),
                            "Notes".white().bold(),
                            "|".fg(Color::Rgb(100, 100, 100)),
                        ])
                        .border_style(Style::default().fg(Color::Rgb(100, 100, 100))),
                )
                .wrap(Wrap { trim: true }),
            chunks[1],
        );
    }
    {
        let area = chunks[1];
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Percentage(100)].as_ref())
            .split(area);
        let tabs = Tabs::new(MAIN_TABS.iter().map(|v| Line::from(*v)))
            .select(state.tab as usize)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::White),
            );
        frame.render_widget(tabs, chunks[0]);
        let selected_index = state.list.state.selected().unwrap_or_default();
        let items_len = state.list.items.len();
        let page = selected_index / LIST_LIMIT;
        let headers = ELF_INFO_TABS[state.info_index].headers();
        let mut table_state = TableState::default();
        table_state.select(Some(selected_index % LIST_LIMIT));
        let max_row_width = (area.width as usize / headers.len()).saturating_sub(2);
        let items = state
            .list
            .items
            .iter()
            .skip(page * LIST_LIMIT)
            .take(LIST_LIMIT)
            .map(|items| {
                Row::new(items.iter().enumerate().map(|(i, value)| {
                    Cell::from(Line::from(
                        if value.width() > max_row_width && i == items.len() - 1 {
                            vec![
                                value.chars().take(max_row_width).collect::<String>().into(),
                                "…".fg(Color::Rgb(100, 100, 100)),
                            ]
                        } else {
                            vec![value.to_string().into()]
                        },
                    ))
                }))
            });
        frame.render_stateful_widget(
            Table::new(
                items,
                &[Constraint::Percentage(
                    (100 / headers.len()).try_into().unwrap_or_default(),
                )]
                .repeat(headers.len()),
            )
            .header(Row::new(
                headers.to_vec().iter().map(|v| Cell::from((*v).bold())),
            ))
            .block(
                Block::bordered()
                    .border_style(Style::default().fg(Color::Rgb(100, 100, 100)))
                    .title_bottom(
                        if items_len != 0 {
                            Line::from(vec![
                                "|".fg(Color::Rgb(100, 100, 100)),
                                format!("{}/{}", selected_index.saturating_add(1), items_len)
                                    .white()
                                    .bold(),
                                "|".fg(Color::Rgb(100, 100, 100)),
                            ])
                        } else {
                            Line::default()
                        }
                        .right_aligned(),
                    )
                    .title_bottom(get_input_line(state)),
            )
            .highlight_style(Style::default().fg(Color::Green)),
            area,
            &mut table_state,
        );
        render_cursor(state, area, frame);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            area.inner(&Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut ScrollbarState::new(items_len).position(selected_index),
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
        let tabs = Tabs::new(ELF_INFO_TABS.iter().map(|v| Line::from(v.title())))
            .select(state.info_index)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::White),
            );
        frame.render_widget(tabs, chunks[1]);
        render_details(state, rect, frame);
    }
}

/// Renders the strings tab.
pub fn render_strings(state: &mut State, frame: &mut Frame, rect: Rect) {
    let selected_index = state.list.state.selected().unwrap_or_default();
    let items_len = state.list.items.len();
    let page = selected_index / LIST_LIMIT;
    let items = state
        .list
        .items
        .iter()
        .skip(page * LIST_LIMIT)
        .take(LIST_LIMIT);
    let left_padding = items
        .clone()
        .last()
        .cloned()
        .unwrap_or_default()
        .first()
        .map(|v| v.len())
        .unwrap_or_default()
        + 1;
    if items_len == 0 && state.input.value().is_empty() {
        frame.render_widget(Block::bordered(), rect);
        frame.render_widget(
            Paragraph::new("Loading...".italic()).alignment(Alignment::Center),
            rect.inner(&Margin {
                vertical: 1,
                horizontal: 0,
            }),
        );
        return;
    }
    let mut list_state = TableState::default();
    list_state.select(Some(selected_index % LIST_LIMIT));
    let max_row_width = rect.width.saturating_sub(4) as usize;
    frame.render_stateful_widget(
        Table::new(
            items.map(|items| {
                Row::new(vec![Cell::from(Line::from({
                    let index = format!("{:>p$}", items[0], p = left_padding);
                    let value = items[1].to_string();
                    let mut line = vec![index.clone().cyan(), " ".into()];
                    if index.width() + value.width() > max_row_width {
                        line.push(
                            value
                                .chars()
                                .take(max_row_width.saturating_sub(index.width()))
                                .collect::<String>()
                                .into(),
                        );
                        line.push("…".fg(Color::Rgb(100, 100, 100)));
                    } else {
                        line.push(value.into());
                    }
                    line
                }))])
            }),
            &[Constraint::Percentage(100)],
        )
        .header(Row::new(vec![STRINGS_HEADERS.join(" ").bold()]))
        .block(
            Block::bordered()
                .title_top(
                    Line::from(vec![
                        "|".fg(Color::Rgb(100, 100, 100)),
                        format!("Min length: {}", state.analyzer.strings_len)
                            .white()
                            .bold(),
                        "|".fg(Color::Rgb(100, 100, 100)),
                    ])
                    .right_aligned(),
                )
                .title_bottom(
                    if items_len != 0 {
                        Line::from(vec![
                            "|".fg(Color::Rgb(100, 100, 100)),
                            format!("{}/{}", selected_index.saturating_add(1), items_len)
                                .white()
                                .bold(),
                            "|".fg(Color::Rgb(100, 100, 100)),
                        ])
                    } else {
                        Line::default()
                    }
                    .right_aligned(),
                )
                .title_bottom(get_input_line(state)),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD)),
        rect,
        &mut list_state,
    );
    render_cursor(state, rect, frame);
    render_details(state, rect, frame);
    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        rect.inner(&Margin {
            vertical: 1,
            horizontal: 0,
        }),
        &mut ScrollbarState::new(items_len).position(selected_index),
    );
}

/// Renders the cursor.
fn render_cursor(state: &mut State<'_>, area: Rect, frame: &mut Frame<'_>) {
    if state.input_mode {
        let (x, y) = (
            area.x
                + Input::default()
                    .with_value(format!("search: {}", state.input.value()))
                    .visual_cursor() as u16
                + 2,
            area.bottom().saturating_sub(1),
        );
        frame.render_widget(
            Clear,
            Rect {
                x,
                y,
                width: 1,
                height: 1,
            },
        );
        frame.set_cursor(x, y);
    }
}

/// Renders details popup.
fn render_details(state: &mut State<'_>, area: Rect, frame: &mut Frame<'_>) {
    if state.show_details {
        let headers;
        match state.tab {
            Tab::StaticAnalysis => {
                headers = ELF_INFO_TABS[state.info_index].headers();
            }
            Tab::Strings => {
                headers = STRINGS_HEADERS;
            }
            _ => {
                unimplemented!()
            }
        }
        let max_row_width = (area.width - 2) / 2;
        let items = state.list.selected().cloned().unwrap_or_default();
        let lines: Vec<Line> = items
            .iter()
            .enumerate()
            .flat_map(|(i, v)| {
                let mut lines = Vec::new();
                if v.width() as u16 > max_row_width {
                    lines.extend(
                        textwrap::wrap(v, textwrap::Options::new(max_row_width as usize))
                            .into_iter()
                            .enumerate()
                            .map(|(x, v)| {
                                if x == 0 {
                                    Line::from(vec![
                                        Span::styled(
                                            headers[i].to_string(),
                                            Style::default().fg(Color::Cyan),
                                        ),
                                        Span::raw(": ").fg(Color::Rgb(100, 100, 100)),
                                        v.to_string().into(),
                                    ])
                                } else {
                                    Line::from(v.to_string())
                                }
                            }),
                    )
                } else {
                    lines.push(Line::from(vec![
                        Span::styled(headers[i].to_string(), Style::default().fg(Color::Cyan)),
                        Span::raw(": ").fg(Color::Rgb(100, 100, 100)),
                        Span::styled(v, Style::default().fg(Color::White)),
                    ]));
                }
                lines
            })
            .collect();
        let popup = Popup::new(
            vec![
                "|".fg(Color::Rgb(100, 100, 100)),
                "Details".white().bold(),
                "|".fg(Color::Rgb(100, 100, 100)),
            ],
            lines,
        );
        frame.render_widget(popup.to_widget(), area);
    }
}

/// Renders the dynamic analysis tab.
pub fn render_dynamic_analysis(state: &mut State, frame: &mut Frame, rect: Rect) {
    if state.analyzer.tracer.syscalls.is_empty() {
        frame.render_widget(
            Paragraph::new(vec![
                Line::from(vec![
                    "Press ".into(),
                    "Enter".cyan(),
                    " to run the executable.".into(),
                ]),
                Line::from(vec![
                    "(".fg(Color::Rgb(100, 100, 100)),
                    state.analyzer.path.italic(),
                    ")".fg(Color::Rgb(100, 100, 100)),
                ]),
            ])
            .block(Block::bordered())
            .alignment(Alignment::Center),
            rect,
        );
    } else {
        let text = state
            .analyzer
            .tracer
            .syscalls
            .into_text()
            .unwrap_or_else(|_| Text::from("ANSI error occurred"));
        let max_height = text.height().saturating_sub(rect.height as usize) + 2;
        if max_height < state.scroll_index {
            state.scroll_index = max_height;
        }

        frame.render_widget(
            Paragraph::new(text.clone())
                .block(Block::bordered().title(vec![
                    "|".fg(Color::Rgb(100, 100, 100)),
                    "System Calls".white().bold(),
                    "|".fg(Color::Rgb(100, 100, 100)),
                ]))
                .scroll((state.scroll_index as u16, 0)),
            rect,
        );

        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            rect.inner(&Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut ScrollbarState::new(max_height).position(state.scroll_index),
        );

        if state.show_details && !state.analyzer.tracer.summary.is_empty() {
            let summary = state
                .analyzer
                .tracer
                .summary
                .into_text()
                .unwrap_or_else(|_| Text::from("ANSI error occurred"))
                .into_iter()
                .filter(|v| v.width() != 0)
                .collect::<Vec<Line>>();
            let popup = Popup::new(
                vec![
                    "|".fg(Color::Rgb(100, 100, 100)),
                    "Details".white().bold(),
                    "|".fg(Color::Rgb(100, 100, 100)),
                ],
                summary,
            );
            frame.render_widget(popup.to_widget(), rect);
        }
    }
}

/// Returns the input line.
fn get_input_line<'a>(state: &'a State) -> Line<'a> {
    if !state.input.value().is_empty() || state.input_mode {
        Line::from(vec![
            "|".fg(Color::Rgb(100, 100, 100)),
            "search: ".green(),
            state.input.value().white(),
            if state.input_mode { " " } else { "" }.into(),
            "|".fg(Color::Rgb(100, 100, 100)),
        ])
    } else {
        Line::default()
    }
}
