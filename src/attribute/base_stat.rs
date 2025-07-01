use bon::Builder;

#[derive(Builder, Debug, Clone)]
pub struct BaseStats {
    pub hp: u64,
    pub mp: u64,
    pub attack: u64,
    pub magical_attack: u64,
    pub defense: u64,
    pub magical_defense: u64,
    pub attack_speed: u64,
    pub cast_speed: u64,
}

#[derive(Debug, Clone)]
pub enum StatType {
    HP,
    MP,
    Attack,
    MagicalAttack,
    Defense,
    MagicalDefense,
    AttackSpeed,
    CastSpeed,
}