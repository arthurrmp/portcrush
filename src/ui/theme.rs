use ratatui::layout::Rect;
use ratatui::style::Color;

pub const TEXT: Color = Color::Rgb(220, 220, 220);
pub const TEXT_DIM: Color = Color::Rgb(120, 120, 130);
pub const BORDER: Color = Color::Rgb(60, 60, 70);

pub const ACCENT: Color = Color::Rgb(251, 146, 60);
pub const KILL_RED: Color = Color::Rgb(248, 113, 113);
pub const SUCCESS: Color = Color::Rgb(74, 222, 128);

pub const FOOTER_BG: Color = Color::Rgb(25, 25, 35);
pub const ROW_HL: Color = Color::Rgb(38, 38, 50);

pub const MAX_WIDTH: u16 = 76;
pub const MAX_HEIGHT: u16 = 24;

pub fn centered_box(area: Rect, max_width: u16, max_height: u16) -> Rect {
    let w = max_width.min(area.width);
    let h = max_height.min(area.height);
    let x = area.x + (area.width.saturating_sub(w)) / 2;
    let y = area.y + (area.height.saturating_sub(h)) / 2;
    Rect {
        x,
        y,
        width: w,
        height: h,
    }
}
