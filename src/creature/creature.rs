use bon::Builder;

use crate::{
    attribute::{attribute::Attribute},
    condition::{condition::Targetable, ConditionKey},
};

#[derive(Builder, Debug, Clone)]
pub struct Creature {
    pub attributes: Vec<Attribute>,
}

impl Targetable for Creature {
    fn clone_box(&self) -> Box<dyn Targetable> {
        Box::new(self.clone())
    }

    fn get_condition_key_value(&self, key: &dyn ConditionKey) -> Option<String> {
        Some(key.value().to_string())
    }
}
