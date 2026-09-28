#!/usr/bin/env python3
"""Référence normative de ALG_ENERGY_AWARE.

Arguments positionnels identiques au binaire Rust:
SEED INITIAL_ENERGY MISSION_STEPS MISSION_COST RETURN_COST SAFETY_RESERVE
"""

import math
import random
import sys

HEADER = "seed,initial_energy,mission_steps,mission_cost_per_step,return_cost,safety_reserve,decision,completed_steps,endurance_steps,coverage,energy_remaining,return_energy,fallback_margin"


def simulate(seed, initial_energy, mission_steps, mission_cost, return_cost, reserve):
    values = (initial_energy, mission_cost, return_cost, reserve)
    if not all(math.isfinite(value) for value in values):
        raise ValueError("les paramètres énergétiques doivent être finis")
    if any(value < 0.0 for value in values):
        raise ValueError("les paramètres énergétiques doivent être positifs ou nuls")
    if mission_steps <= 0:
        raise ValueError("mission_steps doit être strictement positif")

    rng = random.Random(seed)
    energy = initial_energy
    completed = 0
    while completed < mission_steps:
        next_completed = completed + 1
        next_progress = float(next_completed) / float(mission_steps)
        next_return = return_cost * (1.0 - next_progress)
        variability = 0.9 + 0.2 * rng.random()
        step_cost = mission_cost * variability
        energy_after = energy - step_cost
        if energy_after < reserve + next_return:
            break
        energy = energy_after
        completed = next_completed

    coverage = float(completed) / float(mission_steps)
    return_energy = return_cost * (1.0 - coverage)
    fallback_margin = energy - return_energy - reserve
    if initial_energy < reserve + return_cost:
        decision = "INFEASIBLE"
    elif completed == mission_steps:
        decision = "COMPLETE"
    else:
        decision = "RETURN"
    return decision, completed, coverage, energy, return_energy, fallback_margin


def main(argv):
    if len(argv) != 7:
        print(f"usage: {argv[0]} SEED INITIAL_ENERGY MISSION_STEPS MISSION_COST RETURN_COST SAFETY_RESERVE", file=sys.stderr)
        return 2
    try:
        seed = int(argv[1])
        initial_energy = float(argv[2])
        mission_steps = int(argv[3])
        mission_cost = float(argv[4])
        return_cost = float(argv[5])
        reserve = float(argv[6])
        decision, completed, coverage, energy, return_energy, margin = simulate(
            seed, initial_energy, mission_steps, mission_cost, return_cost, reserve
        )
    except ValueError as error:
        print(f"ERREUR: {error}", file=sys.stderr)
        return 2

    print(HEADER)
    print(
        f"{seed},{initial_energy:.6f},{mission_steps},{mission_cost:.6f},"
        f"{return_cost:.6f},{reserve:.6f},{decision},{completed},{completed},"
        f"{coverage:.6f},{energy:.6f},{return_energy:.6f},{margin:.6f}"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
