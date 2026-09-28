//! Portage Rust déterministe de `ALG_ENERGY_AWARE`.

pub mod rng;
pub mod sim;

pub use sim::{ConfigError, Decision, MissionConfig, SimulationResult, simulate};
