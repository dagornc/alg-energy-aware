use energy_aware_rs::{MissionConfig, simulate};
use std::env;
use std::process::ExitCode;
use std::str::FromStr;

const HEADER: &str = "seed,initial_energy,mission_steps,mission_cost_per_step,return_cost,safety_reserve,decision,completed_steps,endurance_steps,coverage,energy_remaining,return_energy,fallback_margin";

fn parse<T: FromStr>(raw: &str, name: &str) -> Result<T, String> {
    raw.parse().map_err(|_| format!("argument invalide {name}: {raw}"))
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 7 {
        return Err(format!(
            "usage: {} SEED INITIAL_ENERGY MISSION_STEPS MISSION_COST RETURN_COST SAFETY_RESERVE",
            args.first().map_or("energy_aware_rs", String::as_str)
        ));
    }

    let seed = parse::<u64>(&args[1], "SEED")?;
    let config = MissionConfig {
        initial_energy: parse(&args[2], "INITIAL_ENERGY")?,
        mission_steps: parse(&args[3], "MISSION_STEPS")?,
        mission_cost_per_step: parse(&args[4], "MISSION_COST")?,
        return_cost: parse(&args[5], "RETURN_COST")?,
        safety_reserve: parse(&args[6], "SAFETY_RESERVE")?,
    };
    let result = simulate(seed, config).map_err(|error| error.to_string())?;

    println!("{HEADER}");
    println!(
        "{},{:.6},{},{:.6},{:.6},{:.6},{},{},{},{:.6},{:.6},{:.6},{:.6}",
        result.seed,
        config.initial_energy,
        config.mission_steps,
        config.mission_cost_per_step,
        config.return_cost,
        config.safety_reserve,
        result.decision.as_str(),
        result.completed_steps,
        result.endurance_steps,
        result.coverage,
        result.energy_remaining,
        result.return_energy,
        result.fallback_margin,
    );
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("ERREUR: {error}");
            ExitCode::from(2)
        }
    }
}
