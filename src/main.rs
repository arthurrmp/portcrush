mod app;
mod scanner;
mod ui;

use app::App;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use std::io;
use std::process::ExitCode;
use std::time::{Duration, Instant};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if !args.is_empty() {
        return kill_ports(&args);
    }

    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        ratatui::restore();
        original_hook(info);
    }));

    let mut terminal = ratatui::init();
    let result = run(App::new(), &mut terminal);
    ratatui::restore();
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn kill_ports(args: &[String]) -> ExitCode {
    let ports: Vec<u16> = args
        .iter()
        .filter_map(|a| a.parse::<u16>().ok())
        .collect();

    if ports.is_empty() {
        eprintln!("usage: portcrush [PORT ...]");
        return ExitCode::FAILURE;
    }

    let entries = scanner::scan();
    let mut killed = 0;
    let mut failed = false;

    for port in &ports {
        let matches: Vec<_> = entries.iter().filter(|e| e.port == *port).collect();
        if matches.is_empty() {
            eprintln!(":{port} — nothing listening");
            continue;
        }
        for entry in matches {
            match scanner::kill_process(entry.pid) {
                Ok(()) => {
                    println!(
                        ":{} — killed {} (PID {})",
                        entry.port, entry.process, entry.pid
                    );
                    killed += 1;
                }
                Err(e) => {
                    eprintln!(":{} — {} (PID {}): {e}", entry.port, entry.process, entry.pid);
                    failed = true;
                }
            }
        }
    }

    if killed == 0 && failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn run(mut app: App, terminal: &mut ratatui::DefaultTerminal) -> io::Result<()> {
    let tick_rate = Duration::from_millis(80);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|frame| ui::draw(frame, &app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Char('c')
                        && key.modifiers.contains(KeyModifiers::CONTROL)
                    {
                        app.should_quit = true;
                    } else {
                        app.handle_key(key.code);
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}
