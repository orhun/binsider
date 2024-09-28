use std::time::Instant;

use ansi_to_tui::IntoText;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::Text,
    widgets::{Widget, WidgetRef},
};

const LOGO: &str = "
\x1b[49m   \x1b[48;2;22;22;22m  \x1b[38;2;22;22;22;49m▄\x1b[49m        \x1b[38;2;22;22;22;49m▄\x1b[48;2;22;22;22m  \x1b[49m   \x1b[m
\x1b[49m   \x1b[48;2;22;22;22m    \x1b[38;2;22;22;22;49m▄▄▄▄▄▄\x1b[48;2;22;22;22m    \x1b[49m   \x1b[m
\x1b[49m   \x1b[48;2;22;22;22m   \x1b[38;2;248;190;117;48;2;22;22;22m▄\x1b[48;2;22;22;22m      \x1b[38;2;248;190;117;48;2;22;22;22m▄\x1b[48;2;22;22;22m   \x1b[49m   \x1b[m
\x1b[49m   \x1b[48;2;22;22;22m  \x1b[48;2;248;190;117m \x1b[48;2;22;22;22m \x1b[48;2;248;190;117m \x1b[48;2;22;22;22m    \x1b[48;2;248;190;117m \x1b[48;2;22;22;22m \x1b[48;2;248;190;117m \x1b[48;2;22;22;22m  \x1b[49m   \x1b[m
\x1b[49;38;2;22;22;22m▀\x1b[48;2;22;22;22m  \x1b[38;2;255;255;255;48;2;22;22;22m▄▄▄▄▄▄▄▄▄▄▄▄▄▄\x1b[48;2;22;22;22m  \x1b[49;38;2;22;22;22m▀\x1b[m
\x1b[49m  \x1b[49;38;2;117;117;117m\x1b[48;2;22;22;22m \x1b[48;2;255;255;255m  \x1b[38;2;255;255;255;48;2;22;22;22m▄\x1b[48;2;22;22;22m \x1b[48;2;255;255;255m  \x1b[48;2;22;22;22m  \x1b[48;2;255;255;255m  \x1b[48;2;22;22;22m \x1b[38;2;255;255;255;48;2;22;22;22m▄\x1b[48;2;255;255;255m  \x1b[48;2;22;22;22m \x1b[49m  \x1b[m
\x1b[49m  \x1b[48;2;22;22;22m \x1b[48;2;255;255;255m   \x1b[48;2;22;22;22m \x1b[48;2;255;255;255m  \x1b[48;2;22;22;22m  \x1b[48;2;255;255;255m  \x1b[48;2;22;22;22m \x1b[48;2;255;255;255m   \x1b[48;2;22;22;22m \x1b[49m  \x1b[m
\x1b[49m  \x1b[49;38;2;22;22;22m▀\x1b[48;2;22;22;22m \x1b[48;2;255;255;255m  \x1b[48;2;22;22;22m \x1b[48;2;255;255;255m  \x1b[48;2;22;22;22m  \x1b[48;2;255;255;255m  \x1b[48;2;22;22;22m \x1b[48;2;255;255;255m  \x1b[48;2;22;22;22m \x1b[49;38;2;22;22;22m▀\x1b[49m  \x1b[m
\x1b[49m   \x1b[48;2;22;22;22m \x1b[48;2;255;255;255m  \x1b[48;2;22;22;22m \x1b[48;2;255;255;255m  \x1b[48;2;22;22;22m  \x1b[48;2;255;255;255m  \x1b[48;2;22;22;22m \x1b[48;2;255;255;255m  \x1b[48;2;22;22;22m \x1b[49m   \x1b[m
\x1b[49m   \x1b[48;2;22;22;22m \x1b[38;2;22;22;22;48;2;104;104;104m▄\x1b[48;2;255;255;255m          \x1b[38;2;22;22;22;48;2;73;73;73m▄\x1b[48;2;22;22;22m \x1b[49m   \x1b[m
\x1b[49m    \x1b[49;38;2;22;22;22m▀▀▀▀▀▀▀▀▀▀▀▀\x1b[49m    \x1b[m
";
const WIDTH: u16 = 20;
const HEIGHT: u16 = 12;

#[derive(Debug)]
pub struct Logo {
    pub init_time: Instant,
    pub is_rendered: bool,
}

impl Default for Logo {
    fn default() -> Self {
        Self {
            init_time: Instant::now(),
            is_rendered: false,
        }
    }
}

impl WidgetRef for Logo {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let text: Text = LOGO.into_text().expect("failed to parse ANSI");
        text.render(area, buf);
        let message = "Loading...";
        buf.set_string(
            WIDTH / 2 - message.len() as u16 / 2 + area.x,
            HEIGHT + area.y,
            message,
            Style::default().fg(Color::Rgb(248, 190, 117)).italic(),
        )
    }
}

impl Logo {
    /// Returns the size of the logo.
    pub fn get_size(&self) -> (u16, u16) {
        (WIDTH, HEIGHT)
    }
}
