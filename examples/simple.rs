use bon::Builder;
use fabrigo::{
    attribute::{Attribute, AttributeEffect},
    condition::{Comparator, ConditionKey, Targetable},
    creature::creature::Creature,
};

fn main() {
    let human_race = CreatureRace::builder()
        .value("human".to_string())
        .special_attributes(vec![])
        .build();

    let mut player = Player::builder()
        .creature(Creature::builder().attributes(vec![]).build())
        .race(human_race)
        .build();

    let strength_effect = AttributeEffect::builder()
        .stat(fabrigo::attribute::StatType::Attack)
        .multiplier(1.4)
        .conditions(vec![])
        .build();
    
    let strength = Attribute::builder()
        .name("Strength".to_string())
        .effects(vec![strength_effect])
        .build();

    player.creature.attributes.push(strength);

    println!("{:#?}", player);
}

#[derive(Builder, Debug, Clone)]
pub struct Player {
    pub creature: Creature,
    pub race: CreatureRace,
}

#[derive(Builder, Debug, Clone)]
pub struct CreatureRace {
    pub value: String,
    pub special_attributes: Vec<Attribute>,
}

impl ConditionKey for CreatureRace {
    fn clone_box(&self) -> Box<dyn ConditionKey> {
        Box::new(self.clone())
    }

    fn value(&self) -> &str {
        &self.value
    }

    fn matches(&self, target: &dyn Targetable) -> bool {
        if let Some(target_value) = target.get_condition_key_value(self) {
            Comparator::compare_string(Comparator::Equal, &self.value().to_string(), &target_value)
        } else {
            false
        }
    }
}
