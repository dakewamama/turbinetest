#[cfg(test)]
mod tests {
    use solana_sdk::{
        signature::{Keypair, Signer, read_keypair_file},
        pubkey::Pubkey,
    };
    use bs58;
    use std::io::{self, BufRead};
    use solana_client::rpc_client::RpcClient;

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
        
    }
}