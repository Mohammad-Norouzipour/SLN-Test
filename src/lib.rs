mod call;
mod instruction;
mod state;

use borsh::{BorshDeserialize, BorshSerialize};

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
            caller,
            callee,
            pda_address,
            length,
        } => {
            call::methods::call_init_pda(program_id, accounts, caller, callee, pda_address, length)
        }

        instruction::CallInstruction::CallUpdate {
            session,
            callee,
            start_index,
            end_index,
            pcm16,
        } => call::methods::call_update(program_id, accounts, id, callee, pcm16),

        instruction::CallInstruction::CallSend {
            session,
            caller,
            callee,
        } => call::methods::call_send(program_id, accounts, callee, caller),

        instruction::CallInstruction::CallAnswer { session, caller } => {
            call::methods::call_answer(program_id, accounts, session, caller)
        }

        instruction::CallInstruction::CallReject { session, callee } => {
            call: methods::call_reject(program_id, accounts, session, callee)
        }

        instruction::CallInstruction::CallEnd { session } => {
            call::methods::call_end(program_id, accounts, session)
        }

        instruction::CallInstruction::CallCancel { session, callee } => {
            call::methods::call_cancel(program_id, accounts, callee)
        }
    }
}
