#[derive(Debug, Clone, Copy)]
pub enum Comparator {
    Equal,          // ==
    NotEqual,       // !=
    GreaterThan,    // >
    LessThan,       // <
    GreaterOrEqual, // >=
    LessOrEqual,    // <=
}

impl Comparator {
    pub fn compare_int(self, a: &i32, b: &i32) -> bool {
        match self {
            Comparator::Equal => a == b,
            Comparator::NotEqual => a != b,
            Comparator::GreaterThan => a > b,
            Comparator::LessThan => a < b,
            Comparator::GreaterOrEqual => a >= b,
            Comparator::LessOrEqual => a <= b,
        }
    }
    pub fn compare_string(self, a: &String, b: &String) -> bool {
        match self {
            Comparator::Equal => a == b,
            Comparator::NotEqual => a != b,
            _ => false,
        }
    }
}
