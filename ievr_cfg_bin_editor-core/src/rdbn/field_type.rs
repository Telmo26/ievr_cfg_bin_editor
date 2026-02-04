use serde::{Deserialize, Serialize};

#[repr(i16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RdbnFieldType {
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

impl TryFrom<i16> for RdbnFieldType {
    type Error = ();

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RdbnFieldType::AbilityData),
            1 => Ok(RdbnFieldType::EnhanceData),
            2 => Ok(RdbnFieldType::StatusRate),
            3 => Ok(RdbnFieldType::Bool),
            4 => Ok(RdbnFieldType::Byte),
            5 => Ok(RdbnFieldType::Short),
            6 => Ok(RdbnFieldType::Int),
            9 => Ok(RdbnFieldType::ActType),
            10 => Ok(RdbnFieldType::Flag),
            0xD => Ok(RdbnFieldType::Float),
            0xF => Ok(RdbnFieldType::Hash),
            0x12 => Ok(RdbnFieldType::RateMatrix),
            0x13 => Ok(RdbnFieldType::Position),
            0x14 => Ok(RdbnFieldType::String),
            0x15 => Ok(RdbnFieldType::DataTuple),
            _ => Err(()),
        }
    }
}