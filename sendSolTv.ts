import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, TransactionInstruction, TransactionMessage, VersionedTransaction } from "@solana/web3.js";
import bs58 from "bs58";
import { load, DataType, open, close } from 'ffi-rs';
import 'dotenv/config'
import os from 'node:os'


const PRIVATE_KEY = process.env.PRIVATE_KEY!;
const wallet = Keypair.fromSecretKey(bs58.decode(PRIVATE_KEY));
const recipient = new PublicKey("8g6WcD6ELCFTffxT2bGJmv1zrs4B647MaxBJhVNRpzc3")
const amountLamports = LAMPORTS_PER_SOL / 100; // Sending 0.01 SOL

async function main() {

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

  //const rpc_url = "https://api.devnet.solana.com";
  //const ws_url = "wss://api.devnet.solana.com";

  let rpc_url = "https://api.devnet.solana.com";
  let ws_url = "wss://api.devnet.solana.com/";
  const connection = new Connection(rpc_url, {
    wsEndpoint: ws_url,
  });

  open({
    library: 'tpu_client', // key
    path: dynamicLib
  })


  const instructions: TransactionInstruction[] = [
    SystemProgram.transfer({
      fromPubkey: wallet.publicKey,
      toPubkey: recipient,
      lamports: amountLamports
    }),
  ];


  // Step 1 - Fetch Latest Blockhash
  const latestBlockhash = await connection.getLatestBlockhash({
    commitment: "confirmed",
  });
  console.log("   ✅ - Fetched latest blockhash. Last Valid Height:", latestBlockhash.lastValidBlockHeight);

  // Step 2 - Generate Transaction Message
  const messageV0 = new TransactionMessage({
    payerKey: wallet.publicKey,
    recentBlockhash: latestBlockhash.blockhash,
    instructions: instructions
  }).compileToV0Message();
  console.log("   ✅ - Compiled Transaction Message");
  const transaction = new VersionedTransaction(messageV0);

  // Step 3 - Sign your transaction with the required `Signers`
  transaction.sign([wallet]);

  const serializedTransaction = transaction.serialize();
  // convert Uint8Array to Buffer
  const txBuffer = Buffer.from(serializedTransaction);



  // Send the transaction to the network
  //const serializedTransaction = transaction.serialize();
  console.log("Sending transaction...", new Date().toISOString());
  const txResult = load({
    library: "tpu_client", // path to the dynamic library file
    funcName: 'send_tpu_tx', // the name of the function to call
    retType: DataType.Boolean, // the return value type
    paramsType: [DataType.String, DataType.String, DataType.U8Array, DataType.I32], // the parameter types
    paramsValue: [rpc_url, ws_url, txBuffer, txBuffer.length] // the actual parameter values
  })
  close('tpu_client')
  console.log("Sent", new Date().toISOString());

  // // compute tx signature
  // const signature = bs58.encode(transaction.signature!);
  // console.log("Transaction sent:", signature);
  // if (txResult) {
  //   console.log(`https://explorer.solana.com/tx/${signature}?cluster=devnet`);
  // }
}

async function createAndSendV0Tx(connection: Connection, txInstructions: TransactionInstruction[]) {
  // Step 1 - Fetch Latest Blockhash
  const latestBlockhash = await connection.getLatestBlockhash({
    commitment: "confirmed",
  });
  console.log("   ✅ - Fetched latest blockhash. Last Valid Height:", latestBlockhash.lastValidBlockHeight);

  // Step 2 - Generate Transaction Message
  const messageV0 = new TransactionMessage({
    payerKey: wallet.publicKey,
    recentBlockhash: latestBlockhash.blockhash,
    instructions: txInstructions
  }).compileToV0Message();
  console.log("   ✅ - Compiled Transaction Message");
  const transaction = new VersionedTransaction(messageV0);

  // Step 3 - Sign your transaction with the required `Signers`
  transaction.sign([wallet]);
  console.log("   ✅ - Transaction Signed");

  // Step 4 - Send our v0 transaction to the cluster
  const txid = await connection.sendTransaction(transaction, { maxRetries: 5 });
  console.log("   ✅ - Transaction sent to network");
  // Step 5 - Confirm Transaction 
  const confirmation = await connection.confirmTransaction({
    signature: txid,
    blockhash: latestBlockhash.blockhash,
    lastValidBlockHeight: latestBlockhash.lastValidBlockHeight
  })
  if (confirmation.value.err) { throw new Error("   ❌ - Transaction not confirmed.") }
  console.log('🎉 Transaction Succesfully Confirmed!', '\n', `https://explorer.solana.com/tx/${txid}?cluster=devnet`);
}

main().catch((error) => {
  console.error("Error:", error);
});
