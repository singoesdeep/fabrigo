use crate::condition::{Targetable};

pub trait ConditionKey: std::fmt::Debug + Send + Sync {
    fn value(&self) -> &str;
    fn matches(&self, target: &dyn Targetable) -> bool;
    fn clone_box(&self) -> Box<dyn ConditionKey>;
}

impl Clone for Box<dyn ConditionKey> {
    fn clone(&self) -> Box<dyn ConditionKey> {
        self.clone_box()
    }
}
