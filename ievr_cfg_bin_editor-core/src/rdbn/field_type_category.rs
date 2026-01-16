#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RdbnFieldTypeCategory {
    Primitive = 1,
    Special = 2,
    Composite = 3
}

impl TryFrom<i16> for RdbnFieldTypeCategory {
    type Error = ();

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(RdbnFieldTypeCategory::Primitive),
            2 => Ok(RdbnFieldTypeCategory::Special),
            3 => Ok(RdbnFieldTypeCategory::Composite),
            _ => Err(()),
        }
    }
}