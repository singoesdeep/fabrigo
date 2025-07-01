use bon::Builder;

use crate::condition::{comparator::Comparator, condition_key::ConditionKey};

#[derive(Builder, Debug, Clone)]
pub struct Condition {
    pub target: Box<dyn Targetable>,
    pub key: Box<dyn ConditionKey>,
    pub op: Comparator,
    pub value: String,
    pub chance: u16,
}

pub trait Conditionable {
    fn conditions(&self) -> &[Condition];

    fn check(&self, target: &dyn Targetable) -> bool {
        self.conditions()
            .iter()
            .all(|c| c.key.as_ref().matches(target))
    }
}


pub trait Targetable: std::fmt::Debug + Send + Sync {
    fn get_condition_key_value(&self, key: &dyn ConditionKey) -> Option<String> {
        Some(key.value().to_string())
    }

    fn clone_box(&self) -> Box<dyn Targetable>;
}

impl Clone for Box<dyn Targetable> {
    fn clone(&self) -> Box<dyn Targetable> {
        self.clone_box()
    }
}
