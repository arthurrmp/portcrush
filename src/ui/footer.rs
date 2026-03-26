use super::theme;
use crate::app::{App, AppState};
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let line = match &app.state {
        AppState::Filtering => {
            let spans = vec![
                Span::styled(
                    " / ",
                    Style::default()
                        .fg(Color::White)
                        .bg(theme::BORDER)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!(" {}\u{2588}", app.filter),
                    Style::default().fg(theme::ACCENT),
                ),
            ];
            Line::from(spans)
        }
        AppState::ConfirmKill => {
            let spans = vec![
                shortcut("y", "confirm"),
                shortcut("n", "cancel"),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
            Line::from(spans)
        }
        AppState::Normal => {
            let mut s: Vec<Vec<Span>> = vec![
                shortcut("\u{2191}\u{2193}", "navigate"),
                shortcut("enter", "kill"),
                shortcut("r", "refresh"),
                shortcut("s", "sort"),
                shortcut("/", "filter"),
                shortcut("q", "quit"),
            ];
            if !app.filter.is_empty() {
                s.insert(5, shortcut("esc", "clear"));
            }
            let spans: Vec<Span> = s.into_iter().flatten().collect();
            Line::from(spans)
        }
    };

    let footer = Paragraph::new(line).style(Style::default().bg(theme::FOOTER_BG));
    frame.render_widget(footer, area);
}

fn shortcut<'a>(key: &'a str, action: &'a str) -> Vec<Span<'a>> {
    vec![
        Span::styled(
            format!(" {} ", key),
            Style::default()
                .fg(Color::White)
                .bg(theme::BORDER)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(" {}  ", action),
            Style::default().fg(theme::TEXT_DIM),
        ),
    ]
}
