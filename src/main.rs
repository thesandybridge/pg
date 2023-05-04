use clap::{arg, Command};
use anyhow::Result;

fn cli() -> Command {
    Command::new("pg")
        .about("Calculate percentage of growth")
        .subcommand_required(false)
        .arg(arg!(<INITIAL_VALUE> "The initial value").required(true))
        .arg(arg!(<FINAL_VALUE> "The final value").required(true))
}

fn main() -> Result<()>{
    let command = cli().get_matches();

    let initial_value = pg::parse_with_commas(command.get_one("INITIAL_VALUE").unwrap())?;
    let final_value = pg::parse_with_commas(command.get_one("FINAL_VALUE").unwrap())?;

    let result = pg::percentage_of_growth(initial_value, final_value)?;

    println!("{:.2}%", result);

    Ok(())
}
