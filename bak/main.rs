use solana_client::tpu_client::{TpuClient, TpuClientConfig};
use solana_sdk::{
    bs58::decode,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::str::FromStr;
use std::sync::Arc;

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
pub async fn main() {
    let PRIVATE_KEY =
        "";

    let wallet = import_wallet(PRIVATE_KEY);
    println!("Wallet address: {:?}", wallet.pubkey());

    let rpc_url = "https://api.devnet.solana.com".to_string();
    let rpc_ws_url = "wss://api.devnet.solana.com/".to_string();

    // Initialize RPC client
    let rpc_client = Arc::new(solana_client::rpc_client::RpcClient::new(rpc_url));

    let config = TpuClientConfig::default();
    let latest_blockhash = rpc_client.get_latest_blockhash().unwrap();

    // Initialize TPU client
    let tpu_client = TpuClient::new(rpc_client, rpc_ws_url.as_str(), config).unwrap();

    let recipient_pubkey =
        Pubkey::from_str("2xjaQvvUxLjdffPWjaaNnXp5aoCRMPhLtLxYPyZNnKQq").unwrap();
    let lamports_to_send = 1_000; // 1 SOL

    let transfer_instruction = system_instruction::transfer(
        &wallet.pubkey(),  // From (payer)
        &recipient_pubkey, // To (recipient)
        lamports_to_send,  // Amount
    );

    let mut transaction =
        Transaction::new_with_payer(&[transfer_instruction], Some(&wallet.pubkey()));
    transaction.sign(&[&wallet], latest_blockhash);
    let wire_transaction = bincode::serialize(&transaction).unwrap();
    let transaction_signature = transaction.signatures[0];

    println!("Sending transaction: {:?}", transaction_signature);

    let result = tpu_client.send_wire_transaction(wire_transaction);

    match result {
        true => println!("Wire transaction sent successfully"),
        false => println!("Failed to send wire transaction"),
    }
}

fn send_tpu_tx(rpc_url: &str, rpc_ws_url: &str, wire_transaction: Vec<u8>) -> bool {
    let rpc_client = Arc::new(solana_client::rpc_client::RpcClient::new(rpc_url));
    let config = TpuClientConfig::default();
    let tpu_client = TpuClient::new(rpc_client, rpc_ws_url, config).unwrap();
    let result = tpu_client.send_wire_transaction(wire_transaction);

    // match result {
    //     true => println!("Wire transaction sent successfully"),
    //     false => println!("Failed to send wire transaction"),
    // }

    return result;
}

fn import_wallet(private_key: &str) -> Keypair {
    let private_key_bytes = decode(private_key)
        .into_vec()
        .map_err(|err| {
            println!("Error decoding base58 private key: {}", err);
            std::process::exit(1);
        })
        .unwrap();

    let wallet = Keypair::from_bytes(&private_key_bytes)
        .map_err(|err| {
            println!("Error creating Keypair from private key bytes: {}", err);
            std::process::exit(1);
        })
        .unwrap();

    return wallet;
}
