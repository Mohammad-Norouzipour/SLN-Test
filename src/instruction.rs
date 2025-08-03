use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum CallInstruction {
    CallInit {
        caller: String,
        callee: String,
        pda_address: String,
        length: u16,
    },

    CallUpdate {
        session: String,
        callee: String,
        start_index: u8,
        end_index: u8,
        pcm16: [u16; 8000],
    },

    CallSend {
        session: String,
        caller: String,
        callee: String,
    },

    CallAnswer {
        session: String,
        caller: String,
    },

    CallReject {
        session: String,
        callee: String,
    },

    CallEnd {
        session: String,
    },

    CallCancel {
        session: String,
        callee: String,
    },
}

impl CallInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        match variant {
            0 => {
                let payload = CallInstruction::try_from_slice(rest)?;
                Ok(payload)
            }
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
