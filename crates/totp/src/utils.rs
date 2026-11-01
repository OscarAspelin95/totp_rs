pub trait U32Fmt {
    fn as_code_str(&self) -> String;
}

impl U32Fmt for u32 {
    fn as_code_str(&self) -> String {
        format!("{:06}", self)
    }
}
