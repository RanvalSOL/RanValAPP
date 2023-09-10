use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    solana_sdk::{
        instruction::{AccountMeta, Instruction},
        program_pack::Pack,
        program_pack::Sealed,
        pubkey::Pubkey as SolanaPubkey,
        sysvar::{rent::Rent, Sysvar},
    },
    sysvar::SysvarIds,
    sysvar::SysvarId,
};
use std::convert::TryInto;

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &SolanaPubkey,
    accounts: &[SolanaPubkey],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Vanral Smart Contract: Instruction received");

    // Parse accounts
    let accounts_iter = &mut accounts.iter();
    let authority_account = next_account_info(accounts_iter)?;
    let user_account = next_account_info(accounts_iter)?;
    let candy_machine_account = next_account_info(accounts_iter)?;
    let nft_mint_account = next_account_info(accounts_iter)?;
    let nft_token_account = next_account_info(accounts_iter)?;
    let rent_sysvar_account = next_account_info(accounts_iter)?;

    // Ensure the program is the owner of the authority account
    if authority_account.owner != program_id {
        msg!("Authority account is not owned by this program");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Deserialize instruction data
    let instruction_data = InstructionData::unpack(instruction_data)?;

    // Verify that the user account has sufficient SOL to pay for the transaction
    let rent = Rent::from_account_info(rent_sysvar_account)?;
    let required_lamports = rent
        .minimum_balance(instruction_data.text.len())
        .max(1); // Ensure at least 1 lamport is required
    if user_account.lamports() < required_lamports {
        msg!("Insufficient funds to create NFT");
        return Err(ProgramError::InsufficientFunds);
    }

    // Verify that the Candy Machine account has not been initialized yet
    if candy_machine_account.data.borrow()[0] != 0 {
        msg!("Candy Machine is already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // Initialize the Candy Machine account with the provided data
    let candy_machine_data = CandyMachineData {
        authority: *authority_account.key,
        nft_mint: *nft_mint_account.key,
        nft_token_account: *nft_token_account.key,
        text: instruction_data.text,
    };
    CandyMachineData::pack(candy_machine_data, &mut candy_machine_account.data.borrow_mut())?;

    // Transfer required lamports from user account to the program authority account
    **user_account.lamports.borrow_mut() -= required_lamports;
    **authority_account.lamports.borrow_mut() += required_lamports;

    msg!("Candy Machine initialized successfully");
    Ok(())
}

// Define the Candy Machine data structure
#[derive(Debug)]
struct CandyMachineData {
    authority: SolanaPubkey,
    nft_mint: SolanaPubkey,
    nft_token_account: SolanaPubkey,
    text: Vec<u8>,
}

impl Sealed for CandyMachineData {}

impl solana_program::program_pack::Pack for CandyMachineData {
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let authority = SolanaPubkey::new(&src[0..32]);
        let nft_mint = SolanaPubkey::new(&src[32..64]);
        let nft_token_account = SolanaPubkey::new(&src[64..96]);
        let text_len = u32::from_le_bytes(src[96..100].try_into().unwrap()) as usize;
        let text = src[100..(100 + text_len)].to_vec();

        Ok(CandyMachineData {
            authority,
            nft_mint,
            nft_token_account,
            text,
        })
    }

    fn pack_into_slice(&self, _dst: &mut [u8]) {}
}

// Define the instruction data structure
#[derive(Debug)]
struct InstructionData {
    text: Vec<u8>,
}

impl Sealed for InstructionData {}

impl solana_program::program_pack::Pack for InstructionData {
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let text_len = u32::from_le_bytes(src[0..4].try_into().unwrap()) as usize;
        let text = src[4..(4 + text_len)].to_vec();

        Ok(InstructionData { text })
    }

    fn pack_into_slice(&self, _dst: &mut [u8]) {}
}
fn main() {
    
}
