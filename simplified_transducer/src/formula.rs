type VarName = String;

#[derive(Debug)]
pub enum Formula {
    // Constants
    True,
    False,
    // Basic tests
    Less(VarName, VarName),
    Equal(VarName, VarName),
    NotEqual(VarName, VarName),
    Greater(VarName, VarName),
    GreaterEqual(VarName, VarName),
    IsLetter(VarName, String),
    IsNotLetter(VarName, String),
    // Combinators
    Or(Box<Formula>, Box<Formula>),
    And(Box<Formula>, Box<Formula>),
    Not(Box<Formula>),
}

type Arity = usize;
type Names = Vec<VarName>;

pub struct Interpretation<D> {
    pub dimension: Vec<(D, Arity)>,
    pub universe:  fn(D, Names) -> Formula,
    pub order:     fn(D, Names, D, Names) -> Formula,
    pub letter:    fn(D, Names) -> Formula,
}
