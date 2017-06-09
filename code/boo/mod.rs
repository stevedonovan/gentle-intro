pub fn answer()->u32 {
	42
}

pub mod bar {
    pub fn question() -> &'static str {
        "the meaning of everything"
    }
}
