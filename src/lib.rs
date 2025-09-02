#[cfg(test)] mod tests {
  use solana_sdk::{signature::{Keypair, Signer}, pubkey::Pubkey};
  use bs58; 
  use std::io::{self, BufRead};
#[test]
  fn keygen() {
    use solana_sdk::{signature::{Keypair, Signer}, pubkey::Pubkey};
    let kp: Keypair = Keypair::new();
    println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
    println!("");
    println!("To save your wallet, copy and paste the following into a JSON file:");
    println!("{:?}", kp.to_bytes());
  }


#[test]
fn wallet_to_base58() {
    use std::fs;
    use std::path::Path;

    // fetch wallet from dev-wallet.json
    let path = Path::new("dev-wallet.json");
    let data = fs::read_to_string(path).expect("Unable to read dev-wallet.json");

    // Parse JSON into Vec<u8>
    let wallet: Vec<u8> = serde_json::from_str(&data).expect("Invalid JSON in dev-wallet.json");

    // Convert to base58
    let base58 = bs58::encode(wallet).into_string();

    println!("Your Base58-encoded private key is:\n{}", base58);
}


#[test]
  fn airdrop() {}

#[test]
  fn transfer_sol() {}
}