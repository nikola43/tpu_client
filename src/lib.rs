use solana_client::{nonblocking::tpu_client::TpuClient, tpu_client::TpuClientConfig};
use std::{slice, sync::Arc};

#[no_mangle]
pub extern "C" fn send_tpu_tx(
    rpc_url: *const i8,
    rpc_ws_url: *const i8,
    wire_transaction: *const u8, // Vec<u8>
    wire_transaction_len: usize, // Length of the transaction data
) -> bool {
    let rpc_url = unsafe { std::ffi::CStr::from_ptr(rpc_url).to_str().unwrap() };
    let rpc_ws_url = unsafe { std::ffi::CStr::from_ptr(rpc_ws_url).to_str().unwrap() };
    let wire_transaction = unsafe { slice::from_raw_parts(wire_transaction, wire_transaction_len) };
    let rpc_client = Arc::new(solana_client::nonblocking::rpc_client::RpcClient::new(
        rpc_url.to_string(),
    ));

    let config = TpuClientConfig::default();
    let result = tokio::runtime::Runtime::new().unwrap().block_on(async {
        let tpu_client = TpuClient::new("test", rpc_client, rpc_ws_url, config)
            .await
            .unwrap();
        let tx_result = tpu_client
            .try_send_wire_transaction(wire_transaction.to_vec())
            .await;

        match tx_result {
            Ok(_) => return true,
            Err(err) => {
                println!("Failed to send wire transaction: {:?}", err);
                return false;
            }
        }
    });
    result
}
