///
/// In this file we define what is a quantifier
/// free interpretation of words.
///
use crate::ast::Bexpr;

#[derive(Debug, Clone)]
pub struct QfInterpretation {
    pub labels: Vec<(String, usize)>,
    pub universe: Vec<(usize, Bexpr)>,
    pub order: Vec<(usize, usize, Bexpr)>,
    pub letters: Vec<(usize, String, Bexpr)>,
}
