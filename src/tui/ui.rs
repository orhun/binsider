use crate::{elf::Info, tui::state::State};
use ansi_to_tui::IntoText;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Position, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, Cell, Clear, Paragraph, Row, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Table, TableState, Tabs, Wrap,
    },
    Frame,
};
use tui_big_text::{BigTextBuilder, PixelSize};
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
    /// General information.
    General = 0,
    /// Static analysis.
    StaticAnalysis = 1,
    /// Dynamic analysis.
    DynamicAnalysis = 2,
    /// String.
    Strings = 3,
    /// Hexdump.
    Hexdump = 4,
}

impl Default for Tab {
    fn default() -> Self {
        Self::General
    }
}

impl Tab {
    /// Returns the available tabs.
    const fn get_headers() -> &'static [&'static str] {
        &["General", "Static", "Dynamic", "Strings", "Hexdump"]
    }
}

impl From<usize> for Tab {
    fn from(v: usize) -> Self {
        match v {
            0 => Self::General,
            1 => Self::StaticAnalysis,
            2 => Self::DynamicAnalysis,
            3 => Self::Strings,
            4 => Self::Hexdump,
            _ => Self::default(),
        }
    }
}

/// Renders the user interface widgets.
pub fn render(state: &mut State, frame: &mut Frame) {
    let chunks = Layout::new(
        Direction::Vertical,
        [Constraint::Length(3), Constraint::Min(0)],
    )
    .direction(Direction::Vertical)
    .margin(1)
    .split(frame.area());

    {
        frame.render_widget(
            Block::bordered()
                .title(vec![
                    "|".fg(Color::Rgb(100, 100, 100)),
                    env!("CARGO_PKG_NAME").bold(),
                    "-".fg(Color::Rgb(100, 100, 100)),
                    env!("CARGO_PKG_VERSION").into(),
                    "|".fg(Color::Rgb(100, 100, 100)),
                ])
                .title_alignment(Alignment::Center),
            chunks[0],
        );
        let chunks = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
        .margin(1)
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
        let mut files = Vec::new();
        for (i, file) in state.analyzer.files.iter().enumerate() {
            if i != state.analyzer.files.len() - 1 {
                files.push(
                    file.file_name()
                        .map(|v| v.to_string_lossy().to_string())
                        .unwrap_or_else(|| "?".to_string())
                        .italic(),
                );
                files.push("→ ".fg(Color::Rgb(100, 100, 100)));
            } else {
                files.push(file.to_string_lossy().to_string().italic());
            }
        }
        files.push(" ".into());
        frame.render_widget(
            Paragraph::new(Line::from(files)).alignment(Alignment::Right),
            chunks[1],
        )
    }
    match state.tab {
        Tab::General => {
            render_general_info(state, frame, chunks[1]);
        }
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
            {
                let chunks = Layout::vertical([Constraint::Percentage(100), Constraint::Min(1)])
                    .split(chunks[1]);
                state.analyzer.heh.render_frame(frame, chunks[0]);
            }
            frame.render_widget(Block::new().borders(Borders::BOTTOM), chunks[1])
        }
    }
    render_key_bindings(state, frame, chunks[1]);
}

/// Renders the key bindings.
pub fn render_key_bindings(state: &mut State, frame: &mut Frame, rect: Rect) {
    let chunks = Layout::vertical([Constraint::Percentage(100), Constraint::Min(1)]).split(rect);
    let key_bindings = state.get_key_bindings();
    let line = Line::from(
        key_bindings
            .iter()
            .enumerate()
            .flat_map(|(i, (keys, desc))| {
                vec![
                    "[".fg(Color::Rgb(100, 100, 100)),
                    keys.yellow(),
                    "→ ".fg(Color::Rgb(100, 100, 100)),
                    Span::from(*desc),
                    "]".fg(Color::Rgb(100, 100, 100)),
                    if i != key_bindings.len() - 1 { " " } else { "" }.into(),
                ]
            })
            .collect::<Vec<Span>>(),
    );
    if line.width() as u16 > chunks[1].width.saturating_sub(25) {
        if get_input_line(state).width() != 0
            && (state.tab != Tab::StaticAnalysis || state.tab != Tab::Hexdump)
        {
            return;
        }
    }
    frame.render_widget(Paragraph::new(line.alignment(Alignment::Center)), chunks[1]);
}

