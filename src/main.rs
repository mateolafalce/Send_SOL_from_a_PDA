use anchor_client::{
    anchor_lang::system_program,
    solana_sdk::{
        pubkey::Pubkey,
        signature::{read_keypair_file, Signature},
    },
    Client, Cluster, Program,
};
use anyhow::Result;
use std::rc::Rc;
use std::str::FromStr;

fn main() -> Result<()> {
    initialize_pda(
        Client::new(
            Cluster::Devnet,
            Rc::new(
                read_keypair_file(&*shellexpand::tilde(
                    "C:/Users/Mateo/.config/solana/id.json",
                ))
                .expect("Example requires a keypair file"),
            ),
        )
        .program(Pubkey::from_str("Ap1RcsWctAUWwMaWpwSd15bWtBQAgL5gcFnvSEXa3Tzh").unwrap()),
    )?;
    send_sol_from_a_pda(
        Client::new(
            Cluster::Devnet,
            Rc::new(
                read_keypair_file(&*shellexpand::tilde(
                    "C:/Users/Mateo/.config/solana/id.json",
                ))
                .expect("Example requires a keypair file"),
            ),
        )
        .program(Pubkey::from_str("Ap1RcsWctAUWwMaWpwSd15bWtBQAgL5gcFnvSEXa3Tzh").unwrap()),
        15000,
    )?;
    Ok(())
}

fn initialize_pda(program: Program) -> Result<()> {
    let (pda, _bump) =
        Pubkey::find_program_address(&[b"send_one_sol", program.payer().as_ref()], &program.id());
    let signature: Signature = program
        .request()
        .accounts(sendsolfromapda::accounts::CratePDA {
            pda: pda,
            user: program.payer(),
            system_program: system_program::ID,
        })
        .args(sendsolfromapda::instruction::InitializePda {})
        .send()?;
    println!("Created pda: {}", signature);
    let tx = program.rpc().request_airdrop(&pda, 195352)?;
    println!("Airdrop: {}", tx);
    Ok(())
}
fn send_sol_from_a_pda(program: Program, amount: u64) -> Result<()> {
    let (pda, _bump) =
        Pubkey::find_program_address(&[b"send_one_sol", program.payer().as_ref()], &program.id());
    let signature: Signature = program
        .request()
        .accounts(sendsolfromapda::accounts::Transaction {
            pda: pda,
            to: program.payer(),
            user: program.payer(),
        })
        .args(sendsolfromapda::instruction::SendSolFromAPda { amount })
        .send()?;
    println!("Transfer: {}", signature);
    Ok(())
}
