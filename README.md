# ALG_ENERGY_AWARE — portage Rust

Implémentation Rust déterministe de référence de `ALG_ENERGY_AWARE`, algorithme conceptuel d'arbitrage entre couverture, endurance et marge de repli.

## Utilisation

```sh
cargo run --release -- 1001 100 20 3 15 10
```

Arguments positionnels : `SEED INITIAL_ENERGY MISSION_STEPS MISSION_COST RETURN_COST SAFETY_RESERVE`.
Le programme émet un en-tête puis une ligne CSV. La référence Python accepte exactement les mêmes arguments :

```sh
python3 reference/sim_energy_aware.py 1001 100 20 3 15 10
```

## Ce qui est implémenté

- une mission discrétisée en étapes de couverture ;
- une consommation par étape modulée dans `[0,9 ; 1,1[` par le MT19937 compatible `random.Random` de CPython ;
- un coût de retour décroissant linéairement avec la couverture réalisée ;
- un arrêt préventif avant qu'une étape ne fasse passer l'énergie sous `réserve + coût de retour` ;
- trois décisions explicites : `COMPLETE`, `RETURN` et `INFEASIBLE` ;
- les métriques demandées : endurance (étapes), couverture réalisée et marge de repli restante ;
- validation des paramètres non finis, négatifs et des missions vides.

Le modèle est local, sans topologie de communication : il représente l'arbitrage énergétique embarqué entre la mission et le repli.

## Ce qui est vérifié

Résultats mesurés localement :

- `cargo build` et `cargo build --release` réussissent ;
- `cargo test` : **7 tests réussis, 0 échec** (4 tests du RNG et 3 tests d'intégration/CLI) ;
- `python3 verify_parite_rust.py` : **300 comparaisons, 300 identiques, 0 écart** sur 50 graines × 6 scénarios ;
- statut : **parité bit-à-bit vérifiée** sur la ligne CSV formatée.

Rejouer les contrôles :

```sh
cargo build --release
cargo test
python3 verify_parite_rust.py
```

## Ce qui ne l'est pas

- Le modèle LikeC4 classe l'algorithme source comme `conceptual`, `idea`, niveau de preuve `NONE` et maturité `TBD` : ce portage n'est donc pas une validation scientifique ou opérationnelle.
- Le coût énergétique linéaire n'est pas calibré sur un véhicule réel et n'intègre ni aérodynamique, ni vent, ni vieillissement batterie, ni incertitude d'estimation de charge.
- La valeur métier de chaque objectif et l'optimisation combinatoire multi-tâches ne sont pas modélisées.
- La parité vérifie les deux implémentations de ce modèle de référence simplifié ; elle ne valide pas le modèle physique.

## Licence

MIT — voir [LICENSE](LICENSE).
