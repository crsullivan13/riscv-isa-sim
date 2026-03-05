#[derive(Debug)]
pub enum Trap {
    MisalignedFetch(u32),
    MisalignedLoad(u32),
    MisalignedStore(u32),
    OutOfBounds(u32),
    InvalidInstruction(u32),
}
