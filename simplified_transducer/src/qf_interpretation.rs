///
/// In this file we define what is a quantifier
/// free interpretation of words.
///
use crate::ast::Bexpr;

#[derive(Debug, Clone)]
pub struct QfInterpretation {
    pub labels: Vec<String>,
    pub arities: Vec<usize>,
    pub universe: Vec<(usize, Bexpr)>,
    pub order: Vec<(usize, usize, Bexpr)>,
    pub letters: Vec<(usize, String, Bexpr)>,
}

fn create_default_interpretation() -> QfInterpretation {
    QfInterpretation {
        labels: vec!["l1".to_string(), "l2".to_string()],
        arities: vec![1, 2],
        universe: vec![
            (0, Bexpr::Str("true".into())),
            (
                1,
                Bexpr::LessEqual(
                    Box::new(Bexpr::Var("x1".into())),
                    Box::new(Bexpr::Var("x2".into())),
                ),
            ),
        ],
        letters: vec![
            (0, "a".to_string(), Bexpr::Str("true".into())),
            (0, "b".to_string(), Bexpr::Str("false".into())),
            (
                1,
                "a".to_string(),
                Bexpr::Equal(
                    Box::new(Bexpr::Var("x1".into())),
                    Box::new(Bexpr::Var("x2".into())),
                ),
            ),
            (
                1,
                "b".to_string(),
                Bexpr::NotEqual(
                    Box::new(Bexpr::Var("x1".into())),
                    Box::new(Bexpr::Var("x2".into())),
                ),
            ),
        ],
        order: vec![
            (
                0,
                0,
                Bexpr::LessEqual(
                    Box::new(Bexpr::Var("x1".into())),
                    Box::new(Bexpr::Var("y1".into())),
                ),
            ),
            (0, 1, Bexpr::Str("true".into())),
            (1, 0, Bexpr::Str("false".into())),
            (
                1,
                1,
                // lexicographic order "x1 < x2 or x1 = x2 and y1 >= y2"
                Bexpr::Or(
                    Box::new(Bexpr::LessEqual(
                        Box::new(Bexpr::Var("x1".into())),
                        Box::new(Bexpr::Var("x2".into())),
                    )),
                    Box::new(Bexpr::And(
                        Box::new(Bexpr::Equal(
                            Box::new(Bexpr::Var("x1".into())),
                            Box::new(Bexpr::Var("x2".into())),
                        )),
                        Box::new(Bexpr::LessEqual(
                            Box::new(Bexpr::Var("y2".into())),
                            Box::new(Bexpr::Var("y1".into())),
                        )),
                    )),
                ),
            ),
        ],
    }
}

/// TODO: implement this
pub fn evaluate(qf: &QfInterpretation, w: String) -> String {
    unimplemented!()
}
