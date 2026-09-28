#!/usr/bin/env python3
"""Mesure la parité exacte Rust/Python de ALG_ENERGY_AWARE."""

import argparse
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent
REFERENCE = ROOT / "reference" / "sim_energy_aware.py"
SCENARIOS = {
    "nominal": (100.0, 20, 3.0, 15.0, 10.0),
    "constrained": (60.0, 30, 2.0, 20.0, 10.0),
    "long": (100.0, 100, 1.2, 25.0, 15.0),
    "infeasible": (20.0, 20, 1.0, 15.0, 10.0),
    "zero_cost": (100.0, 20, 0.0, 15.0, 10.0),
    "high_reserve": (100.0, 40, 2.0, 20.0, 45.0),
}


def data_line(command):
    result = subprocess.run(command, capture_output=True, text=True, check=True)
    return result.stdout.strip().splitlines()[-1]


def arguments(seed, scenario):
    initial, steps, cost, return_cost, reserve = scenario
    return [str(seed), str(initial), str(steps), str(cost), str(return_cost), str(reserve)]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binaire", default=str(ROOT / "target" / "release" / "energy_aware_rs"))
    parser.add_argument("--graines", nargs=2, type=int, default=[1000, 1050], metavar=("DEBUT", "FIN"))
    parser.add_argument("--scenarios", nargs="*", choices=SCENARIOS, default=list(SCENARIOS))
    args = parser.parse_args()

    binary = Path(args.binaire)
    if not binary.exists():
        print(f"ERREUR : binaire introuvable : {binary}", file=sys.stderr)
        return 2

    total = 0
    differences = 0
    for name in args.scenarios:
        identical = 0
        count = 0
        for seed in range(args.graines[0], args.graines[1]):
            count += 1
            total += 1
            positional = arguments(seed, SCENARIOS[name])
            python_line = data_line([sys.executable, str(REFERENCE), *positional])
            rust_line = data_line([str(binary), *positional])
            if python_line == rust_line:
                identical += 1
            else:
                differences += 1
                if differences <= 10:
                    print(f"DIVERGENCE [{name}] seed={seed}")
                    print(f"  python: {python_line}")
                    print(f"  rust  : {rust_line}")
        print(f"scénario {name:12s} : {identical}/{count} identiques")

    print(f"TOTAL : {total - differences}/{total} identiques")
    print(f"COMPARAISONS : {total}")
    print(f"ECARTS : {differences}")
    if differences == 0:
        print("PARITÉ BIT-À-BIT VÉRIFIÉE ✓")
        return 0
    print("PARITÉ NON VÉRIFIÉE")
    return 1


if __name__ == "__main__":
    sys.exit(main())
