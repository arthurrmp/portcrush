use std::collections::HashSet;
use std::process::Command;

#[derive(Clone, Debug)]
pub struct PortEntry {
    pub port: u16,
    pub proto: String,
    pub pid: u32,
    pub process: String,
    pub address: String,
}

pub fn scan() -> Vec<PortEntry> {
    let output = Command::new("lsof")
        .args(["-iTCP", "-sTCP:LISTEN", "-P", "-n"])
        .output();

    let output = match output {
        Ok(o) => o,
        Err(_) => return Vec::new(),
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut entries = Vec::new();
    let mut seen = HashSet::new();

    for line in stdout.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 9 {
            continue;
        }

        let process = parts[0].to_string();
        let pid: u32 = match parts[1].parse() {
            Ok(p) => p,
            Err(_) => continue,
        };
        let proto = parts[7].to_string();
        let name = parts[8];

        let (address, port) = parse_name(name);
        if port == 0 {
            continue;
        }

        if !seen.insert((pid, port)) {
            continue;
        }

        entries.push(PortEntry {
            port,
            proto,
            pid,
            process,
            address,
        });
    }

    entries
}

fn parse_name(name: &str) -> (String, u16) {
    if let Some(bracket_end) = name.find("]:") {
        let addr = &name[1..bracket_end];
        let port = name[bracket_end + 2..].parse().unwrap_or(0);
        (addr.to_string(), port)
    } else if let Some(colon) = name.rfind(':') {
        let addr = &name[..colon];
        let port = name[colon + 1..].parse().unwrap_or(0);
        (addr.to_string(), port)
    } else {
        (name.to_string(), 0)
    }
}

pub fn kill_process(pid: u32) -> Result<(), String> {
    let ret = unsafe { libc::kill(pid as i32, libc::SIGTERM) };
    if ret == 0 {
        Ok(())
    } else {
        Err(format!("Failed to kill PID {} (permission denied?)", pid))
    }
}
