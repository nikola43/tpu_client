use chrono::{DateTime, Local, Utc};

use solana_client::{nonblocking::tpu_client::TpuClient, tpu_client::TpuClientConfig};
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
    let PRIVATE_KEY = "";

    let wallet = import_wallet(PRIVATE_KEY);
    println!("Wallet address: {:?}", wallet.pubkey());

    // let rpc_url = "http://127.0.0.1:8899".to_string();
    // let rpc_ws_url = "ws://127.0.0.1:8900".to_string();

    let rpc_url = "https://api.devnet.solana.com".to_string();
    let rpc_ws_url = "wss://api.devnet.solana.com/".to_string();

    // Initialize RPC client
    let rpc_client = Arc::new(solana_client::nonblocking::rpc_client::RpcClient::new(rpc_url));

    let config = TpuClientConfig::default();
    let latest_blockhash = rpc_client.get_latest_blockhash().await.unwrap();

    // Initialize TPU client
    let tpu_client = TpuClient::new("test", rpc_client, rpc_ws_url.as_str(), config).await.unwrap();

    let recipient_pubkey =
        Pubkey::from_str("8g6WcD6ELCFTffxT2bGJmv1zrs4B647MaxBJhVNRpzc3").unwrap();
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

    let now = Local::now();
    let date_with_seconds = now.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("{}", date_with_seconds);
    let result = tpu_client.try_send_wire_transaction(wire_transaction).await;
    println!("result : {:?}", result);


    // check if transaction was successful
    match result {
        Ok(_) => println!("Wire transaction sent successfully"),
        Err(err) => println!("Failed to send wire transaction: {:?}", err),
    }

    // let now2 = Local::now();
    //let date_with_seconds2 = now2.format("%Y-%m-%d %H:%M:%S").to_string();
    //println!("{}", date_with_seconds2);

    // match result {
    //     true => println!("Wire transaction sent successfully"),
    //     false => println!("Failed to send wire transaction"),
    // }
}

// fn send_tpu_tx(rpc_url: &str, rpc_ws_url: &str, wire_transaction: Vec<u8>) -> bool {
//     let rpc_client = Arc::new(solana_client::rpc_client::RpcClient::new(rpc_url));
//     let config = TpuClientConfig::default();
//     let tpu_client = TpuClient::new("test", rpc_client, rpc_ws_url, config).unwrap();
//     let result = tpu_client.try_send_wire_transaction(wire_transaction);

//     // match result {
//     //     true => println!("Wire transaction sent successfully"),
//     //     false => println!("Failed to send wire transaction"),
//     // }

//     return result;
// }

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
