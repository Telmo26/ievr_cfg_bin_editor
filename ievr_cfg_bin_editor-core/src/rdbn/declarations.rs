use super::{
    field_type::FieldType, 
    field_type_category::FieldTypeCategory
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RdbnFieldDeclaration {
    pub name: String,
    pub count: i32,
    pub size: i32,
    pub field_type: FieldType,
    pub field_type_category: FieldTypeCategory,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RdbnTypeDeclaration {
    pub name: String,
    pub(crate) unk_hash: u32,
    pub(crate) fields: Vec<RdbnFieldDeclaration>
}