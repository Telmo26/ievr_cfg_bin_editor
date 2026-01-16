#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FieldTypeCategory {
    Primitive = 1,
    Special = 2,
    Composite = 3
}

impl TryFrom<i16> for FieldTypeCategory {
    type Error = ();

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(FieldTypeCategory::Primitive),
            2 => Ok(FieldTypeCategory::Special),
            3 => Ok(FieldTypeCategory::Composite),
            _ => Err(()),
        }
    }
}