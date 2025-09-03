use solana_client::rpc_client::RpcClient;
use solana_program::system_program;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction,
};
use std::str::FromStr;

pub fn submit_proof() {
    // 1. RPC connection
    const RPC_URL: &str = "https://api.devnet.solana.com";
    let rpc_client = RpcClient::new(RPC_URL.to_string());

    // 2. Load Turbine wallet as signer
    let signer = read_keypair_file("Turbine-wallet.json").expect("Couldn't find wallet file");

    // 3. Define program + accounts
    let mint = Keypair::new(); 
    let turbin3_program =
        Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
    let collection =
        Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();
    let mpl_core_program =
        Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
    let system_program = system_program::id();

    // 4. PDA for prereq account
    let binding = signer.pubkey();
    let seeds = &[b"prereqs", binding.as_ref()];
    let (prereq_pda, _bump) = Pubkey::find_program_address(seeds, &turbin3_program);

    // 5. PDA for authority account
    let seeds_auth = &[b"collection", collection.as_ref()];
    let (authority_pda, _bump_auth) = Pubkey::find_program_address(seeds_auth, &turbin3_program);

    // 6. Instruction data 
    let data = vec![77, 124, 82, 163, 21, 133, 181, 206];

    // 7. Accounts metadata
    let accounts = vec![
        AccountMeta::new(signer.pubkey(), true),
        AccountMeta::new(prereq_pda, false),
        AccountMeta::new(mint.pubkey(), true),
        AccountMeta::new(collection, false),
        AccountMeta::new_readonly(authority_pda, false),
        AccountMeta::new_readonly(mpl_core_program, false),
        AccountMeta::new_readonly(system_program, false),
    ];

    // 8. Blockhash
    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    // 9. Instruction
    let instruction = Instruction {
        program_id: turbin3_program,
        accounts,
        data,
    };

    // 10. Tx
    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&signer.pubkey()),
        &[&signer, &mint],
        blockhash,
    );

    // 11. Send
    let sig = rpc_client
        .send_and_confirm_transaction(&tx)
        .expect("Failed to send transaction");

    println!(
        "Success! TX: https://explorer.solana.com/tx/{}/?cluster=devnet",
        sig
    );
}
