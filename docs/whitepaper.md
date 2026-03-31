# Whitepaper Prims

Version de travail : étape 10.8  
Statut du projet couvert par ce document : prototype avancé / préparation testnet-mainnet après clôture 10.7  
Date : 31 mars 2026

---

## 1. Résumé exécutif

> À rédiger en dernier, quand tout le document sera stabilisé.

---

## 2. Avertissement sur l état réel du projet

Ce whitepaper décrit Prims à deux niveaux distincts :
- **État implémenté et validé à ce jour** : ce qui existe déjà dans le code, les tests, les benchmarks et le journal d avancement.
- **Éléments prévus par la roadmap** : ce qui est visé plus tard, mais n est pas encore finalisé ni déployé en production.

Le document ne doit pas présenter comme déjà disponible une fonctionnalité qui reste encore au stade de prototype, de validation locale, de préparation testnet ou de plan roadmap.

---

## 3. Vision et proposition de valeur

### 3.1 Vision

Prims vise la construction d une infrastructure blockchain publique capable de combiner haut debit, finalite rapide, securite, simplicite d usage et confidentialite optionnelle. L ambition long terme est de fournir une base technique suffisamment performante pour supporter des paiements, des applications decentralisees et des usages inter-shards sans reproduire les congestions, les frais eleves et les mecanismes d ordre preferentiel observes sur de nombreux reseaux historiques.

Cette vision doit toutefois etre lue avec une distinction claire entre cible et realite actuelle. Au 31 mars 2026, Prims est un prototype Rust avance, fortement teste en local, avec une preparation testnet deja engagee, mais pas encore un reseau mainnet ouvert ni un systeme declare pret pour la production a grande echelle.

### 3.2 Problèmes visés

Prims cherche a traiter plusieurs limites recurrentes des systemes existants :
- debit insuffisant et congestion lors de la montee en charge ;
- frais variables ou eleves selon la demande ;
- priorisation par frais qui favorise les comportements opportunistes ;
- exposition aux mecanismes de MEV, de front-running et d attaques sandwich ;
- difficultes de passage a l echelle sans degrader l experience developpeur ;
- manque d options natives pour concilier transparence publique et confidentialite selective ;
- fragmentation entre couche de paiement, couche smart contracts et architectures multi-shards.

Dans l etat actuel du projet, tous ces problemes ne sont pas encore resolus a l echelle d un reseau public ouvert. En revanche, plusieurs briques techniques ont deja ete prototypees, testees et benchmarkees pour preparer une reponse coherente a ces limites.

### 3.3 Positionnement de Prims

Prims se positionne aujourd hui comme un projet de blockchain nouvelle generation en cours de maturation, construit en Rust, avec les choix structurants suivants deja presents dans le prototype :
- un reseau P2P base sur libp2p ;
- un stockage RocksDB ;
- un consensus Proof of Stake avec votes pondérés et logique de finalisation ;
- une mempool partitionnee et un travail explicite sur le parallellisme ;
- une architecture de sharding prototypee ;
- une confidentialite optionnelle basee sur des zk-SNARKs ;
- une execution de smart contracts WebAssembly ;
- une interface outillee via JSON-RPC, CLI et explorateur web.

Ce positionnement reste volontairement pragmatique : Prims ne pretend pas, a ce stade, etre deja un reseau de production. Le projet se situe dans une phase de preparation testnet/mainnet, avec un socle technique deja large, des validations locales nombreuses, et une progression encore gouvernee par la roadmap.

### 3.4 Différenciation recherchée

La differenciation recherchee par Prims repose sur plusieurs axes.

Premier axe : le parallellisme. La roadmap vise une architecture shardee et extensible, tandis que le prototype actuel a deja valide des briques de mempool partitionnee, de consensus de shard et de transactions cross-shard.

