use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum CallInstruction {
    CallInit {
        callee: String,
        pcm16: [u16; 8000],
        description: String,
    },

    CallUpdate {
        callee: String,
        pcm16: [u16; 8000],
        description: String,
    },

    CallSend {
        callee: String,
        pcm16: [u16; 8000],
        description: String,
    },

    CallAnswer {
        callee: String,
        pcm16: [u16; 8000],
        description: String,
    },

    CallReject {
        callee: String,
        pcm16: [u16; 8000],
        description: String,
    },

    CallEnd {
        callee: String,
        pcm16: [u16; 8000],
        description: String,
    },

    CallCancel {
        callee: String,
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
