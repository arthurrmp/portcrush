use super::theme;
use crate::app::App;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let sep = "\u{2500}".repeat(area.width as usize);
    let sep_line = Line::from(Span::styled(&sep, Style::default().fg(theme::BORDER)));

    let status_line = if let Some((msg, is_success, _)) = &app.message {
        let color = if *is_success {
            theme::SUCCESS
        } else {
            theme::KILL_RED
        };
        let icon = if *is_success { "\u{2713}" } else { "\u{2717}" };
        Line::from(vec![
            Span::raw("  "),
            Span::styled(
                format!("{} {}", icon, msg),
                Style::default()
                    .fg(color)
                    .add_modifier(Modifier::BOLD),
            ),
        ])
    } else {
        let count = app.filtered.len();
        let total = app.all_ports.len();

        let left = if !app.filter.is_empty() {
            format!(
                "  {} of {} ports (filter: \"{}\")",
                count, total, app.filter
            )
        } else {
            let noun = if count == 1 { "port" } else { "ports" };
            format!("  {} listening {}", count, noun)
        };

        let right = format!("sorted by {} ", app.sort_mode.label());
        let pad = (area.width as usize).saturating_sub(left.len() + right.len());
        let padding = " ".repeat(pad);

        Line::from(vec![
            Span::styled(left, Style::default().fg(theme::TEXT)),
            Span::raw(padding),
            Span::styled(right, Style::default().fg(theme::TEXT_DIM)),
        ])
    };

    let header = Paragraph::new(vec![sep_line.clone(), status_line, sep_line]);
    frame.render_widget(header, area);
}
