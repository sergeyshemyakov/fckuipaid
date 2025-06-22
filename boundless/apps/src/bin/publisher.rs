// Copyright 2025 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// This application demonstrates how to send an off-chain proof request
// to the Bonsai proving service and publish the received proofs directly
// to your deployed app contract.

use alloy_primitives::{Address, U256};
use anyhow::{ensure, Context, Result};
use clap::Parser;
use fckuipaid_methods::{PAYMENT_CHECK_ELF, PAYMENT_CHECK_ID};
use hex;
use revm_primitives::hardfork::SpecId;
use risc0_ethereum_contracts::encode_seal;
use risc0_steel::alloy::{
    network::EthereumWallet,
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    sol,
    sol_types::{SolCall, SolValue},
};
use risc0_steel::{
    config::ChainSpec, ethereum::EthEvmEnv, host::BlockNumberOrTag, Commitment, Contract,
};
use risc0_zkvm::{default_prover, Digest, ExecutorEnv, ProverOpts, VerifierContext};
use tokio::task;
use tracing_subscriber::EnvFilter;
use url::Url;

sol! {
    /// Payment receiver payment check function signature.
    interface PaymentReceiver {
        function hasPaid(address userAddr, uint64 month) public view returns (bool);
    }

    /// Data committed to by the guest.
    struct Journal {
        Commitment commitment;
        uint64 month;
    }
}

/// Simple program to create a proof to increment the Counter contract.
#[derive(Parser)]
struct Args {
    /// Arbitrum Sepolia RPC endpoint URL
    #[arg(long, env = "ARB_RPC_URL")]
    arb_rpc_url: Url,

    /// Arbitrum sepolia block to use as the state for the contract call
    #[arg(long, env = "EXECUTION_BLOCK", default_value_t = BlockNumberOrTag::Parent)]
    execution_block: BlockNumberOrTag,

    /// Address of the Payment Receiver contract
    #[arg(long)]
    payment_receiver: Address,

    /// Address of the user to query the payment for
    #[arg(long)]
    user_address: Address,

    /// Month for which to query the payment
    #[arg(long)]
    month: u64,

    /// Signature bytes as hex string
    #[arg(long)]
    signature: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    // Parse the command line arguments.
    let args = Args::try_parse()?;

    // Create Arbitrum Sepolia chain spec
    let arb_sepolia_chain_spec = ChainSpec::new_single(421614, SpecId::CANCUN);

    let mut env = EthEvmEnv::builder()
        .rpc(args.arb_rpc_url)
        .chain_spec(&arb_sepolia_chain_spec)
        .build()
        .await?;

    // Prepare the function call
    let call = PaymentReceiver::hasPaidCall {
        userAddr: args.user_address,
        month: args.month,
    };

    // Preflight the call to prepare the input that is required to execute the function in
    // the guest without RPC access. It also returns the result of the call.
    let mut contract = Contract::preflight(args.payment_receiver, &mut env);
    let returns = contract.call_builder(&call).call().await?;
    assert!(returns == true);

    // Finally, construct the input from the environment.
    // There are two options: Use EIP-4788 for verification by providing a Beacon API endpoint,
    // or use the regular `blockhash' opcode.
    let evm_input = env.into_input().await?;

    // Create the steel proof.
    let prove_info = task::spawn_blocking(move || {
        // Decode hex signature string to bytes
        let signature_bytes =
            hex::decode(&args.signature.trim_start_matches("0x")).expect("Invalid hex signature");

        let env = ExecutorEnv::builder()
            .write(&evm_input)?
            .write(&args.payment_receiver)?
            .write(&args.month)?
            .write(&args.user_address)?
            .write(&signature_bytes)?
            .build()
            .unwrap();

        default_prover().prove_with_ctx(
            env,
            &VerifierContext::default(),
            PAYMENT_CHECK_ELF,
            &ProverOpts::groth16(),
        )
    })
    .await?
    .context("failed to create proof")?;
    let receipt = prove_info.receipt;
    let journal = &receipt.journal.bytes;

    // Decode and log the commitment
    let journal = Journal::abi_decode(journal).context("invalid journal")?;
    log::debug!("Steel commitment: {:?}", journal.commitment);

    // ABI encode the seal.
    let seal = encode_seal(&receipt).context("invalid receipt")?;

    Ok(())
}
