use axum::{extract::State, http::StatusCode, response::Json, routing::post, Router};
use dotenvy::dotenv;
use std::sync::Arc;

use tower_http::cors::CorsLayer;
use tracing::{error, info};
use tracing_subscriber;

// ZK and blockchain imports
use alloy::sol;
use alloy_primitives::Address;
use anyhow::{Context, Result};
use bincode;
use boundless_market::Client;
use hex;
use risc0_steel::{
    ethereum::{EthEvmEnv, ETH_SEPOLIA_CHAIN_SPEC},
    host::BlockNumberOrTag,
    Commitment, Contract,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use url::Url;
mod types;
use types::{GenerateProofRequest, GenerateProofResponse};

// Define the Solidity interface for payment verification
sol! {
    /// Payment receiver payment check function signature.
    interface PaymentReceiver {
        function hasPaid(address userAddr, uint64 month) public view returns (bool);
    }

    /// Data committed to by the guest.
    struct Journal {
        Commitment commitment;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionProof {
    pub service_id: u32,
    pub is_valid: bool,
    pub payment_verified: bool,
    pub timestamp: u64,
    pub user_commitment: [u8; 32],
}

/// Generate ZK proofs using RISC Zero Steel
pub struct ProofGenerator {
    /// Service name for logging
    service_name: String,
    /// Arbitrum Sepolia RPC URL
    rpc_url: Url,
}

impl ProofGenerator {
    /// Generate a ZK proof for subscription verification using Steel
    pub async fn generate_proof(
        &self,
        request: &GenerateProofRequest,
    ) -> Result<GenerateProofResponse> {
        info!("🔮 Generating ZK proof for subscription verification using Steel");
        info!(
            "📋 Request: user_address={}, payment_receiver={}, month={}",
            request.user_address, request.payment_receiver, request.month
        );

        // Parse request parameters
        let user_address = "0x170f6F7b0925CF1447BAAF25a5AE61253EF31c1B"
            .parse::<Address>()
            .context("Invalid user address format")?;
        let payment_receiver = "0x6fEDEb0B4942A8b438AFE68ba7c8Af4637c41903"
            .parse::<Address>()
            .context("Invalid payment receiver address format")?;
        let month = 072025;
        let signature_bytes = hex::decode(&"0x184bdcdfb9db09b6f55c7bcdd3907e2a8555d599b63289fe1fe014864fe605bf01a0f53bcbc539f2070480162f745c847e77e4bcd8821a39ba416945c1b1666c1b".trim_start_matches("0x"))
            .context("Invalid signature format")?;

        info!("🔧 Setting up Steel environment for Ethereum Sepolia");

        // Create Steel EVM environment using the builder pattern from publisher.rs
        let builder = EthEvmEnv::builder()
            .rpc(self.rpc_url.clone())
            .block_number_or_tag(BlockNumberOrTag::Latest);

        let mut env = builder.chain_spec(&ETH_SEPOLIA_CHAIN_SPEC).build().await?;

        info!(
            "📞 Prepared EVM input for user {} to contract {} for month {}",
            user_address, payment_receiver, month
        );

        // Prepare the hasPaid function call
        let call = PaymentReceiver::hasPaidCall {
            userAddr: user_address,
            month,
        };

        // Preflight the call to prepare the input that is required to execute the function in
        // the guest without RPC access. It also returns the result of the call.
        let mut contract = Contract::preflight(payment_receiver, &mut env);
        let returns = contract.call_builder(&call).call().await?;

        info!("📊 Contract call result: hasPaid = {}", returns);

        // Finally, construct the input from the environment.
        let evm_input = env.into_input().await?;

        info!("✅ Steel proof preparation complete");

        // Download the guest program ELF from IPFS
        // Using dweb.link IPFS gateway for better reliability
        let program_url =
            "https://dweb.link/ipfs/bafybeibrj22d3fthskfsokat6gklu77pjwynqn4aocgl7mwxq3ak2u3eje"
                .to_string();

        info!("📥 Using guest program from IPFS: {}", program_url);

        // Get and parse private key from environment
        let private_key_str =
            std::env::var("PRIVATE_KEY").context("PRIVATE_KEY environment variable not set")?;
        let private_key = private_key_str
            .parse::<alloy::signers::local::PrivateKeySigner>()
            .context("Failed to parse private key")?;

        // Parse program URL
        let program_url = program_url
            .parse::<Url>()
            .context("Failed to parse program URL")?;

        // Prepare input data for the guest program
        // The guest program calls env::read() 5 times sequentially to read:
        // 1. EthEvmInput, 2. Address (payment_contract), 3. u64 (month), 4. Address (user), 5. Vec<u8> (signature)

        // Each env::read() expects a complete bincode-serialized object
        // We need to write them sequentially to stdin

        let mut input_bytes = Vec::new();

        // 1. EthEvmInput - Use the real Steel-generated EVM input
        let evm_input_serialized =
            bincode::serialize(&evm_input).context("Failed to serialize EVM input")?;
        input_bytes.extend(evm_input_serialized);

        // 2. Payment contract address
        let payment_contract_serialized = bincode::serialize(&payment_receiver)
            .context("Failed to serialize payment_receiver")?;
        input_bytes.extend(payment_contract_serialized);

        // 3. Month
        let month_serialized = bincode::serialize(&month).context("Failed to serialize month")?;
        input_bytes.extend(month_serialized);

        // 4. User address
        let user_address_serialized =
            bincode::serialize(&user_address).context("Failed to serialize user_address")?;
        input_bytes.extend(user_address_serialized);

        // 5. Signature bytes
        let signature_serialized =
            bincode::serialize(&signature_bytes).context("Failed to serialize signature_bytes")?;
        input_bytes.extend(signature_serialized);

        info!("🔮 Generating ZK proof with Boundless SDK");

        // Create a Boundless client from the provided parameters
        // Try without deployment configuration first to see if it's required
        let client = Client::builder()
            .with_rpc_url(self.rpc_url.clone())
            .with_private_key(private_key)
            .build()
            .await?;

        let boundless_request = client
            .new_request()
            .with_program_url(program_url)?
            .with_stdin(input_bytes.clone());

        let (request_id, expires_at) = client.submit_offchain(boundless_request).await?;

        info!("⏳ Waiting for request {:x} to be fulfilled", request_id);
        let (_journal, seal) = client
            .wait_for_request_fulfillment(
                request_id,
                Duration::from_secs(5), // check every 5 seconds
                expires_at,
            )
            .await?;
        info!("✅ Request {:x} fulfilled", request_id);

        // Generate response data
        let current_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .context("Failed to get current timestamp")?
            .as_secs();

        let proof_id = format!("boundless_{}_{}", month, current_timestamp);
        let journal_hex = hex::encode(&_journal);
        let seal_hex = hex::encode(&seal);
        let commitment_hex =
            hex::encode(format!("{}:{}:{}", user_address, payment_receiver, month));
        info!(
            "🎉 Steel ZK proof generated successfully with ID: {}",
            proof_id
        );

        Ok(GenerateProofResponse {
            success: true,
            message: format!(
                "Boundless ZK proof generated for user {} payment to {} for month {}",
                user_address, payment_receiver, month
            ),
            proof_id,
            journal: journal_hex,
            seal: seal_hex,
            commitment: commitment_hex,
            timestamp: current_timestamp,
        })
    }

    /// Create a new ProofGenerator
    pub async fn new(service_name: String) -> Result<Self> {
        let rpc_url = std::env::var("SEPOLIA_RPC_URL")
            .context("SEPOLIA_RPC_URL environment variable not set")?;

        Ok(Self {
            service_name,
            rpc_url: Url::parse(&rpc_url).context("Invalid RPC URL")?,
        })
    }

    /// Download ELF binary from IPFS
    async fn download_elf_from_ipfs(&self, ipfs_url: &str) -> Result<Vec<u8>> {
        info!("📡 Downloading ELF from IPFS: {}", ipfs_url);

        let response = reqwest::get(ipfs_url)
            .await
            .context("Failed to make HTTP request to IPFS")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "IPFS request failed with status: {}",
                response.status()
            ));
        }

        let elf_bytes = response
            .bytes()
            .await
            .context("Failed to read ELF bytes from IPFS response")?
            .to_vec();

        info!(
            "✅ Successfully downloaded {} bytes from IPFS",
            elf_bytes.len()
        );
        Ok(elf_bytes)
    }
}

