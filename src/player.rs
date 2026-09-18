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