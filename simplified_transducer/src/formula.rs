#[derive(Debug)]
pub enum Formula<'a> {
    Var('a str),
    Less(Box<Formula<'a>>, Box<Formula<'a>>),
    Greater(Box<Formula<'a>>, Box<Formula<'a>>),
    LessEqual(Box<Formula<'a>>, Box<Formula<'a>>),
    GreaterEqual(Box<Formula<'a>>, Box<Formula<'a>>),
    Equal(Box<Formula<'a>>, Box<Formula<'a>>),
    NotEqual(Box<Formula<'a>>, Box<Formula<'a>>),
    A(Box<Formula<'a>>),
    NotA(Box<Formula<'a>>),
    Or(Box<Formula<'a>>, Box<Formula<'a>>),
    And(Box<Formula<'a>>, Box<Formula<'a>>),
    Not(Box<Formula<'a>>),
}

pub struct Interpreter<'a> {
    pub universe: fn(&'a str) -> Formula<'a>,
    pub order: fn(&'a str, &'a str) -> Formula<'a>,
    pub letter: fn(&'a str, &'a str, &'a str) -> Formula<'a>,
}
