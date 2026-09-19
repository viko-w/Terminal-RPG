use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub const SKILLS: [&str; 8] = [
    "Command", "Linux", "Git", "Docker", "Coding", "Networking", "Shell", "DevOps",
];

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub level: u64,
    pub xp: u64,
    pub total_xp: u64,
    pub skills: BTreeMap<String, u64>,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            level: 1,
            xp: 0,
            total_xp: 0,
            skills: SKILLS.iter().map(|s| (s.to_string(), 0)).collect(),
        }
    }
}

pub fn data_file() -> Result<PathBuf, String> {
    let base = env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")))
        .ok_or("HOME not set")?;
    Ok(base.join("terminal-rpg").join("player.toml"))
}

pub fn events_file() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("events.toml")
}

pub fn load_player(path: &Path) -> Result<Player, String> {
    if !path.exists() {
        return Ok(Player::default());
    }
    let raw = fs::read_to_string(path).map_err(|e| format!("read save: {e}"))?;
    toml::from_str(&raw).map_err(|e| format!("parse save: {e}"))
}

pub fn save_player(path: &Path, p: &Player) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|e| format!("create dir: {e}"))?;
    }
    let raw = toml::to_string(p).map_err(|e| format!("serialize: {e}"))?;
    fs::write(path, raw).map_err(|e| format!("write save: {e}"))
}

pub fn xp_needed(level: u64) -> u64 {
    (100.0 * (level as f64).powf(1.5)).round() as u64
}

pub fn add_xp(p: &mut Player, amount: u64) {
    p.total_xp += amount;
    p.xp += amount;
    let mut leveled = false;
    while p.xp >= xp_needed(p.level) {
        p.xp -= xp_needed(p.level);
        p.level += 1;
        leveled = true;
    }
    println!("\x1b[32m+{} XP\x1b[0m", amount);
    if leveled {
        println!();
        println!("\x1b[1;33mLEVEL UP!\x1b[0m");
        println!();
        println!("You are now level {}!", p.level);
    }
}

#[derive(Deserialize)]
pub struct Event {
    pub xp: u64,
    #[serde(default)]
    pub skills: Vec<String>,
}

pub fn default_events() -> BTreeMap<String, Event> {
    [
        ("git".to_string(), Event { xp: 10, skills: vec!["Git".to_string()] }),
        ("git commit".to_string(), Event { xp: 15, skills: vec!["Git".to_string()] }),
        ("git push".to_string(), Event { xp: 20, skills: vec!["Git".to_string()] }),
        ("docker build".to_string(), Event { xp: 20, skills: vec!["Docker".to_string(), "DevOps".to_string()] }),
        ("docker compose up".to_string(), Event { xp: 25, skills: vec!["Docker".to_string(), "DevOps".to_string()] }),
    ]
    .into_iter()
    .collect()
}

pub fn load_events() -> Result<BTreeMap<String, Event>, String> {
    let path = events_file();
    if !path.exists() {
        return Ok(default_events());
    }
    let raw = fs::read_to_string(&path).map_err(|e| format!("read config: {e}"))?;
    toml::from_str(&raw).map_err(|e| format!("parse config: {e}"))
}

pub fn match_event<'a>(
    events: &'a BTreeMap<String, Event>,
    cmd: &str,
) -> Option<(&'a str, &'a Event)> {
    events
        .iter()
        .filter(|(key, _)| cmd == *key || cmd.starts_with(&format!("{key} ")))
        .max_by_key(|(key, _)| key.len())
        .map(|(key, ev)| (key.as_str(), ev))
}

pub fn award_cmd(p: &mut Player, events: &BTreeMap<String, Event>, cmd: &str) {
    let (xp, skills) = match match_event(events, cmd) {
        Some((_, ev)) => (ev.xp, ev.skills.clone()),
        None => (5, vec!["Command".to_string()]),
    };
    add_xp(p, xp);
    for s in &skills {
        *p.skills.entry(s.clone()).or_insert(0) += 1;
    }
    println!();
    for s in skills {
        println!("\x1b[32m{} +1\x1b[0m", s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh() -> (Player, BTreeMap<String, Event>) {
        (Player::default(), default_events())
    }

    #[test]
    fn longest_cmd_match_wins() {
        let (mut p, events) = fresh();
        award_cmd(&mut p, &events, "git push origin main");
        assert_eq!(p.xp, 20, "git push must win over git");
        assert_eq!(p.skills["Git"], 1);

        let (mut p, events) = fresh();
        award_cmd(&mut p, &events, "git commit -m x");
        assert_eq!(p.xp, 15);

        let (mut p, events) = fresh();
        award_cmd(&mut p, &events, "git status");
        assert_eq!(p.xp, 10, "bare git must match subcommands");
    }

    #[test]
    fn unmatched_command_gets_fallback_xp() {
        let (mut p, events) = fresh();
        award_cmd(&mut p, &events, "ls -la");
        assert_eq!(p.xp, 5);
        assert_eq!(p.skills["Command"], 1);
    }

    #[test]
    fn parses_events_toml() {
        let raw = r#"
            ["git push"]
            xp = 20
            skills = ["Git"]

            ["cargo build"]
            xp = 15
        "#;
        let events: BTreeMap<String, Event> = toml::from_str(raw).unwrap();
        assert_eq!(events["git push"].xp, 20);
        assert_eq!(events["cargo build"].skills, Vec::<String>::new());
    }
}