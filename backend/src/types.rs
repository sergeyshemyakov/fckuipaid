use serde::{Deserialize, Serialize};

/// Custom serializer for payment amounts to handle large numbers
mod string_amount {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    pub fn serialize<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        u64::from_str(&s).map_err(serde::de::Error::custom)
    }
}

/// Request to generate a ZK proof for subscription verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateProofRequest {
    /// User's Ethereum address
    pub user_address: String,
    /// Payment amount in wei
    pub payment_receiver: String,
    /// Month of payment
    pub month: u64,
    /// Signature
    pub signature: String,
}

/// Response containing the generated ZK proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateProofResponse {
    /// Whether proof generation was successful
    pub success: bool,
    /// Response message
    pub message: String,
    /// Unique proof identifier
    pub proof_id: String,
    /// Proof journal (public outputs) as hex string
    pub journal: String,
    /// Proof seal (ZK proof) as hex string
    pub seal: String,
    /// User commitment hash as hex string
    pub commitment: String,
    /// Timestamp when proof was generated
    pub timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct VerifyProofRequest {
    /// 🔮 The ZK proof to verify
    pub proof: String,
    /// 🔗 The public commitment
    pub commitment: String,

    /// 📅 Month/year they're claiming payment for
    pub month_year: u64,
}

/// Result of verifying a ZK proof - whether to grant access or not.
#[derive(Debug, Serialize)]
pub struct VerifyProofResponse {
    /// 🚪 Whether access should be granted to the user
    pub access_granted: bool,
    /// ❌ Reason for denial (if access_granted is false)
    pub reason: Option<String>,
    /// 🏷️ Which service was verified (if successful)
    pub verified_service: Option<String>,
    /// 📅 Which month/year was verified (if successful)
    pub verified_month_year: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    /// Human-readable error message
    pub error: String,
    /// Machine-readable error code for frontend handling
    pub error_code: String,
    /// Optional additional details about the error
    pub details: Option<String>,
}

impl ErrorResponse {
    /// Create a new error response
    pub fn new(error: &str, error_code: &str, details: Option<String>) -> Self {
        Self {
            error: error.to_string(),
            error_code: error_code.to_string(),
            details,
        }
    }

    /// Validation error (invalid input data)
    pub fn validation_error(details: String) -> Self {
        Self::new("Validation error", "VALIDATION_ERROR", Some(details))
    }

    /// Proof generation error (ZK proof creation failed)
    pub fn proof_generation_error(details: String) -> Self {
        Self::new("Proof generation failed", "PROOF_ERROR", Some(details))
    }

    /// Proof verification error (ZK proof verification failed)
    pub fn verification_error(details: String) -> Self {
        Self::new(
            "Proof verification failed",
            "VERIFICATION_ERROR",
            Some(details),
        )
    }
}

#[derive(Debug, Serialize)]
pub struct ServiceInfo {
    /// Unique service identifier (1-6)
    pub id: u32,
    /// Human-readable service name
    pub name: String,
    /// Minimum payment amount required (in wei)
    pub min_amount: u64,
    /// Description of the service
    pub description: String,
}

pub fn get_supported_services() -> Vec<ServiceInfo> {
    vec![
        ServiceInfo {
            id: 1,
            name: "VitalikFeetPics.com".to_string(),
            min_amount: 20_000_000_000_000_000_000u128 as u64, // ~$20 in wei (premium content!)
            description: "Premium exclusive content subscription".to_string(),
        },
        ServiceInfo {
            id: 2,
            name: "Netflix".to_string(),
            min_amount: 15_000_000_000_000_000_000u128 as u64, // ~$15 in wei
            description: "Netflix streaming service".to_string(),
        },
        ServiceInfo {
            id: 3,
            name: "Spotify".to_string(),
            min_amount: 10_000_000_000_000_000_000u128 as u64, // ~$10 in wei
            description: "Spotify music streaming".to_string(),
        },
        ServiceInfo {
            id: 4,
            name: "Disney+".to_string(),
            min_amount: 8_000_000_000_000_000_000u128 as u64, // ~$8 in wei
            description: "Disney+ streaming service".to_string(),
        },
        ServiceInfo {
            id: 5,
            name: "Amazon Prime".to_string(),
            min_amount: 12_000_000_000_000_000_000u128 as u64, // ~$12 in wei
            description: "Amazon Prime Video".to_string(),
        },
        ServiceInfo {
            id: 6,
            name: "Hulu".to_string(),
            min_amount: 8_000_000_000_000_000_000u128 as u64, // ~$8 in wei
            description: "Hulu streaming service".to_string(),
        },
    ]
}
