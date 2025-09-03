#[cfg(test)]
mod tests {
    use solana_sdk::{
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction,
    };
    use bs58;
    use solana_client::rpc_client::RpcClient;
    use solana_program::system_instruction::transfer;
    use solana_sdk::pubkey::Pubkey;
    use std::str::FromStr;
    use solana_sdk::hash::hash;


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
    fn airdrop() {
        const RPC_URL: &str = "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";
        
        fn claim_airdrop() {
            // Import our keypair
            let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
            
            // We'll establish a connection to Solana devnet using the const we defined above
            let client = RpcClient::new(RPC_URL);
            
            // We're going to claim 2 devnet SOL tokens (2 billion lamports)
            match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
                Ok(sig) => {
                    println!("Success! Check your TX here:");
                    println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
                }
                Err(err) => {
                    println!("Airdrop failed: {}", err);
                }
            }
        }
        
        claim_airdrop();
    }

    #[test]
    fn transfer_sol() {
       // find  wallet 
       let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
       // Generate a signature from the keypair
       let pubkey = keypair.pubkey();

       let message_bytes = b"I verify my Solana Keypair!";
       let sig = keypair.sign_message(message_bytes);
       let sig_hashed = hash(sig.as_ref()); 

       // Verify the signature using the public key
        match sig.verify(&pubkey.to_bytes(), &sig_hashed.to_bytes()) {
            true => println!("Signature verified"),
            false => println!("Verification failed"),
        }
        let to_pubkey = Pubkey::from_str("Cf27GhB8m4q1p9RK8e5MYgmuGQZQngbBguAyKJWpTEKZ").unwrap();
        // Connect to devnet
        const RPC_URL: &str = "https://api.devnet.solana.com";

        let rpc_client = RpcClient::new(RPC_URL);
        // Fetch recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash,
        );
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }
}