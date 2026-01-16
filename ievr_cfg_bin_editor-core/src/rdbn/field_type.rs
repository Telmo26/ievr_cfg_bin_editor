#[repr(i16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FieldType{
    AbilityData = 0,
    EnhanceData = 1,
    StatusRate = 2,
    Bool = 3,
    Byte = 4,
    Short = 5,
    Int = 6,

    ActType = 9,
    Flag = 10,

    Float = 13,

    Hash = 15,

    RateMatrix = 18,
    Position = 19,
    String = 20,
    DataTuple = 21
}

impl TryFrom<i16> for FieldType {
    type Error = ();

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FieldType::AbilityData),
            1 => Ok(FieldType::EnhanceData),
            2 => Ok(FieldType::StatusRate),
            3 => Ok(FieldType::Bool),
            4 => Ok(FieldType::Byte),
            5 => Ok(FieldType::Short),
            6 => Ok(FieldType::Int),
            9 => Ok(FieldType::ActType),
            10 => Ok(FieldType::Flag),
            0xD => Ok(FieldType::Float),
            0xF => Ok(FieldType::Hash),
            0x12 => Ok(FieldType::RateMatrix),
            0x13 => Ok(FieldType::Position),
            0x14 => Ok(FieldType::String),
            0x15 => Ok(FieldType::DataTuple),
            _ => Err(()),
        }
    }
}