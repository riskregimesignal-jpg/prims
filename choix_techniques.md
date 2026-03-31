# Choix techniques pour Prims

- **Langage** : Rust (performance, sécurité)
- **Réseau P2P** : libp2p (gossipsub, kadmelia, noise, mplex)
- **Stockage** : RocksDB
- **Cryptographie** : ed25519-dalek (signatures), sha2 (hash), arkworks (zk-SNARKs)
- **Consensus** : Proof of Stake maison avec votes BFT, slashing
- **Sharding** : Beacon chain + comités de validateurs, protocole cross-shard à deux phases
- **Smart contracts** : wasmtime (WebAssembly), support Rust/AssemblyScript
- **API** : jsonrpsee (JSON-RPC), axum (explorateur web)
- **CLI** : clap
- **Tests & benchmarks** : cargo test, cargo bench, criterion
