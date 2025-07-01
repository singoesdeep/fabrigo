use bon::Builder;

use crate::attribute::{AttributeEffect};

#[derive(Builder, Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub effects: Vec<AttributeEffect>,
}

