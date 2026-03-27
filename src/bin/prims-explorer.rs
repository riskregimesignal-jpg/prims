use askama::Template;
use axum::{
    Router,
    extract::{Form, Query, State},
    http::StatusCode,
    response::Html,
    routing::{get, post},
};
use prims::{
    blockchain::types::{DEFAULT_SHARD_ID, FIXED_TRANSACTION_FEE, Transaction, TransactionType},
    crypto::sign_transaction,
};
use reqwest::Client;
use serde::Deserialize;
use serde_json::{Value, json};
use std::{env, fs};

#[derive(Clone)]
struct ExplorerState {
    http: Client,
    rpc_url: String,
    faucet_mode: FaucetMode,
}

#[derive(Clone)]
enum FaucetMode {
    Disabled { reason: String },
    Enabled(FaucetConfig),
}

#[derive(Clone)]
struct FaucetConfig {
    secret_key: [u8; 32],
    public_key: [u8; 32],
    amount: u64,
    source_shard: u16,
    destination_shard: u16,
}

#[derive(Deserialize)]
struct AddressQuery {
    address: String,
}

#[derive(Deserialize)]
struct FaucetRequest {
    address: String,
}

#[derive(Template)]
#[template(
    source = r###"
<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="utf-8">
    <title>Prims Testnet</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, sans-serif; max-width: 1040px; margin: 40px auto; padding: 0 16px; line-height: 1.55; color: #1f2328; }
        h1, h2, h3 { margin-bottom: 0.4em; }
        .card { border: 1px solid #d0d7de; border-radius: 14px; padding: 18px; margin: 18px 0; background: #fff; }
        .grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 18px; }
        input[type="text"] { width: 100%; padding: 10px; box-sizing: border-box; margin: 8px 0 12px; border: 1px solid #d0d7de; border-radius: 10px; }
        button { padding: 10px 14px; cursor: pointer; border: 1px solid #d0d7de; border-radius: 10px; background: #f6f8fa; }
        pre { white-space: pre-wrap; word-break: break-word; background: #f6f8fa; padding: 12px; border-radius: 10px; overflow-x: auto; }
        code { background: #f6f8fa; padding: 2px 6px; border-radius: 6px; }
        .muted { color: #59636e; }
        .ok { color: #0a7f3f; font-weight: 600; }
        .warn { color: #9a6700; font-weight: 600; }
    </style>
</head>
<body>
    <h1>Prims Testnet</h1>
    <p class="muted">
        Site web simple pour le testnet Prims : informations du nœud, recherche de solde, instructions de démarrage et faucet de test.
    </p>

    <div class="card">
        <h2>État du service</h2>
        <p>Endpoint RPC utilisé : <strong>{{ rpc_url }}</strong></p>
        <p>Montant du faucet : <strong>{{ faucet_amount }}</strong> token(s) de test par demande.</p>
        {% if faucet_enabled %}
        <p class="ok">Faucet actif</p>
        <p class="muted">Adresse publique du faucet : <code>{{ faucet_sender_address }}</code></p>
        <h3>État du compte faucet</h3>
        <pre>{{ faucet_account_json }}</pre>
        {% else %}
        <p class="warn">Faucet désactivé</p>
        <pre>{{ faucet_status }}</pre>
        {% endif %}
    </div>

    <div class="card">
        <h2>Instructions testnet</h2>
        <ol>
            <li>Démarrer le nœud principal Prims et laisser l’API RPC disponible sur <code>127.0.0.1:7002</code> ou sur l’URL définie dans <code>PRIMS_RPC_URL</code>.</li>
            <li>Démarrer ce site web avec <code>cargo run --bin prims-explorer</code>.</li>
            <li>Utiliser une adresse publique Ed25519 hexadécimale sur 32 octets pour consulter le solde ou demander des tokens de test.</li>
            <li>Pour activer le faucet, définir <code>PRIMS_SECRET_KEY_FILE</code> ou <code>PRIMS_SECRET_KEY_HEX</code> avant de lancer le site. Il est recommandé d’utiliser un fichier local hors dépôt.</li>
            <li>Optionnel : régler <code>PRIMS_FAUCET_AMOUNT</code>, <code>PRIMS_FAUCET_SOURCE_SHARD</code> et <code>PRIMS_FAUCET_DESTINATION_SHARD</code>.</li>
        </ol>
        <p class="muted">
            Sécurité : ne jamais commiter de clé privée, mot de passe, token GitHub ou fichier sensible dans le dépôt.
        </p>
    </div>

    <div class="grid">
        <div class="card">
            <h2>Recherche de solde</h2>
            <form action="/address" method="get">
                <label for="address">Adresse publique</label>
                <input id="address" name="address" type="text" placeholder="Adresse hexadécimale 32 octets" required>
                <button type="submit">Voir le solde</button>
            </form>
        </div>

        <div class="card">
            <h2>Faucet testnet</h2>
            <form action="/faucet" method="post">
                <label for="faucet-address">Adresse destinataire</label>
                <input id="faucet-address" name="address" type="text" placeholder="Adresse hexadécimale 32 octets" required>
                <button type="submit">Recevoir {{ faucet_amount }} token(s) de test</button>
            </form>
            <p class="muted">
                Le faucet construit une transaction <code>Transfer</code> signée, puis l’envoie au nœud via <code>send_transaction</code>.
            </p>
        </div>
    </div>

    <div class="grid">
        <div class="card">
            <h2>Informations du nœud</h2>
            <pre>{{ info_json }}</pre>
        </div>

        <div class="card">
            <h2>Validateurs</h2>
            <pre>{{ validators_json }}</pre>
        </div>
    </div>

    <div class="card">
        <h2>Commitments anonymes</h2>
        <pre>{{ note_commitments_json }}</pre>
    </div>
</body>
</html>
"###,
    ext = "html"
)]
struct HomeTemplate {
    rpc_url: String,
    faucet_enabled: bool,
    faucet_status: String,
    faucet_sender_address: String,
    faucet_amount: u64,
    faucet_account_json: String,
    info_json: String,
    validators_json: String,
    note_commitments_json: String,
}

#[derive(Template)]
#[template(
    source = r###"
<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="utf-8">
    <title>Prims Testnet - Adresse</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, sans-serif; max-width: 960px; margin: 40px auto; padding: 0 16px; line-height: 1.55; color: #1f2328; }
        .card { border: 1px solid #d0d7de; border-radius: 14px; padding: 18px; margin: 18px 0; background: #fff; }
        pre { white-space: pre-wrap; word-break: break-word; background: #f6f8fa; padding: 12px; border-radius: 10px; overflow-x: auto; }
        a { text-decoration: none; }
        .muted { color: #59636e; }
    </style>
</head>
<body>
    <p><a href="/">← Retour à l’accueil</a></p>
    <h1>Solde d’une adresse</h1>
    <p class="muted">Endpoint RPC utilisé : <strong>{{ rpc_url }}</strong></p>

    <div class="card">
        <h2>Adresse</h2>
        <pre>{{ address }}</pre>
    </div>

    <div class="card">
        <h2>Réponse RPC get_balance</h2>
        <pre>{{ balance_json }}</pre>
    </div>
</body>
</html>
"###,
    ext = "html"
)]
struct AddressTemplate {
    rpc_url: String,
    address: String,
    balance_json: String,
}

#[derive(Template)]
#[template(
    source = r###"
<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="utf-8">
    <title>Prims Testnet - Faucet</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, sans-serif; max-width: 960px; margin: 40px auto; padding: 0 16px; line-height: 1.55; color: #1f2328; }
        .card { border: 1px solid #d0d7de; border-radius: 14px; padding: 18px; margin: 18px 0; background: #fff; }
        pre { white-space: pre-wrap; word-break: break-word; background: #f6f8fa; padding: 12px; border-radius: 10px; overflow-x: auto; }
        a { text-decoration: none; }
        .muted { color: #59636e; }
        .ok { color: #0a7f3f; font-weight: 600; }
        .warn { color: #9a6700; font-weight: 600; }
    </style>
</head>
<body>
    <p><a href="/">← Retour à l’accueil</a></p>
    <h1>Résultat du faucet</h1>
    <p class="muted">Endpoint RPC utilisé : <strong>{{ rpc_url }}</strong></p>

    <div class="card">
        <h2>Adresse destinataire</h2>
        <pre>{{ address }}</pre>
    </div>

    <div class="card">
        <h2>Statut</h2>
        {% if success %}
        <p class="ok">Demande traitée</p>
        {% else %}
        <p class="warn">Demande refusée</p>
        {% endif %}
        <pre>{{ status_message }}</pre>
    </div>

    <div class="card">
        <h2>Détails</h2>
        <pre>{{ response_json }}</pre>
    </div>
</body>
</html>
"###,
    ext = "html"
)]
struct FaucetTemplate {
    rpc_url: String,
    address: String,
    success: bool,
    status_message: String,
    response_json: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let bind_address =
        env::var("PRIMS_EXPLORER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:7003".to_string());
    let rpc_url = env::var("PRIMS_RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:7002".to_string());

    let faucet_mode = FaucetMode::from_env();

    let state = ExplorerState {
        http: Client::builder().build()?,
        rpc_url: rpc_url.clone(),
        faucet_mode,
    };

    let app = Router::new()
        .route("/", get(home))
        .route("/address", get(address))
        .route("/faucet", post(faucet))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&bind_address).await?;
    println!(
        "PRIMS testnet website listening on http://{}",
        listener.local_addr()?
    );
    println!("Connected RPC endpoint: {}", rpc_url);

    axum::serve(listener, app).await?;
    Ok(())
}

impl FaucetMode {
    fn from_env() -> Self {
        match load_faucet_config() {
            Ok(Some(config)) => FaucetMode::Enabled(config),
            Ok(None) => FaucetMode::Disabled {
                reason: "Aucune clé faucet fournie : définis PRIMS_SECRET_KEY_FILE ou PRIMS_SECRET_KEY_HEX pour activer le service de distribution de tokens de test.".to_string(),
            },
            Err(error) => FaucetMode::Disabled {
                reason: format!("Configuration faucet invalide : {error}"),
            },
        }
    }

    fn amount(&self) -> u64 {
        match self {
            FaucetMode::Disabled { .. } => faucet_amount_from_env().unwrap_or(100),
            FaucetMode::Enabled(config) => config.amount,
        }
    }
}

fn load_faucet_config() -> Result<Option<FaucetConfig>, String> {
    let secret_key = if let Ok(path) = env::var("PRIMS_SECRET_KEY_FILE") {
        let raw = fs::read(&path)
            .map_err(|e| format!("impossible de lire PRIMS_SECRET_KEY_FILE={path}: {e}"))?;
        let text = std::str::from_utf8(&raw)
            .map_err(|e| format!("contenu non UTF-8 pour PRIMS_SECRET_KEY_FILE={path}: {e}"))?;
        parse_secret_key_material(text)?
    } else if let Ok(value) = env::var("PRIMS_SECRET_KEY_HEX") {
        parse_secret_key_material(&value)?
    } else {
        return Ok(None);
    };

    let signing_key = ed25519_dalek::SigningKey::from_bytes(&secret_key);
    let public_key = signing_key.verifying_key().to_bytes();

    Ok(Some(FaucetConfig {
        secret_key,
        public_key,
        amount: faucet_amount_from_env()?,
        source_shard: shard_from_env("PRIMS_FAUCET_SOURCE_SHARD")?,
        destination_shard: shard_from_env("PRIMS_FAUCET_DESTINATION_SHARD")?,
    }))
}

fn faucet_amount_from_env() -> Result<u64, String> {
    match env::var("PRIMS_FAUCET_AMOUNT") {
        Ok(raw) => raw
            .parse::<u64>()
            .map_err(|e| format!("PRIMS_FAUCET_AMOUNT invalide: {e}")),
        Err(_) => Ok(100),
    }
}

fn shard_from_env(name: &str) -> Result<u16, String> {
    match env::var(name) {
        Ok(raw) => raw
            .parse::<u16>()
            .map_err(|e| format!("{name} invalide: {e}")),
        Err(_) => Ok(DEFAULT_SHARD_ID),
    }
}

fn parse_secret_key_material(input: &str) -> Result<[u8; 32], String> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err("clé privée vide".to_string());
    }

    if trimmed.starts_with('{') {
        let json_value: Value = serde_json::from_str(trimmed)
            .map_err(|e| format!("contenu JSON de clé privée invalide: {e}"))?;

        if let Some(secret_key) = json_value.get("secret_key").and_then(Value::as_str) {
            return decode_hex_32("secret_key", secret_key);
        }

        if let Some(secret_key) = json_value.get("secret_key_hex").and_then(Value::as_str) {
            return decode_hex_32("secret_key_hex", secret_key);
        }

        return Err(
            "le JSON fourni ne contient ni champ secret_key ni champ secret_key_hex".to_string(),
        );
    }

    decode_hex_32("secret_key", trimmed)
}

fn decode_hex_32(label: &str, value: &str) -> Result<[u8; 32], String> {
    let bytes = decode_hex_vec(label, value)?;
    bytes.try_into().map_err(|_| {
        format!("{label} doit contenir exactement 32 octets après décodage hexadécimal")
    })
}

fn decode_hex_vec(label: &str, value: &str) -> Result<Vec<u8>, String> {
    let normalized = value
        .trim()
        .strip_prefix("0x")
        .or_else(|| value.trim().strip_prefix("0X"))
        .unwrap_or(value.trim());

    hex::decode(normalized).map_err(|e| format!("{label} invalide: {e}"))
}

async fn home(State(state): State<ExplorerState>) -> Result<Html<String>, (StatusCode, String)> {
    let info_json =
        pretty_json_or_error(rpc_call(&state.http, &state.rpc_url, "get_info", json!({})).await);
    let validators_json = pretty_json_or_error(
        rpc_call(&state.http, &state.rpc_url, "get_validators", json!({})).await,
    );
    let note_commitments_json = pretty_json_or_error(
        rpc_call(
            &state.http,
            &state.rpc_url,
            "get_note_commitments",
            json!({}),
        )
        .await,
    );

    let (faucet_enabled, faucet_status, faucet_sender_address, faucet_account_json, faucet_amount) =
        match &state.faucet_mode {
            FaucetMode::Disabled { reason } => (
                false,
                reason.clone(),
                String::new(),
                "Faucet inactif".to_string(),
                state.faucet_mode.amount(),
            ),
            FaucetMode::Enabled(config) => {
                let faucet_sender_address = hex::encode(config.public_key);
                let faucet_account_json = pretty_json_or_error(
                    rpc_call(
                        &state.http,
                        &state.rpc_url,
                        "get_balance",
                        json!({ "address": faucet_sender_address.clone() }),
                    )
                    .await,
                );

                (
                    true,
                    "Faucet actif".to_string(),
                    faucet_sender_address,
                    faucet_account_json,
                    config.amount,
                )
            }
        };

    render_template(&HomeTemplate {
        rpc_url: state.rpc_url,
        faucet_enabled,
        faucet_status,
        faucet_sender_address,
        faucet_amount,
        faucet_account_json,
        info_json,
        validators_json,
        note_commitments_json,
    })
}

async fn address(
    State(state): State<ExplorerState>,
    Query(query): Query<AddressQuery>,
) -> Result<Html<String>, (StatusCode, String)> {
    let address = query.address;

    let balance_json = pretty_json_or_error(
        rpc_call(
            &state.http,
            &state.rpc_url,
            "get_balance",
            json!({ "address": address.clone() }),
        )
        .await,
    );

    render_template(&AddressTemplate {
        rpc_url: state.rpc_url,
        address,
        balance_json,
    })
}

async fn faucet(
    State(state): State<ExplorerState>,
    Form(request): Form<FaucetRequest>,
) -> Result<Html<String>, (StatusCode, String)> {
    let address = request.address;
    let outcome = submit_faucet_transfer(&state, &address).await;

    let (success, status_message, response_json) = match outcome {
        Ok(value) => (
            true,
            "Transaction faucet construite, signée et envoyée au nœud.".to_string(),
            serde_json::to_string_pretty(&value)
                .unwrap_or_else(|_| "<résultat JSON non affichable>".to_string()),
        ),
        Err(error) => (
            false,
            "Le faucet n’a pas pu envoyer la transaction.".to_string(),
            error,
        ),
    };

    render_template(&FaucetTemplate {
        rpc_url: state.rpc_url,
        address,
        success,
        status_message,
        response_json,
    })
}

async fn submit_faucet_transfer(
    state: &ExplorerState,
    recipient_address: &str,
) -> Result<Value, String> {
    let config = match &state.faucet_mode {
        FaucetMode::Disabled { reason } => return Err(reason.clone()),
        FaucetMode::Enabled(config) => config.clone(),
    };

    let recipient = decode_hex_vec("address", recipient_address)?;
    if recipient.len() != 32 {
        return Err(
            "l’adresse destinataire doit contenir exactement 32 octets hexadécimaux".to_string(),
        );
    }

    let faucet_address = hex::encode(config.public_key);
    let faucet_balance = rpc_call(
        &state.http,
        &state.rpc_url,
        "get_balance",
        json!({ "address": faucet_address.clone() }),
    )
    .await?;

    let current_nonce = faucet_balance
        .get("nonce")
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            format!(
                "réponse get_balance inattendue pour le faucet: {}",
                serde_json::to_string_pretty(&faucet_balance)
                    .unwrap_or_else(|_| "<payload non affichable>".to_string())
            )
        })?;

    let mut transaction = Transaction {
        tx_type: TransactionType::Transfer,
        from: config.public_key.to_vec(),
        to: recipient.clone(),
        amount: config.amount,
        fee: FIXED_TRANSACTION_FEE,
        nonce: current_nonce.saturating_add(1),
        source_shard: config.source_shard,
        destination_shard: config.destination_shard,
        signature: Vec::new(),
        data: None,
    };

    transaction.signature = sign_transaction(&transaction, &config.secret_key)
        .map_err(|e| format!("échec de signature de la transaction faucet: {e}"))?;

    let tx_bytes = bincode::serialize(&transaction)
        .map_err(|e| format!("échec de sérialisation bincode de la transaction faucet: {e}"))?;
    let hex_tx = hex::encode(tx_bytes);

    let rpc_result = rpc_call(
        &state.http,
        &state.rpc_url,
        "send_transaction",
        json!({ "hex_tx": hex_tx }),
    )
    .await?;

    Ok(json!({
        "from": faucet_address,
        "to": hex::encode(recipient),
        "amount": config.amount,
        "fee": FIXED_TRANSACTION_FEE,
        "nonce": transaction.nonce,
        "source_shard": config.source_shard,
        "destination_shard": config.destination_shard,
        "rpc_result": rpc_result
    }))
}

async fn rpc_call(
    http: &Client,
    rpc_url: &str,
    method: &str,
    params: Value,
) -> Result<Value, String> {
    let response = http
        .post(rpc_url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params
        }))
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {e}"))?;

    let status = response.status();
    let payload: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to decode JSON-RPC response: {e}"))?;

    if let Some(result) = payload.get("result") {
        Ok(result.clone())
    } else if let Some(error) = payload.get("error") {
        let code = error
            .get("code")
            .and_then(Value::as_i64)
            .map(|v| v.to_string())
            .unwrap_or_else(|| "?".to_string());
        let message = error
            .get("message")
            .and_then(Value::as_str)
            .unwrap_or("unknown RPC error");
        Err(format!("RPC error {code}: {message}"))
    } else {
        Err(format!(
            "Unexpected JSON-RPC payload (HTTP {status}): {}",
            serde_json::to_string_pretty(&payload)
                .unwrap_or_else(|_| "<payload non affichable>".to_string())
        ))
    }
}

fn pretty_json_or_error(result: Result<Value, String>) -> String {
    match result {
        Ok(value) => serde_json::to_string_pretty(&value)
            .unwrap_or_else(|_| "<résultat JSON non affichable>".to_string()),
        Err(error) => format!("Erreur: {error}"),
    }
}

fn render_template<T: Template>(template: &T) -> Result<Html<String>, (StatusCode, String)> {
    template.render().map(Html).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Template render error: {e}"),
        )
    })
}