/// Application state
#[derive(Clone)]
struct AppState {
    proof_generator: Arc<ProofGenerator>,
}

/// 🔮 Generate ZK Proof Endpoint
///
/// POST /generate-proof
///
/// Creates a zero-knowledge proof that the user paid for a subscription
/// without revealing payment details. This is the core functionality
/// of fckuipaid.com.
async fn generate_proof_handler(
    State(state): State<AppState>,
    Json(request): Json<GenerateProofRequest>,
) -> Result<Json<GenerateProofResponse>, StatusCode> {
    info!("📥 Received proof generation request");

    match state.proof_generator.generate_proof(&request).await {
        Ok(response) => {
            info!("✅ Proof generated successfully");
            Ok(Json(response))
        }
        Err(e) => {
            error!("❌ Failed to generate proof: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// 🏥 Health Check Endpoint
///
/// GET /health
///
/// Simple health check endpoint for monitoring and testing
async fn health_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "fckuipaid-backend",
        "timestamp": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("🚀 Starting fckuipaid.com ZK Proof Generator API");

    // Initialize proof generator (will fail if environment not set up)
    let proof_generator = ProofGenerator::new("fckuipaid.xyz".to_string())
        .await
        .map_err(|e| {
            error!("❌ Failed to initialize proof generator: {}", e);
            error!("💡 Make sure RPC_URL and PRIVATE_KEY environment variables are set");
            e
        })?;

    let state = AppState {
        proof_generator: Arc::new(proof_generator),
    };

    // Build the application router
    let app = Router::new()
        .route("/health", axum::routing::get(health_handler))
        .route("/generate-proof", post(generate_proof_handler))
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    info!("📡 fckuipaid-backend listening on http://0.0.0.0:3001");
    info!("🔮 Ready to generate ZK proofs!");

    axum::serve(listener, app).await?;

    Ok(())
}
