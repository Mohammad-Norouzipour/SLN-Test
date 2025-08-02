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

pub fn callInitPDA(
    pID: &Pubkey,
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
    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    // Derive PDA
    let (pda, bump_seed) =
        Pubkey::find_program_address(&[initializer.key.as_ref(), callee.as_bytes().as_ref()], pID);

    // Calculate account size required
    let account_len: usize = 1 + (2 * length) + (4 + callee.len()) + (4 + caller.len());

    // Calculate rent required
    let rent = Rent::get()?;
    let rent_lamports =     rent.minimum_balance(account_len);

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
    id: String,
    callee: String,
    pcm16: &[u16],
) -> ProgramResult {
    msg!("Call Started");
    msg!("Contact is : {}", callee);
    Ok(())
}

pub fn callSend(
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

pub fn callAnswer(
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

pub fn callReject(
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

pub fn callCancel(
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

pub fn eventRise(
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
