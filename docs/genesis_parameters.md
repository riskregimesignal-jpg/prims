# Parametres du bloc genesis de Prims

## Statut
Document de reference pour l etape 11.2 de la roadmap.
Ce document formalise les parametres cibles du bloc genesis avant lancement mainnet.
Il ne modifie pas la branche `mainnet` gelee ; il prepare le lancement depuis `main`.

## Offre totale
- Offre totale cible : 1_000_000_000 PRIMS

## Repartition initiale cible
- 60% recompenses de staking = 600_000_000 PRIMS
- 20% tresorerie DAO = 200_000_000 PRIMS
- 10% premiers contributeurs = 100_000_000 PRIMS
- 10% fondateur = 100_000_000 PRIMS

## Principes de formalisation
- La somme des allocations initiales doit etre egale a l offre totale.
- Les adresses de destination doivent etre definies explicitement avant tout lancement.
- Les allocations genesis doivent etre traçables et documentees.
- La branche `mainnet` reste gelee ; toute preparation documentaire se fait sur `main`.

## Adresses initiales a definir avant lancement
- adresse_tresorerie_dao : A_DEFINIR
- adresse_fondateur : A_DEFINIR
- adresses_premiers_contributeurs : A_DEFINIR
- adresses_validateurs_initiaux : A_DEFINIR

## Points restant a verrouiller
- justification et liste des premiers contributeurs
- liste exacte des validateurs de confiance du lancement initial
- format technique final du fichier ou module genesis
- procedure de verification publique des allocations
