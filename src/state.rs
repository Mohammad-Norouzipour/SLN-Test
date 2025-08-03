use borsh::{BorshDeserialize, BorshSerialize};
#[derive(BorshSerialize, BorshDeserialize, Default)]
pub struct CallAccount {
    pub is_initialized: bool,
    pub session : String
    pub state : u8,
    pub pcm16: [u16; 8000],
    pub caller: String,
    pub callee: String,
}