Deuxieme axe : une reduction des asymetries d execution. Le projet cherche a limiter les effets de priorisation par frais et les abus d ordre de transaction. A ce jour, le prototype applique deja des frais fixes et ne donne pas de priorite par frais dans la mempool.

Troisieme axe : la confidentialite optionnelle. Prims ne vise pas une opacite totale par defaut, mais une coexistence entre transactions publiques et transactions anonymes, avec passerelles entre les deux modeles. Cette orientation est deja prototypee dans le code, meme si elle doit encore etre durcie et eprouvee davantage avant toute mise en production.

Quatrieme axe : l unification de plusieurs couches dans un meme systeme. Prims cherche a reunir paiements, smart contracts Wasm, sharding et outillage utilisateur/developpeur dans une architecture coherente, au lieu d empiler des composants heterogenes sans ligne directrice commune.

Cinquieme axe : une progression prudente et verifiable. Le projet avance par etapes validees, avec tests, benchmarks, audits documentes, commits traces et sauvegardes locales. Cette discipline constitue elle-meme un element de differenciation important par rapport a des annonces purement speculatives.

---

## 4. Principes de conception

### 4.1 Performance
### 4.2 Sécurité
### 4.3 Simplicité d usage
### 4.4 Extensibilité
### 4.5 Confidentialité optionnelle
### 4.6 Décentralisation progressive

---

## 5. État actuel du projet au 31 mars 2026

### 5.1 Phases déjà validées
### 5.2 Niveau de maturité actuel
### 5.3 Ce qui est démontré par tests et benchmarks
### 5.4 Ce qui reste avant le mainnet

---

## 6. Architecture technique actuelle

### 6.1 Couche réseau P2P
### 6.2 Structures blockchain et stockage
### 6.3 Cryptographie et validation
### 6.4 Consensus Proof of Stake
### 6.5 Mempool partitionnée et parallélisme
### 6.6 Sharding
### 6.7 Confidentialité optionnelle
### 6.8 API RPC, explorateur et CLI
### 6.9 Machine virtuelle Wasm et smart contracts
### 6.10 Testnet, automatisation et sécurité

---

## 7. Fonctionnalités déjà implémentées et validées

### 7.1 Réseau
### 7.2 Stockage
### 7.3 Sécurité transactionnelle
### 7.4 Consensus
### 7.5 Sharding
### 7.6 Confidentialité
### 7.7 Outils développeur et utilisateur
### 7.8 Exécution de smart contracts
### 7.9 Audit et durcissement sécurité

---

## 8. Limites actuelles et points encore en cours

### 8.1 Limites du prototype actuel
### 8.2 Écarts entre vision long terme et état actuel
### 8.3 Hypothèses à confirmer
### 8.4 Points de vigilance sécurité

---

## 9. Tokenomics

### 9.1 Rôle du token PRIMS
### 9.2 Offre totale
### 9.3 Répartition proposée
### 9.4 Usage du token dans le réseau
### 9.5 Récompenses et incitations
### 9.6 Ce qui est déjà décidé vs ce qui reste à formaliser

---

## 10. Gouvernance

### 10.1 Gouvernance visée à long terme
### 10.2 Place de la DAO dans la roadmap
### 10.3 État actuel
### 10.4 Transition progressive vers le mainnet

---

## 11. Roadmap à partir de l état actuel

### 11.1 Étape 10.8 : whitepaper
### 11.2 Étape 10.9 : documentation et tutoriels
### 11.3 Étape 10.10 : derniers audits et tests de pénétration
### 11.4 Phase 11 : préparation du mainnet

---

## 12. Risques, hypothèses et stratégie de livraison

### 12.1 Risques techniques
### 12.2 Risques sécurité
### 12.3 Risques de complexité
### 12.4 Stratégie de progression

---

## 13. Conclusion

---

## 14. Annexes

### 14.1 Choix techniques retenus
### 14.2 Résumé des validations importantes
### 14.3 Terminologie du projet
