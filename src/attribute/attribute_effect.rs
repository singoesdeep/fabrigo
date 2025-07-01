use bon::Builder;

use crate::{attribute::StatType, condition::{Condition, Conditionable}};

#[derive(Builder, Debug, Clone)]
pub struct AttributeEffect {
    pub stat: StatType,
    pub multiplier: f32,
    pub conditions: Vec<Condition>,
}

impl Conditionable for AttributeEffect {
    fn conditions(&self) -> &[Condition] {
        &self.conditions
    }
}
