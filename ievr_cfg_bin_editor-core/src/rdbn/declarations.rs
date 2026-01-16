use super::{
    field_type::RdbnFieldType, 
    field_type_category::RdbnFieldTypeCategory
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RdbnFieldDeclaration {
    pub name: String,
    pub count: i32,
    pub size: i32,
    pub field_type: RdbnFieldType,
    pub field_type_category: RdbnFieldTypeCategory,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RdbnTypeDeclaration {
    pub name: String,
    pub(crate) unk_hash: u32,
    pub(crate) fields: Vec<RdbnFieldDeclaration>
}