//This code generates a brand new Solana wallet (Keypair) and displays its credentials.
use solana_sdk::signature::{Keypair, Signer};

fn main() {
    let keypair = Keypair::new();
    println!("New Keypair: {:?}", keypair);
    println!("Public Key: {}", keypair.pubkey());
    println!("Secret Key: {}", keypair.to_base58_string());
}
