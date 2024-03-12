#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Priority(u8);

impl From<Priority> for u8 {
    fn from(value: Priority) -> Self {
        value.0
    }
}

impl TryFrom<u8> for Priority {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        // TODO check if this is correct (min and max priority)
        Ok(Priority(value))
    }
}
