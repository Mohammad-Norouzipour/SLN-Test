pub mod call;
pub mod instruction;
pub mod state;

use borsh::{BorshDeserialize, BorshSerialize};
pub use call::*;
use crate::instruction::*;
use crate::state::*;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let ins = instruction::CallInstruction::unpack(instruction_data)?;
    match ins {
        instruction::CallInstruction::CallInit {
            callee,
            pcm16,
            description,
        } => callInitPDA(program_id, accounts, callee, &pcm16, description),

        instruction::CallInstruction::CallUpdate {
            callee,
            pcm16,
            description,
        } => callUpdate(program_id, accounts, callee, &pcm16, description),

        instruction::CallInstruction::CallSend {
            callee,
            pcm16,
            description,
        } => streamPcm16(program_id, accounts, callee, &pcm16, description),

        instruction::CallInstruction::CallAnswer {
            callee,
            pcm16,
            description,
        } => streamPcm16(program_id, accounts, callee, &pcm16, description),

        instruction::CallInstruction::CallReject {
            callee,
            pcm16,
            description,
        } => streamPcm16(program_id, accounts, callee, &pcm16, description),

        instruction::CallInstruction::CallCancel {
            callee,
            pcm16,
            description,
        } => streamPcm16(program_id, accounts, callee, &pcm16, description),
    }
}

pub fn callInitPDA(
    pID: &Pubkey,
    _accounts: &[AccountInfo],
    callee: String,
    pcm16: &[u16],
    description: String,
) -> ProgramResult {
    msg!("Call Started");
    msg!("Contact is : {}", callee);
    msg!("Description: {}", description);
    // Get Account iterator
    let account_info_iter = &mut _accounts.iter();
    // Get accounts
    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    // Derive PDA
    let (pda, bump_seed) =
        Pubkey::find_program_address(&[initializer.key.as_ref(), callee.as_bytes().as_ref()], pID);

    // Calculate account size required
    let account_len: usize = 1 + (2 * pcm16.len()) + (4 + callee.len()) + (4 + description.len());

    // Calculate rent required
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    // Create the account
    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_account.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            pID,
        ),
        &[
            initializer.clone(),
            pda_account.clone(),
            system_program.clone(),
        ],
        &[&[
            initializer.key.as_ref(),
            callee.as_bytes().as_ref(),
            &[bump_seed],
        ]],
    )?;

    msg!("Call PDA created: {}", pda);

    msg!("Unpacking state account");
    let mut pda_data =
        CallAccount::try_from_slice(&pda_account.data.borrow()).unwrap_or(CallAccount::default());

    pda_data.caller = callee;
    pda_data.callee = callee;
    pda_data.pcm16 = pcm16.clone();
    pda_data.state = description;
    pda_data.is_initialized = true;

    msg!("Serializing account");
    pda_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("State account serialized");
    Ok(())
}

pub fn streamPcm16(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    callee: String,
    pcm16: &[u16],
    description: String,
) -> ProgramResult {
    msg!("Call Started");
    msg!("Contact is : {}", callee);
    msg!("Description: {}", description);
    Ok(())
}

pub fn callUpdate(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    callee: String,
    pcm16: &[u16],
    description: String,
) -> ProgramResult {
    msg!("Call Started");
    msg!("Contact is : {}", callee);
    msg!("Description: {}", description);
    Ok(())
}
