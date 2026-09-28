use energy_aware_rs::{simulate, Decision, MissionConfig};
use std::process::Command;

#[test]
fn mission_sans_consommation_est_complete() {
    let cfg = MissionConfig {
        initial_energy: 100.0,
        mission_steps: 4,
        mission_cost_per_step: 0.0,
        return_cost: 20.0,
        safety_reserve: 10.0,
    };
    let result = simulate(42, cfg).expect("configuration valide");
    assert_eq!(result.decision, Decision::Complete);
    assert_eq!(result.completed_steps, 4);
    assert_eq!(result.endurance_steps, 4);
    assert_eq!(result.coverage, 1.0);
    assert_eq!(result.return_energy, 0.0);
    assert_eq!(result.fallback_margin, 90.0);
}

#[test]
fn energie_initiale_insuffisante_est_infeasible() {
    let cfg = MissionConfig {
        initial_energy: 25.0,
        mission_steps: 10,
        mission_cost_per_step: 1.0,
        return_cost: 20.0,
        safety_reserve: 10.0,
    };
    let result = simulate(7, cfg).expect("configuration valide");
    assert_eq!(result.decision, Decision::Infeasible);
    assert_eq!(result.completed_steps, 0);
    assert_eq!(result.fallback_margin, -5.0);
}

#[test]
fn cli_emet_le_contrat_csv() {
    let output = Command::new(env!("CARGO_BIN_EXE_energy_aware_rs"))
        .args(["42", "100", "4", "0", "20", "10"])
        .output()
        .expect("exécution CLI");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout UTF-8");
    assert_eq!(
        stdout,
        "seed,initial_energy,mission_steps,mission_cost_per_step,return_cost,safety_reserve,decision,completed_steps,endurance_steps,coverage,energy_remaining,return_energy,fallback_margin\n42,100.000000,4,0.000000,20.000000,10.000000,COMPLETE,4,4,1.000000,100.000000,0.000000,90.000000\n"
    );
}