/// Renders the general info tab.
pub fn render_general_info(state: &mut State, frame: &mut Frame, rect: Rect) {
    frame.render_widget(Block::bordered(), rect);
    let area = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage(5),
            Constraint::Length(7),
            Constraint::Percentage(100),
        ],
    )
    .margin(1)
    .split(rect);

    let banner = BigTextBuilder::default()
        .pixel_size(PixelSize::Sextant)
        .lines([format!("{}.", env!("CARGO_PKG_NAME")).into()])
        .build();
    let banner_width = 34;
    let banner_area = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Length((area[1].width.checked_sub(banner_width)).unwrap_or_default() / 2),
            Constraint::Min(banner_width),
            Constraint::Length((area[1].width.checked_sub(banner_width)).unwrap_or_default() / 2),
        ],
    )
    .split(area[1]);
    frame.render_widget(banner, banner_area[1]);
    frame.render_widget(
        Paragraph::new(Text::from(vec![
            Line::default(),
            Line::default(),
            Line::default(),
            Line::from(vec![
                "Analyze ELF binaries ".white(),
                "like a boss.".yellow().italic(),
            ]),
            Line::from(
                ratatui::symbols::line::HORIZONTAL
                    .repeat(33)
                    .fg(Color::Rgb(100, 100, 100)),
            ),
            Line::from(env!("CARGO_PKG_REPOSITORY").italic()),
            Line::from(vec![
                "[".fg(Color::Rgb(100, 100, 100)),
                "with ".into(),
                "♥".cyan(),
                " by ".into(),
                "@orhun".cyan(),
                "]".fg(Color::Rgb(100, 100, 100)),
            ]),
        ]))
        .centered(),
        banner_area[1],
    );

    let lines = vec![
        Line::from(vec![
            "Size".cyan(),
            Span::raw(": ").fg(Color::Rgb(100, 100, 100)),
            state.analyzer.file.size.to_string().white(),
        ]),
        Line::from(vec![
            " ".into(),
            "Blocks".cyan(),
            Span::raw(": ").fg(Color::Rgb(100, 100, 100)),
            state.analyzer.file.blocks.to_string().white(),
            " ".into(),
        ]),
        Line::from(vec![
            "Block Size".cyan(),
            Span::raw(": ").fg(Color::Rgb(100, 100, 100)),
            state.analyzer.file.block_size.to_string().white(),
        ]),
        Line::from(vec![
            "Device".cyan(),
            Span::raw(": ").fg(Color::Rgb(100, 100, 100)),
            state.analyzer.file.links.to_string().white(),
        ]),
        Line::from(vec![
            "Inode".cyan(),
            Span::raw(": ").fg(Color::Rgb(100, 100, 100)),
            state.analyzer.file.inode.to_string().white(),
        ]),
        Line::from(vec![
            "Links".cyan(),
            Span::raw(": ").fg(Color::Rgb(100, 100, 100)),
            state.analyzer.file.links.to_string().white(),
        ]),
        Line::from(vec![
            "Access".cyan(),
            Span::raw(": ").fg(Color::Rgb(100, 100, 100)),
            state.analyzer.file.access.mode.to_string().white(),
        ]),
        Line::from(vec![
            "Uid".cyan(),
            Span::raw(": ").fg(Color::Rgb(100, 100, 100)),
            state.analyzer.file.access.uid.to_string().white(),
        ]),
        Line::from(vec![
            "Gid".cyan(),
            Span::raw(": ").fg(Color::Rgb(100, 100, 100)),
            state.analyzer.file.access.gid.to_string().white(),
        ]),
        Line::from(vec![
            "Access".cyan(),
            Span::raw(": ").fg(Color::Rgb(100, 100, 100)),
            state.analyzer.file.date.access.to_string().white(),
        ]),
        Line::from(vec![
            "Modify".cyan(),
            Span::raw(": ").fg(Color::Rgb(100, 100, 100)),
            state.analyzer.file.date.modify.to_string().white(),
        ]),
        Line::from(vec![
            "Change".cyan(),
            Span::raw(": ").fg(Color::Rgb(100, 100, 100)),
            state.analyzer.file.date.change.to_string().white(),
        ]),
        Line::from(vec![
            "Birth".cyan(),
            Span::raw(":  ").fg(Color::Rgb(100, 100, 100)),
            state.analyzer.file.date.birth.to_string().white(),
        ]),
    ];

    let info_width = lines.iter().map(|v| v.width()).max().unwrap_or_default() as u16 + 2;
    let rect = area[2].inner(Margin {
        horizontal: 0,
        vertical: 1,
    });
    let area = Layout::new(
        Direction::Vertical,
        if state.list.items.is_empty() {
            vec![Constraint::Max(lines.len() as u16 + 2)]
        } else if (lines.len() as u16).saturating_sub(2) < rect.height / 2 {
            vec![
                Constraint::Min(lines.len() as u16 + 2),
                Constraint::Percentage(100),
            ]
        } else {
            vec![Constraint::Percentage(50), Constraint::Percentage(50)]
        },
    )
    .split(rect);

    let info_area = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Length((area[0].width.checked_sub(info_width)).unwrap_or_default() / 2),
            Constraint::Min(info_width),
            Constraint::Length((area[0].width.checked_sub(info_width)).unwrap_or_default() / 2),
        ],
    )
    .split(area[0])[1];

    let max_height = lines.len().saturating_sub(info_area.height as usize) + 2;
    if max_height < state.general_scroll_index {
        state.general_scroll_index = max_height;
    }

    frame.render_widget(
        Paragraph::new(lines)
            .block(
                Block::bordered()
                    .title(Line::from(vec![
                        "|".fg(Color::Rgb(100, 100, 100)),
                        "File".cyan(),
                        Span::raw(": ").fg(Color::Rgb(100, 100, 100)),
                        state.analyzer.file.name.to_string().white().bold(),
                        "|".fg(Color::Rgb(100, 100, 100)),
                    ]))
                    .title_alignment(Alignment::Center)
                    .border_style(Style::default().fg(Color::Rgb(100, 100, 100))),
            )
            .scroll((state.general_scroll_index as u16, 0))
            .wrap(Wrap { trim: true }),
        info_area,
    );

    if state.list.items.is_empty() {
        return;
    }

    let max_row_width = state
        .list
        .items
        .iter()
        .map(|v| v.join(" ").len())
        .max()
        .unwrap_or_default() as u16
        + 5;

    let table_area = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Length((area[1].width.checked_sub(max_row_width)).unwrap_or_default() / 2),
            Constraint::Min(max_row_width),
            Constraint::Length((area[1].width.checked_sub(max_row_width)).unwrap_or_default() / 2),
        ],
    )
    .split(area[1]);

    let table_area = Layout::new(
        Direction::Vertical,
        [
            Constraint::Min(state.list.items.len() as u16 + 3),
            Constraint::Percentage(100),
        ],
    )
    .split(table_area[1])[0];

    frame.render_stateful_widget(
        Table::new(
            state
                .list
                .items
                .clone()
                .into_iter()
                .map(Row::new)
                .collect::<Vec<Row>>(),
            &[
                Constraint::Min(
                    state
                        .list
                        .items
                        .iter()
                        .map(|v| v[0].len())
                        .max()
                        .unwrap_or_default() as u16
                        + 1,
                ),
                Constraint::Percentage(100),
            ],
        )
        .header(Row::new(vec!["Library".bold(), "Path".bold()]))
        .block(
            Block::bordered()
                .title(vec![
                    "|".fg(Color::Rgb(100, 100, 100)),
                    "Dependencies".white().bold(),
                    "|".fg(Color::Rgb(100, 100, 100)),
                ])
                .title_alignment(Alignment::Center)
                .border_style(Style::default().fg(Color::Rgb(100, 100, 100))),
        )
        .highlight_style(Style::default().fg(Color::Green)),
        table_area,
        &mut state.list.state,
    );
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
    let chunks = Layout::new(
        Direction::Vertical,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    )
    .margin(1)
    .split(rect);
    {
        let chunks = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
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
        let chunks = Layout::new(
            Direction::Vertical,
            [Constraint::Length(1), Constraint::Percentage(100)],
        )
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
                            let mut spans = highlight_search_result(
                                value.chars().take(max_row_width).collect::<String>().into(),
                                &state.input,
                            );
                            spans.push("…".fg(Color::Rgb(100, 100, 100)));
                            spans
                        } else {
                            highlight_search_result(value.to_string().into(), &state.input)
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
            area.inner(Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut ScrollbarState::new(items_len).position(selected_index),
        );

        let chunks = Layout::new(
            Direction::Horizontal,
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
            ],
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
    if !state.strings_loaded {
        frame.render_widget(Block::bordered(), rect);
        frame.render_widget(
            Paragraph::new("Loading...".italic()).alignment(Alignment::Center),
            rect.inner(Margin {
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
                Row::new(vec![Cell::from({
                    let index = format!("{:>p$}", items[0], p = left_padding);
                    let value = items[1].to_string();
                    let mut spans = vec![index.clone().cyan(), " ".into()];
                    if index.width() + value.width() > max_row_width {
                        spans.extend(highlight_search_result(
                            value
                                .chars()
                                .take(max_row_width.saturating_sub(index.width()))
                                .collect::<String>()
                                .into(),
                            &state.input,
                        ));
                        spans.push("…".fg(Color::Rgb(100, 100, 100)));
                    } else {
                        spans.extend(highlight_search_result(value.into(), &state.input))
                    }
                    Line::from(spans)
                })])
            }),
            &[Constraint::Percentage(100)],
        )
        .header(Row::new(vec![
            format!(" {}", STRINGS_HEADERS.join(" ")).bold()
        ]))
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
        .highlight_style(Style::default().fg(Color::Green).bold()),
        rect,
        &mut list_state,
    );
    render_cursor(state, rect, frame);
    render_details(state, rect, frame);
    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        rect.inner(Margin {
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
        frame.set_cursor_position(Position::new(x, y));
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
        let popup = Popup::new(Text::from(lines)).title(Line::from(vec![
            "|".fg(Color::Rgb(100, 100, 100)),
            "Details".white().bold(),
            "|".fg(Color::Rgb(100, 100, 100)),
        ]));
        frame.render_widget(&popup, area);
    }
}

/// Renders the dynamic analysis tab.
pub fn render_dynamic_analysis(state: &mut State, frame: &mut Frame, rect: Rect) {
    if !state.system_calls_loaded {
        frame.render_widget(
            Paragraph::new(vec![Line::from(vec![
                "Press ".into(),
                "Enter".yellow(),
                " to run the executable.".into(),
            ])])
            .block(Block::bordered())
            .alignment(Alignment::Center),
            rect,
        );
    } else {
        let max_height = state
            .analyzer
            .system_calls
            .len()
            .saturating_sub(rect.height as usize)
            + 2;
        if max_height < state.dynamic_scroll_index {
            state.dynamic_scroll_index = max_height;
        }

        frame.render_widget(
            Paragraph::new(
                state
                    .analyzer
                    .system_calls
                    .clone()
                    .into_iter()
                    .map(|line| highlight_search_result(line, &state.input).into())
                    .collect::<Vec<Line>>(),
            )
            .block(
                Block::bordered()
                    .title(vec![
                        "|".fg(Color::Rgb(100, 100, 100)),
                        "System Calls".white().bold(),
                        "|".fg(Color::Rgb(100, 100, 100)),
                    ])
                    .title_bottom(
                        Line::from(vec![
                            "|".fg(Color::Rgb(100, 100, 100)),
                            "Total: ".into(),
                            state.analyzer.system_calls.len().to_string().white().bold(),
                            "|".fg(Color::Rgb(100, 100, 100)),
                        ])
                        .right_aligned(),
                    )
                    .title_bottom(get_input_line(state)),
            )
            .scroll((state.dynamic_scroll_index as u16, 0)),
            rect,
        );

        render_cursor(state, rect, frame);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            rect.inner(Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut ScrollbarState::new(max_height).position(state.dynamic_scroll_index),
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
            let popup = Popup::new(Text::from(summary)).title(Line::from(vec![
                "|".fg(Color::Rgb(100, 100, 100)),
                "Details".white().bold(),
                "|".fg(Color::Rgb(100, 100, 100)),
            ]));
            frame.render_widget(&popup, rect);
        }
    }
}

/// Returns the input line.
fn get_input_line<'a>(state: &'a State) -> Line<'a> {
    if !state.input.value().is_empty() || state.input_mode {
        Line::from(vec![
            "|".fg(Color::Rgb(100, 100, 100)),
            "search: ".yellow(),
            state.input.value().white(),
            if state.input_mode { " " } else { "" }.into(),
            "|".fg(Color::Rgb(100, 100, 100)),
        ])
    } else {
        Line::default()
    }
}

