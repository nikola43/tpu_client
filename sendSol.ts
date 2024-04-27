import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import bs58 from "bs58";
import { load, DataType, open, close } from 'ffi-rs';
import 'dotenv/config'
import os from 'node:os'





async function main() {
  const PRIVATE_KEY = process.env.PRIVATE_KEY!;
  const platform = os.platform();
  let dynamicLib: string;
  switch (platform) {
    case 'darwin':
      dynamicLib = "./target/aarch64-apple-darwin/release/libtpu_client.dylib"
      break;
    case 'linux':
      dynamicLib = "./target/release/libtpu_client.so"
      break;
    case 'win32':
      dynamicLib = "./target/release/libtpu_client.dll"
      break;
    default:
      throw new Error("Unsupported platform");
  }

  const rpc_url = "https://api.devnet.solana.com";
  const ws_url = "wss://api.devnet.solana.com";
  const connection = new Connection(rpc_url, {
    wsEndpoint: ws_url,
  });

  open({
    library: 'tpu_client', // key
    path: dynamicLib
  })

  const wallet = Keypair.fromSecretKey(bs58.decode(PRIVATE_KEY));
  const recipient = new PublicKey("2xjaQvvUxLjdffPWjaaNnXp5aoCRMPhLtLxYPyZNnKQq")
  const amountLamports = LAMPORTS_PER_SOL / 100; // Sending 0.01 SOL

  const transaction = new Transaction();
  const latestBlockhash = await connection.getLatestBlockhash({
    commitment: "finalized",
  });
  transaction.recentBlockhash = latestBlockhash.blockhash;
  transaction.add(
    SystemProgram.transfer({
      fromPubkey: wallet.publicKey,
      toPubkey: recipient,
      lamports: amountLamports,
    })
  );
  transaction.sign(wallet);

  // Send the transaction to the network
  const serializedTransaction = transaction.serialize();
  const txResult = load({
    library: "tpu_client", // path to the dynamic library file
    funcName: 'send_tpu_tx', // the name of the function to call
    retType: DataType.Boolean, // the return value type
    paramsType: [DataType.String, DataType.String, DataType.U8Array, DataType.I32], // the parameter types
    paramsValue: [rpc_url, ws_url, serializedTransaction, serializedTransaction.length] // the actual parameter values
  })
  close('tpu_client')

  // compute tx signature
  const signature = bs58.encode(transaction.signature!);
  console.log("Transaction sent:", signature);
  if(txResult) {
    console.log(`https://explorer.solana.com/tx/${signature}?cluster=devnet`);
  } 
}

main().catch((error) => {
  console.error("Error:", error);
});