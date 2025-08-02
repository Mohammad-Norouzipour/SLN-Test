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
        id: String,
        callee: String,
        pcm16: [u16; 8000],
    },

    CallSend {
        caller: String,
        callee: String,
    },

    CallAnswer {
        id: String,
        caller: String,
        callee: String,
    },

    CallReject {
        id: String,
        callee: String,
    },

    CallEnd {
        id: String,
    },

    CallCancel {
        id: String,
        pcm16: [u16; 8000],
        description: String,
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