/// Returns the line with the search result highlighted.
fn highlight_search_result<'a>(line: Line<'a>, input: &'a Input) -> Vec<Span<'a>> {
    let line_str = line.to_string();
    if line_str.contains(input.value()) && !input.value().is_empty() {
        let splits = line_str.split(input.value());
        let chunks = splits.into_iter().map(|c| Span::from(c.to_owned()));
        let pattern = Span::styled(
            input.value(),
            Style::new().bg(Color::Yellow).fg(Color::Black),
        );
        itertools::intersperse(chunks, pattern).collect::<Vec<Span>>()
    } else {
        line.spans.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_highlight_search_string() {
        let line: Line = "onetwothree".into();
        let query = Input::new("two".into());
        let highlighted = highlight_search_result(line, &query);
        assert_eq!(
            vec![
                Span::raw("one"),
                Span::styled("two", Style::new().bg(Color::Yellow).fg(Color::Black)),
                Span::raw("three")
            ],
            highlighted
        );
    }

    // This test is not passing.
    //
    // See this Discord message for more info:
    // <https://discord.com/channels/1070692720437383208/1072907135664529508/1275922734291095734>
    #[test]
    #[ignore]
    fn test_highlight_search_line() {
        let line: Line = vec![
            Span::raw("one"),
            Span::styled("two", Style::new().bg(Color::Blue).fg(Color::Black)),
            Span::raw("three"),
        ]
        .into();
        let query = Input::new("one".into());
        let highlighted = highlight_search_result(line, &query);
        assert_eq!(
            vec![
                Span::styled("one", Style::new().bg(Color::Yellow).fg(Color::Black)),
                Span::styled("two", Style::new().bg(Color::Blue).fg(Color::Black)),
                Span::raw("three")
            ],
            highlighted
        );
    }
}
