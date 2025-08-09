use crate::state::CallAccount;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

pub fn call_init_pda(
    pid: &Pubkey,
    _accounts: &[AccountInfo],
    caller: String,
    callee: String,
    pda_address: String,
    length: u16,
) -> ProgramResult {
    msg!("callInitPDA");
    msg!("Contact is : {}", callee);
    // Get Account iterator
    let account_info_iter = &mut _accounts.iter();
    // Get accounts
    let payer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    // Derive PDA
    let (pda, bump_seed) =
        Pubkey::find_program_address(&[payer.key.as_ref(), callee.as_bytes().as_ref()], pid);

    // Calculate account size required
    let account_len: usize = 1 + (2 * length) + (4 + callee.len()) + (4 + caller.len());

    // Calculate rent required
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    // Create the account
    invoke_signed(
        &system_instruction::create_account(
            payer.key,
            pda_account.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            pid,
        ),
        &[payer.clone(), pda_account.clone(), system_program.clone()],
        &[&[payer.key.as_ref(), callee.as_bytes().as_ref(), &[bump_seed]]],
    )?;

    msg!("Call PDA created: {}", pda);

    msg!("Unpacking state account");
    let mut pda_data =
        CallAccount::try_from_slice(&pda_account.data.borrow()).unwrap_or(CallAccount::default());

    pda_data.caller = callee;
    pda_data.callee = callee;
    pda_data.pcm16 = (pda_data.pcm16 + 1).clone();
    pda_data.state = pda_data.state;
    pda_data.session = "Calling";
    pda_data.is_initialized = true;

    msg!("Serializing account");
    pda_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("State account serialized");
    Ok(())
}

pub fn call_update(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    session: String,
    callee: String,
    pcm16: u16,
) -> ProgramResult {
    msg!("Call Updated");
    msg!("Contact is : {}", callee);
    Ok(())
}

pub fn call_send(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    callee: String,
    caller: String,
) -> ProgramResult {
    msg!("Call Request Sent to ", callee);
    //Set State to Requested
    //Rise Event Call Sent
    Ok(())
}

pub fn call_answer(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    callee: String,
    caller: String,
    session: String,
) -> ProgramResult {
    msg!("Call Answered");
    //Rise Call Answered Event
    Ok(())
}

pub fn call_reject(
    pid: &Pubkey,
    _accounts: &[AccountInfo],
    callee: String,
    session: String,
) -> ProgramResult {
    msg!("Call Rejected");
    //Rise Call Rejected Event
    Ok(())
}

pub fn call_cancel(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    canceler: String,
    session: String,
) -> ProgramResult {
    msg!("Call Canceled");
    msg!(canceler, " Have Canceled Call ID is ", session);
    Ok(())
}

pub fn call_end(_program_id: &Pubkey, _accounts: &[AccountInfo], session: String) -> ProgramResult {
    msg!("Call Canceled");
    msg!(" Have Ended Call ID is ", session);
    Ok(())
}

pub fn event_rise(_program_id: &Pubkey, _accounts: &[AccountInfo]) -> ProgramResult {
    msg!("Some Event Rised");
    Ok(())
}
