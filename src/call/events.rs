pub enum CallEvents {
    CallInit {
        callee: String,
        pcm16: [u16; 8000],
        description: String,
    },
}

impl CallEvents {}
