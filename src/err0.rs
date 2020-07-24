
enum EAA {
    AA,
    BB,
}

impl From<u32> for EAA {
    fn from(_: u32) -> Self {
        EAA::AA
    }
}

fn e2(err: Result<u8, u32>) -> Result<u8, EAA> {
    Ok(err?)
}
