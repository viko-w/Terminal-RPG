use crate::player::{xp_needed, Player, SKILLS};

fn center(s: &str, w: usize) -> String {
    let pad = w.saturating_sub(s.chars().count());
    let left = pad / 2;
    format!("{}{}{}", " ".repeat(left), s, " ".repeat(pad - left))
}

pub fn cmd_status(p: &Player) {
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