use std::str::FromStr;
use owo_colors::OwoColorize;
use poc_framework::solana_program::pubkey::Pubkey;
use poc_framework::{keypair, RemoteEnvironment,};
use poc_framework::solana_sdk::system_program;
use poc_framework::solana_program::instruction::{AccountMeta, Instruction};

use poc_framework::solana_sdk::{
    signature::{read_keypair_file, Signer},
};

use poc_framework::Environment;
use poc_framework::localhost_client;

use borsh::{BorshSerialize, BorshDeserialize};

// We use the same Structure created in the Smart Contract
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TokenMetadata {
    title: String,
    symbol: String,
    uri: String,
}

pub fn main() {

    let programa_keypair = read_keypair_file("./program/target/so/program-keypair.json").unwrap();
    let programa = programa_keypair.pubkey();
    //mpl token program address
    let mpl_token_metadata = Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").unwrap();
    let cliente1 = localhost_client();
    let mint_account = keypair(1);
    let mint_authority =  keypair(2);
    // exammple:
    // https://github.com/solana-developers/program-examples/blob/c5b1d527ecd5f4afb4fe4c9d9b02fc2f055ff2f1/tokens/token_metadata.json
    // We use the same Structure created in the Smart Contract
    let metadata = TokenMetadata {
        title: String::from("Solana Gold"),
        symbol: String::from("GOLDSOL"),
        uri: String::from("https://images.all-free-download.com/images/graphiclarge/solana_coin_sign_icon_shiny_golden_symmetric_geometrical_design_6919941.jpg"),
    };
    //We create a u8 vector and serialize the metadata
    let mut my_data: Vec<u8> = vec![];
    metadata.serialize(&mut my_data).unwrap();

    //PDA create
    //https://github.com/alexlu0917/SolanaNFTStaking/blob/99684756c661bfe78e22d296ac01d0d74e6779ac/client/src/main.rs
    //let nft = matches.value_of("nft").unwrap().parse::<Pubkey>().unwrap();
    let (metadata_pda, _) = Pubkey::find_program_address(
        &[
        // Avoid this.... 
        // "Program log:  Metadata's key must match seed of ['metadata', program id, mint] provided",
        // Check docs: https://docs.metaplex.com/programs/token-metadata/instructions
        "metadata".as_bytes(),
        &mpl_token_metadata.to_bytes(),
        &mint_account.pubkey().to_bytes(),
        //  &[0]
        ],
        &mpl_token_metadata,
    );

    println!("{:} {:?}", "METADATA PDA Address: ".bold().blue(), metadata_pda.blue());

        //mint_authority = keypair(2)
    let mut env = RemoteEnvironment::new_with_airdrop(cliente1, keypair(2), 10000000000);
            env.execute_as_transaction_debug(
                &[Instruction {
                    program_id: programa,
                    accounts: vec![
                        AccountMeta::new(mint_account.pubkey(), true),
                        AccountMeta::new(metadata_pda, false),
                        AccountMeta::new_readonly(mint_authority.pubkey(), true),
                        AccountMeta::new_readonly(poc_framework::solana_program::sysvar::rent::id(), false),
                        AccountMeta::new_readonly(system_program::ID, false),
                        AccountMeta::new_readonly(poc_framework::spl_token::ID, false),
                        AccountMeta::new_readonly(mpl_token_metadata, false),
                        ],
                        data: metadata.try_to_vec().unwrap(),  
                        }],
                        &[&mint_account, &mint_authority],
                    );
            
            let check_meta = env.get_account(metadata_pda).unwrap();
            println!("Metadata PDA account: {:?}", check_meta);
            println!("");
            println!("Metadata PDA account data: {:?}", check_meta.data);
            
}
