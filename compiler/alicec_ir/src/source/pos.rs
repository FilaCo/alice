#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, salsa::Update)]
pub struct BytePos(usize);

impl From<usize> for BytePos {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, salsa::Update)]
pub struct CharPos(usize);

impl From<usize> for CharPos {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
