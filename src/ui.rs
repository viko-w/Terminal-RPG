use crate::player::{xp_needed, Player, SKILLS};

const RESET: &str = "\x1b[0m";
const CYAN: &str = "\x1b[36m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[1;33m";

fn paint(code: &str, s: &str) -> String {
    format!("{code}{s}{RESET}")
}

fn row(label: &str, value: &str, val_color: &str) {
    let l = format!("{label:<10}");
    let v = format!("{value:>24}");
    println!(
        "{}{}{}{}",
        paint(CYAN, "║"),
        paint(CYAN, &l),
        paint(val_color, &v),
        paint(CYAN, "║")
    );
}

pub fn cmd_status(p: &Player) {
    const WDTH: usize = 34;
    println!("{}", paint(CYAN, &format!("╔{}╗", "═".repeat(WDTH))));
    let pad = (WDTH - "TERMINAL HERO".chars().count()) / 2;
    println!(
        "{}{}{}{}{}",
        paint(CYAN, "║"),
        " ".repeat(pad),
        paint(YELLOW, "TERMINAL HERO"),
        " ".repeat(WDTH - pad - "TERMINAL HERO".chars().count()),
        paint(CYAN, "║")
    );
    println!("{}", paint(CYAN, &format!("╠{}╣", "═".repeat(WDTH))));
    row("Level", &p.level.to_string(), GREEN);
    row(
        "XP",
        &format!("{} / {}", p.xp, xp_needed(p.level)),
        GREEN,
    );
    row("Total XP", &p.total_xp.to_string(), GREEN);
    row("", "", GREEN);
    for skill in SKILLS {
        row(skill, &p.skills.get(skill).unwrap_or(&0).to_string(), GREEN);
    }
    println!("{}", paint(CYAN, &format!("╚{}╝", "═".repeat(WDTH))));
}