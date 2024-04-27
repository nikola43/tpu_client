use solana_client::tpu_client::{TpuClient, TpuClientConfig};
use std::sync::Arc;

#[no_mangle]
pub extern "C" fn send_tpu_tx(
    rpc_url: *const i8,
    rpc_ws_url: *const i8,
    wire_transaction: *const u8, // Vec<u8>
    wire_transaction_len: usize, // Length of the transaction data
) -> bool {
    let rpc_url = unsafe { std::ffi::CStr::from_ptr(rpc_url).to_str().unwrap() };
    let rpc_ws_url = unsafe { std::ffi::CStr::from_ptr(rpc_ws_url).to_str().unwrap() };
    let wire_transaction = unsafe { std::slice::from_raw_parts(wire_transaction, wire_transaction_len) };
    let rpc_client = Arc::new(solana_client::rpc_client::RpcClient::new(rpc_url));
    let config = TpuClientConfig::default();
    let tpu_client = TpuClient::new(rpc_client, rpc_ws_url, config).unwrap();
    let result = tpu_client.send_wire_transaction(wire_transaction.to_vec());
    return result;
}