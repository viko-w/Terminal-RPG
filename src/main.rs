mod player;
mod ui;

use std::env;

use player::{add_xp, award_cmd, data_file, load_events, load_player, save_player, Player};

const USAGE: &str = "terminal-rpg — reward your terminal work

usage:
  rpg                    show character sheet
  rpg status             show character sheet
  rpg xp <amount>        gain XP
  rpg cmd <command...>   award XP for a command (e.g. rpg cmd git push)
";

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
            ui::cmd_status(&player);
            Ok(())
        }
        Some("xp") => {
            let amount = args.get(1).ok_or("xp amount missing")?;
            cmd_add_xp(&mut player, amount)?;
            save_player(&path, &player)
        }
        Some("cmd") => {
            if args.len() < 2 {
                return Err("command missing".into());
            }
            let input = args[1..].join(" ");
            award_cmd(&mut player, &load_events()?, &input);
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