use crate::rng::PyRandom;
use std::fmt;

/// Paramètres énergétiques d'une mission discrétisée en étapes de couverture.
#[derive(Debug, Clone, Copy)]
pub struct MissionConfig {
    pub initial_energy: f64,
    pub mission_steps: u32,
    pub mission_cost_per_step: f64,
    pub return_cost: f64,
    pub safety_reserve: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    Complete,
    Return,
    Infeasible,
}

impl Decision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "COMPLETE",
            Self::Return => "RETURN",
            Self::Infeasible => "INFEASIBLE",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimulationResult {
    pub seed: u64,
    pub decision: Decision,
    pub completed_steps: u32,
    pub endurance_steps: u32,
    pub coverage: f64,
    pub energy_remaining: f64,
    pub return_energy: f64,
    pub fallback_margin: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigError {
    NonFinite,
    NegativeValue,
    EmptyMission,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite => write!(f, "les paramètres énergétiques doivent être finis"),
            Self::NegativeValue => write!(f, "les paramètres énergétiques doivent être positifs ou nuls"),
            Self::EmptyMission => write!(f, "mission_steps doit être strictement positif"),
        }
    }
}

impl std::error::Error for ConfigError {}

fn validate(config: MissionConfig) -> Result<(), ConfigError> {
    let values = [
        config.initial_energy,
        config.mission_cost_per_step,
        config.return_cost,
        config.safety_reserve,
    ];
    if values.iter().any(|value| !value.is_finite()) {
        return Err(ConfigError::NonFinite);
    }
    if values.iter().any(|value| *value < 0.0) {
        return Err(ConfigError::NegativeValue);
    }
    if config.mission_steps == 0 {
        return Err(ConfigError::EmptyMission);
    }
    Ok(())
}

/// Simule un arbitrage énergie/couverture avec arrêt préventif.
///
/// Chaque étape consomme le coût nominal modulé uniformément dans [0,9; 1,1).
/// Une étape n'est autorisée que si l'énergie qui resterait couvre encore la
/// réserve de sûreté et le coût du trajet de retour, décroissant avec l'avancement.
pub fn simulate(seed: u64, config: MissionConfig) -> Result<SimulationResult, ConfigError> {
    validate(config)?;

    let mut rng = PyRandom::new(seed);
    let mut energy = config.initial_energy;
    let mut completed = 0u32;

    while completed < config.mission_steps {
        let next_completed = completed + 1;
        let next_progress = next_completed as f64 / config.mission_steps as f64;
        let next_return = config.return_cost * (1.0 - next_progress);
        let variability = 0.9 + 0.2 * rng.random();
        let step_cost = config.mission_cost_per_step * variability;
        let energy_after = energy - step_cost;

        if energy_after < config.safety_reserve + next_return {
            break;
        }
        energy = energy_after;
        completed = next_completed;
    }

    let coverage = completed as f64 / config.mission_steps as f64;
    let return_energy = config.return_cost * (1.0 - coverage);
    let fallback_margin = energy - return_energy - config.safety_reserve;
    let decision = if config.initial_energy < config.safety_reserve + config.return_cost {
        Decision::Infeasible
    } else if completed == config.mission_steps {
        Decision::Complete
    } else {
        Decision::Return
    };

    Ok(SimulationResult {
        seed,
        decision,
        completed_steps: completed,
        endurance_steps: completed,
        coverage,
        energy_remaining: energy,
        return_energy,
        fallback_margin,
    })
}
