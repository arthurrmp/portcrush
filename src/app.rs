use crate::scanner::{self, PortEntry};
use crossterm::event::KeyCode;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq)]
pub enum SortMode {
    Port,
    Pid,
    Process,
}

impl SortMode {
    pub fn label(self) -> &'static str {
        match self {
            SortMode::Port => "port",
            SortMode::Pid => "pid",
            SortMode::Process => "name",
        }
    }

    pub fn next(self) -> Self {
        match self {
            SortMode::Port => SortMode::Process,
            SortMode::Process => SortMode::Pid,
            SortMode::Pid => SortMode::Port,
        }
    }
}

#[derive(PartialEq)]
pub enum AppState {
    Normal,
    ConfirmKill,
    Filtering,
}

pub struct App {
    pub all_ports: Vec<PortEntry>,
    pub filtered: Vec<usize>,
    pub selected: usize,
    pub state: AppState,
    pub filter: String,
    pub sort_mode: SortMode,
    pub should_quit: bool,
    pub spinner_tick: usize,
    pub message: Option<(String, bool, Instant)>,
    last_scan: Instant,
}

const REFRESH_SECS: u64 = 3;

impl App {
    pub fn new() -> Self {
        let all_ports = scanner::scan();
        let filtered: Vec<usize> = (0..all_ports.len()).collect();
        let mut app = Self {
            all_ports,
            filtered,
            selected: 0,
            state: AppState::Normal,
            filter: String::new(),
            sort_mode: SortMode::Port,
            should_quit: false,
            spinner_tick: 0,
            message: None,
            last_scan: Instant::now(),
        };
        app.sort_and_filter();
        app
    }

    pub fn selected_entry(&self) -> Option<&PortEntry> {
        self.filtered
            .get(self.selected)
            .map(|&i| &self.all_ports[i])
    }

    pub fn handle_key(&mut self, key: KeyCode) {
        match &self.state {
            AppState::Normal => self.handle_normal_key(key),
            AppState::ConfirmKill => self.handle_confirm_key(key),
            AppState::Filtering => self.handle_filter_key(key),
        }
    }

    fn handle_normal_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected + 1 < self.filtered.len() {
                    self.selected += 1;
                }
            }
            KeyCode::Enter | KeyCode::Char('x') => {
                if self.selected_entry().is_some() {
                    self.state = AppState::ConfirmKill;
                }
            }
            KeyCode::Char('r') => self.refresh(),
            KeyCode::Char('s') => {
                self.sort_mode = self.sort_mode.next();
                self.sort_and_filter();
            }
            KeyCode::Char('/') => {
                self.state = AppState::Filtering;
            }
            _ => {}
        }
    }

    fn handle_confirm_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('y') | KeyCode::Enter => {
                if let Some(entry) = self.selected_entry().cloned() {
                    match scanner::kill_process(entry.pid) {
                        Ok(()) => {
                            self.message = Some((
                                format!(
                                    "Killed {} (PID {}) on :{}",
                                    entry.process, entry.pid, entry.port
                                ),
                                true,
                                Instant::now(),
                            ));
                            self.last_scan =
                                Instant::now() - Duration::from_secs(REFRESH_SECS - 1);
                        }
                        Err(e) => {
                            self.message = Some((e, false, Instant::now()));
                        }
                    }
                }
                self.state = AppState::Normal;
            }
            KeyCode::Char('n') | KeyCode::Esc => {
                self.state = AppState::Normal;
            }
            _ => {}
        }
    }

    fn handle_filter_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                self.filter.clear();
                self.apply_filter();
                self.state = AppState::Normal;
            }
            KeyCode::Enter => {
                self.state = AppState::Normal;
            }
            KeyCode::Backspace => {
                self.filter.pop();
                self.apply_filter();
            }
            KeyCode::Char(c) => {
                self.filter.push(c);
                self.apply_filter();
            }
            _ => {}
        }
    }

    pub fn tick(&mut self) {
        self.spinner_tick = self.spinner_tick.wrapping_add(1);

        if let Some((_, _, at)) = &self.message {
            if at.elapsed().as_secs() >= 2 {
                self.message = None;
            }
        }

        if self.last_scan.elapsed().as_secs() >= REFRESH_SECS {
            self.refresh();
        }
    }

    fn refresh(&mut self) {
        let prev = self.selected_entry().map(|e| (e.pid, e.port));
        self.all_ports = scanner::scan();
        self.sort_and_filter();

        if let Some((pid, port)) = prev {
            if let Some(pos) = self
                .filtered
                .iter()
                .position(|&i| self.all_ports[i].pid == pid && self.all_ports[i].port == port)
            {
                self.selected = pos;
            }
        }

        self.last_scan = Instant::now();
    }

    fn sort_and_filter(&mut self) {
        match self.sort_mode {
            SortMode::Port => self.all_ports.sort_by_key(|e| e.port),
            SortMode::Pid => self.all_ports.sort_by_key(|e| e.pid),
            SortMode::Process => self
                .all_ports
                .sort_by(|a, b| a.process.to_lowercase().cmp(&b.process.to_lowercase())),
        }
        self.apply_filter();
    }

    fn apply_filter(&mut self) {
        let query = self.filter.to_lowercase();
        self.filtered = self
            .all_ports
            .iter()
            .enumerate()
            .filter(|(_, e)| {
                if query.is_empty() {
                    return true;
                }
                e.process.to_lowercase().contains(&query)
                    || e.port.to_string().contains(&query)
                    || e.pid.to_string().contains(&query)
                    || e.address.to_lowercase().contains(&query)
            })
            .map(|(i, _)| i)
            .collect();

        if self.selected >= self.filtered.len() {
            self.selected = self.filtered.len().saturating_sub(1);
        }
    }
}
