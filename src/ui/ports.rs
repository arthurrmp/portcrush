use super::theme;
use crate::app::App;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};
use ratatui::Frame;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    if app.filtered.is_empty() {
        render_empty(frame, app, area);
        return;
    }

    let mut lines: Vec<Line> = Vec::new();

    // Column header
    lines.push(column_header());
    let sep = "\u{2500}".repeat((area.width as usize).saturating_sub(2));
    lines.push(Line::from(Span::styled(
        format!(" {}", sep),
        Style::default().fg(theme::BORDER),
    )));

    // Scrolling
    let max_visible = (area.height as usize).saturating_sub(2);
    let total = app.filtered.len();
    let offset = if total <= max_visible {
        0
    } else if app.selected < max_visible / 2 {
        0
    } else if app.selected >= total.saturating_sub(max_visible / 2) {
        total.saturating_sub(max_visible)
    } else {
        app.selected.saturating_sub(max_visible / 2)
    };

    let visible = &app.filtered[offset..total.min(offset + max_visible)];

    for (vi, &idx) in visible.iter().enumerate() {
        let entry = &app.all_ports[idx];
        let is_selected = vi + offset == app.selected;
        lines.push(port_row(entry, is_selected, area.width));
    }

    let content = Paragraph::new(lines);
    frame.render_widget(content, area);
}

pub fn render_confirm(frame: &mut Frame, app: &App, area: Rect) {
    let entry = match app.selected_entry() {
        Some(e) => e,
        None => return,
    };

    let w = 56u16.min(area.width.saturating_sub(4));
    let h = 4u16.min(area.height.saturating_sub(2));
    let dialog = theme::centered_box(area, w, h);

    frame.render_widget(Clear, dialog);

    let block = Block::default()
        .title(" Kill process? ")
        .title_style(
            Style::default()
                .fg(theme::KILL_RED)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme::KILL_RED));
    let inner = block.inner(dialog);
    frame.render_widget(block, dialog);

    let info = format!(
        "  {} (PID {}) on :{}",
        entry.process, entry.pid, entry.port
    );
    let lines = vec![
        Line::from(Span::styled(info, Style::default().fg(Color::White))),
        Line::from(vec![
            Span::raw("  "),
            Span::styled(
                " y ",
                Style::default()
                    .fg(Color::Rgb(30, 30, 30))
                    .bg(theme::KILL_RED)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" / ", Style::default().fg(theme::TEXT_DIM)),
            Span::styled(
                " enter ",
                Style::default()
                    .fg(Color::Rgb(30, 30, 30))
                    .bg(theme::KILL_RED)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" confirm  ", Style::default().fg(theme::TEXT_DIM)),
            Span::styled(
                " n ",
                Style::default()
                    .fg(Color::White)
                    .bg(theme::BORDER)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" / ", Style::default().fg(theme::TEXT_DIM)),
            Span::styled(
                " esc ",
                Style::default()
                    .fg(Color::White)
                    .bg(theme::BORDER)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" cancel", Style::default().fg(theme::TEXT_DIM)),
        ]),
    ];

    let content = Paragraph::new(lines);
    frame.render_widget(content, inner);
}

fn render_empty(frame: &mut Frame, app: &App, area: Rect) {
    let (msg, hint) = if !app.filter.is_empty() {
        (
            format!("No ports matching \"{}\"", app.filter),
            "Press esc to clear filter",
        )
    } else {
        (
            "No listening ports found".to_string(),
            "Press r to refresh",
        )
    };

    let cy = area.y + area.height / 2;
    let lines = vec![
        Line::from(Span::styled(
            msg,
            Style::default()
                .fg(theme::TEXT_DIM)
                .add_modifier(Modifier::BOLD),
        )),
        Line::raw(""),
        Line::from(Span::styled(hint, Style::default().fg(theme::TEXT_DIM))),
    ];

    let p = Paragraph::new(lines).alignment(ratatui::layout::Alignment::Center);
    let msg_area = Rect {
        x: area.x,
        y: cy.saturating_sub(1),
        width: area.width,
        height: 3.min(area.height),
    };
    frame.render_widget(p, msg_area);
}

fn column_header<'a>() -> Line<'a> {
    let style = Style::default()
        .fg(theme::TEXT_DIM)
        .add_modifier(Modifier::BOLD);
    Line::from(vec![
        Span::raw("    "),
        Span::styled(format!("{:>5}", "PORT"), style),
        Span::raw("   "),
        Span::styled(format!("{:<3}", "PRT"), style),
        Span::raw("   "),
        Span::styled(format!("{:<14}", "PROCESS"), style),
        Span::raw("   "),
        Span::styled(format!("{:>6}", "PID"), style),
        Span::raw("   "),
        Span::styled("ADDRESS", style),
    ])
}

fn port_row<'a>(
    entry: &crate::scanner::PortEntry,
    selected: bool,
    _width: u16,
) -> Line<'a> {
    let (marker, marker_style) = if selected {
        (
            " \u{25b8} ",
            Style::default()
                .fg(theme::ACCENT)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        ("   ", Style::default())
    };

    let bg = if selected {
        theme::ROW_HL
    } else {
        Color::Reset
    };

    let text_style = if selected {
        Style::default()
            .fg(Color::White)
            .bg(bg)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme::TEXT).bg(bg)
    };

    let dim_style = if selected {
        Style::default().fg(theme::TEXT).bg(bg)
    } else {
        Style::default().fg(theme::TEXT_DIM).bg(bg)
    };

    let process_style = if selected {
        Style::default()
            .fg(theme::ACCENT)
            .bg(bg)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme::ACCENT).bg(bg)
    };

    let process = if entry.process.len() > 14 {
        format!("{:.14}", entry.process)
    } else {
        format!("{:<14}", entry.process)
    };

    Line::from(vec![
        Span::styled(marker.to_string(), marker_style.bg(bg)),
        Span::styled(format!("{:>5}", entry.port), text_style),
        Span::styled("   ", Style::default().bg(bg)),
        Span::styled(format!("{:<3}", entry.proto), dim_style),
        Span::styled("   ", Style::default().bg(bg)),
        Span::styled(process, process_style),
        Span::styled("   ", Style::default().bg(bg)),
        Span::styled(format!("{:>6}", entry.pid), dim_style),
        Span::styled("   ", Style::default().bg(bg)),
        Span::styled(entry.address.clone(), dim_style),
    ])
}
