use std::fmt;

pub enum Stmt {
    Print(Pexpr),            //
    For0(String, Vec<Stmt>), //first to last
    For1(String, Vec<Stmt>), //last to first
    If(Bexpr, Vec<Stmt>),    // If statement with condition, then branch
}

pub enum Pexpr {
    Label(String),
    Str(String),
}

#[derive(Debug, Clone)]
pub enum Bexpr {
    Var(String),
    Str(String),
    LessEqual(Box<Bexpr>, Box<Bexpr>), // Less than or equal to comparison
    Less(Box<Bexpr>, Box<Bexpr>),      // Less than comparison
    Equal(Box<Bexpr>, Box<Bexpr>),     // Equal to comparison
    NotEqual(Box<Bexpr>, Box<Bexpr>),  // Not equal to comparison
    GreaterEqual(Box<Bexpr>, Box<Bexpr>), // Greater than or equal to comparison
    Greater(Box<Bexpr>, Box<Bexpr>),   // Greater than comparison
    Not(Box<Bexpr>),
    Label(String),
    And(Box<Bexpr>, Box<Bexpr>),
    Or(Box<Bexpr>, Box<Bexpr>),
}

impl fmt::Display for Bexpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Bexpr::Var(var) => write!(f, "{}", var),
            Bexpr::Str(s) => write!(f, "\"{}\"", s),
            Bexpr::Less(lhs, rhs) => write!(f, "({} < {})", lhs, rhs),
            Bexpr::LessEqual(lhs, rhs) => write!(f, "({} <= {})", lhs, rhs),
            Bexpr::Equal(lhs, rhs) => write!(f, "({} == {})", lhs, rhs),
            Bexpr::NotEqual(lhs, rhs) => write!(f, "({} != {})", lhs, rhs),
            Bexpr::GreaterEqual(lhs, rhs) => write!(f, "({} >= {})", lhs, rhs),
            Bexpr::Greater(lhs, rhs) => write!(f, "({} > {})", lhs, rhs),
            Bexpr::Not(expr) => write!(f, "!( {} )", expr),
            Bexpr::And(lhs, rhs) => write!(f, "( {} && {} )", lhs, rhs),
            Bexpr::Or(lhs, rhs) => write!(f, "( {} || {} )", lhs, rhs),
            Bexpr::Label(label) => write!(f, "{}", label),
        }
    }
}

pub enum Fexpr {
    Number(i32),
    Str(String),
    Var(String),
    Label(String),
}
