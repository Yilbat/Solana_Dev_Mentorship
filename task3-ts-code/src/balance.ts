import { Connection, PublicKey, clusterApiUrl, Keypair } from "@solana/web3.js";

// Fetch the wallet we created (import wallet from a JSON file)
import wallet from "../dev-wallet.json";

// 2: Create a keypair from our wallet file (this contains both public and private key)
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Function to check the Solana wallet balance
async function checkWalletBalance() {
  try {
    // Connect to the Solana devnet
    const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

    // Use the public key from the Keypair
    const publicKey = keypair.publicKey;

    // Fetch the wallet balance
    const balance = await connection.getBalance(publicKey);

    // Convert the balance from lamports to SOL (1 SOL = 1e9 lamports)
    console.log(`Wallet Balance: ${balance / 1e9} SOL`);
  } catch (error) {
    console.error("Error checking wallet balance:", error);
  }
}

// Call the function
checkWalletBalance();

// Wallet Balance: 2 SOL
// Done in 3.36s.