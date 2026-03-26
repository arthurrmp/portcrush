pub mod footer;
pub mod header;
pub mod ports;
pub mod theme;

use crate::app::{App, AppState};
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders};
use ratatui::Frame;

pub fn draw(frame: &mut Frame, app: &App) {
    let full = theme::centered_box(frame.area(), theme::MAX_WIDTH, theme::MAX_HEIGHT);

    let outer = Block::default()
        .title(" portcrush ")
        .title_style(
            Style::default()
                .fg(Color::Rgb(30, 30, 30))
                .bg(theme::ACCENT)
                .add_modifier(Modifier::BOLD),
        )
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme::BORDER));
    let inner = outer.inner(full);
    frame.render_widget(outer, full);

    let [header_area, main_area, footer_area] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .areas(inner);

    header::render(frame, app, header_area);
    ports::render(frame, app, main_area);
    footer::render(frame, app, footer_area);

    if app.state == AppState::ConfirmKill {
        ports::render_confirm(frame, app, main_area);
    }
}
