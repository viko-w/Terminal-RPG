use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const SKILLS: [&str; 8] = [
    "Command", "Linux", "Git", "Docker", "Coding", "Networking", "Shell", "DevOps",
];

const USAGE: &str = "terminal-rpg — reward your terminal work

usage:
  rpg                    show character sheet
  rpg status             show character sheet
  rpg xp <amount>        gain XP
";

#[derive(Serialize, Deserialize)]
struct Player {
    level: u64,
    xp: u64,
    total_xp: u64,
    skills: BTreeMap<String, u64>,
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

fn data_file() -> Result<PathBuf, String> {
    let base = env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")))
        .ok_or("HOME not set")?;
    Ok(base.join("terminal-rpg").join("player.toml"))
}

fn load_player(path: &Path) -> Result<Player, String> {
    if !path.exists() {
        return Ok(Player::default());
    }
    let raw = fs::read_to_string(path).map_err(|e| format!("read save: {e}"))?;
    toml::from_str(&raw).map_err(|e| format!("parse save: {e}"))
}

fn save_player(path: &Path, p: &Player) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|e| format!("create dir: {e}"))?;
    }
    let raw = toml::to_string(p).map_err(|e| format!("serialize: {e}"))?;
    fs::write(path, raw).map_err(|e| format!("write save: {e}"))
}

fn xp_needed(level: u64) -> u64 {
    (100.0 * (level as f64).powf(1.5)).round() as u64
}

fn add_xp(p: &mut Player, amount: u64) {
    p.total_xp += amount;
    p.xp += amount;
    let mut leveled = false;
    while p.xp >= xp_needed(p.level) {
        p.xp -= xp_needed(p.level);
        p.level += 1;
        leveled = true;
    }
    println!("+{} XP", amount);
    if leveled {
        println!();
        println!(" ✨ LEVEL UP!");
        println!();
        println!("You are now level {}!", p.level);
    }
}

fn center(s: &str, w: usize) -> String {
    let pad = w.saturating_sub(s.chars().count());
    let left = pad / 2;
    format!("{}{}{}", " ".repeat(left), s, " ".repeat(pad - left))
}

fn cmd_status(p: &Player) {
    const W: usize = 34;
    println!("╔{}╗", "═".repeat(W));
    println!("║{}║", center("TERMINAL HERO", W));
    println!("╠{}╣", "═".repeat(W));
    println!("║{:<W$}║", format!("Level     {}", p.level));
    println!(
        "║{:<W$}║",
        format!("XP        {} / {}", p.xp, xp_needed(p.level))
    );
    println!("║{:<W$}║", format!("Total XP  {}", p.total_xp));
    println!("║{:<W$}║", "");
    let emoji: [&str; 8] = ["⚔", "🌿", "🐳", "🐧", "💻", "🔗", "🖥", "🔧"];
    for (i, skill) in SKILLS.iter().enumerate() {
        println!(
            "║{:<W$}║",
            format!("{} {} {}", emoji[i], skill, p.skills.get(*skill).unwrap_or(&0))
        );
    }
    println!("╚{}╝", "═".repeat(W));
}

fn cmd_add_xp(p: &mut Player, amount_str: &str) -> Result<(), String> {
    let amount = amount_str
        .parse::<u64>()
        .map_err(|_| "xp amount must be a number".to_string())?;
    add_xp(p, amount);
    Ok(())
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    let path = data_file()?;
    let mut player = load_player(&path)?;
    match args.first().map(String::as_str) {
        None | Some("status") => {
            cmd_status(&player);
            Ok(())
        }
        Some("xp") => {
            let amount = args.get(1).ok_or("xp amount missing")?;
            cmd_add_xp(&mut player, amount)?;
            save_player(&path, &player)
        }
        Some(other) => Err(format!("unknown command '{other}'\n\n{USAGE}")),
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}