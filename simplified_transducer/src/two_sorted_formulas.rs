use crate::formula::{Formula as MonoFormula, Interpretation as MonoInterpretation};

use std::fs::File;
/// This module contains the implementation of the
/// `TwoSortedFormulas` logic, with one sort
/// being finite with equality,
/// and one sort being the sort of positions in a finite word.
///
/// The logic is defined as follows:
///
/// φ := φ ∧ φ | φ ∨ φ | φ → φ | φ ↔ φ | ¬φ
///    | ∃x:L. φ | ∀x:L. φ
///    | ∃x:P. φ | ∀x:P. φ
///    | x = y  (x,y : P)
///    | x <= y (x,y : P)
///    | a(x)   (a in Σ, x : P)
///    | x = l \in L
///
// use File write
use std::io::Write;
use tempfile::tempdir;

pub trait ToSmtSolver {
    fn to_alt_ergo(&self) -> String;
    fn to_smtlib(&self) -> String;
    fn to_mona(&self) -> String;
}

#[derive(Debug, Clone)]
pub enum Sort {
    Label,
    Position,
}

#[derive(Debug, Clone)]
pub enum DefaultAlphabet {
    A,
    B,
    C,
    Hash,
}

#[derive(Debug, Clone)]
pub struct DefaultLabel {
    position: Vec<u32>,
}

impl ToSmtSolver for DefaultAlphabet {
    fn to_smtlib(&self) -> String {
        format!("{:?}", self)
    }
    fn to_mona(&self) -> String {
        format!("{:?}", self)
    }
    fn to_alt_ergo(&self) -> String {
        format!("{:?}", self)
    }
}

impl ToSmtSolver for DefaultLabel {
    fn to_smtlib(&self) -> String {
        let val = self
            .position
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("-");
        format!("label-{}", val)
    }
    fn to_mona(&self) -> String {
        let val = self
            .position
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("x");
        format!("D{}", val)
    }
    fn to_alt_ergo(&self) -> String {
        let val = self
            .position
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("_");
        format!("D{}", val)
    }
}

type VarName = String;
type Variable = (VarName, Sort);

impl ToSmtSolver for Variable {
    fn to_mona(&self) -> String {
        let (name, sort) = self;
        match sort {
            Sort::Label => format!("label_{}", name),
            Sort::Position => format!("pos_{}", name),
        }
    }

    fn to_smtlib(&self) -> String {
        let (name, sort) = self;
        match sort {
            Sort::Label => format!("label-{}", name),
            Sort::Position => format!("pos-{}", name),
        }
    }

    fn to_alt_ergo(&self) -> String {
        let (name, sort) = self;
        match sort {
            Sort::Label => format!("l{}", name),
            Sort::Position => format!("p{}", name),
        }
    }
}

/// We construct a functor for formulas
/// where the fixedpoint is not done.
/// S : sorts
/// F : formulas
/// A : alphabet
///
#[derive(Debug)]
pub enum FormulaF<F, A, S> {
    /// `exists x:S. φ`
    Exists(VarName, Sort, F),
    /// `exists x:S. φ`
    Forall(VarName, Sort, F),
    /// φ ∧ φ
    And(F, F),
    /// φ ∨ φ
    Or(F, F),
    /// φ → φ
    Implies(F, F),
    /// φ ↔ φ
    Iff(F, F),
    /// ¬φ
    Not(F),
    /// x = y  (of sort Label or Position)
    Equal(Sort, VarName, VarName),
    /// x <= y (of sort Position)
    LessEqual(VarName, VarName),
    /// a(x) (a in Σ, x : P)
    LetterAtPos(VarName, A),
    /// x = constant
    EqualConstant(VarName, S),
    /// True
    True,
    /// False
    False,
}

impl<A, S> ToSmtSolver for FormulaF<String, A, S>
where
    A: ToSmtSolver, // alphabet
    S: ToSmtSolver, // labels
{
    fn to_alt_ergo(&self) -> String {
        match self {
            FormulaF::True => "true".to_string(),
            FormulaF::False => "false".to_string(),
            FormulaF::Exists(var, Sort::Label, inner) => {
                format!("exists {} : label. {}", var, inner)
            }
            FormulaF::Exists(var, Sort::Position, inner) => {
                format!(
                    "exists {var} : int. 0 <= {var} and {var} <= len and ({})",
                    inner
                )
            }
            FormulaF::Forall(var, Sort::Label, inner) => {
                format!("forall {} : label. {}", var, inner)
            }
            FormulaF::Forall(var, Sort::Position, inner) => {
                format!(
                    "forall {var} : int. (0 <= {var} and {var} <= len) -> ({})",
                    inner
                )
            }
            FormulaF::And(left, right) => format!("({} and {})", left, right),
            FormulaF::Or(left, right) => format!("({} or {})", left, right),
            FormulaF::Implies(left, right) => format!("({} -> {})", left, right),
            FormulaF::Iff(left, right) => format!("({} <-> {})", left, right),
            FormulaF::Not(inner) => format!("not {}", inner),
            FormulaF::Equal(_, left, right) => format!("{} = {}", left, right),
            FormulaF::LessEqual(left, right) => format!("{} <= {}", left, right),
            FormulaF::LetterAtPos(var, letter) => {
                format!("is_letter_{}({})", letter.to_alt_ergo(), var)
            }
            FormulaF::EqualConstant(var, value) => format!("{} = {}", var, value.to_alt_ergo()),
        }
    }

    /// Convert a formula to the SMT-LIB format.
    /// Roughly speaking this is of the following form:
    /// (assert (exists ((l Label)) (forall ((x Int)) (=> (and (<= 0 x) (<= x len)) (=> (= l label_a) (= (select word x) letter_a)))))
    fn to_smtlib(&self) -> String {
        match self {
            FormulaF::True => "true".to_string(),
            FormulaF::False => "false".to_string(),
            FormulaF::Exists(var, Sort::Label, inner) => {
                format!("(exists (({var} Label)) {inner}) ", var = var)
            }
            FormulaF::Exists(var, Sort::Position, inner) => {
                format!("(exists (({var} Int)) (and (<= 0 {var}) (< {var} len) {inner}))")
            }
            FormulaF::Forall(var, Sort::Label, inner) => {
                format!("(forall (({var} Label)) {inner})")
            }
            FormulaF::Forall(var, Sort::Position, inner) => {
                format!("(forall (({var} Int)) (=> (and (<= 0 {var}) (< {var} len)) {inner}))")
            }
            FormulaF::And(left, right) => format!("(and {} {}) ", left, right),
            FormulaF::Or(left, right) => format!("(or {} {}) ", left, right),
            FormulaF::Implies(left, right) => format!("(=> {} {}) ", left, right),
            FormulaF::Iff(left, right) => format!("(= {} {}) ", left, right),
            FormulaF::Not(inner) => format!("(not {}) ", inner),
            FormulaF::Equal(_, left, right) => format!("(= {} {}) ", left, right),
            FormulaF::LessEqual(left, right) => format!("(<= {} {}) ", left, right),
            FormulaF::LetterAtPos(var, letter) => {
                format!("(= (word {}) {}) ", var, letter.to_smtlib())
            }
            FormulaF::EqualConstant(var, value) => format!("(= {} {}) ", var, value.to_smtlib()),
        }
    }

    /// Convert a formula to the MONA format.
    ///
    /// assert (L inter W = empty);
    /// assert (all1 x: (x in L) | (x in W));
    fn to_mona(&self) -> String {
        match self {
            FormulaF::True => "true".to_string(),
            FormulaF::False => "false".to_string(),
            FormulaF::Exists(var, sort, inner) => {
                let sort_str = match sort {
                    Sort::Label => "L",
                    Sort::Position => "W",
                };
                format!("ex1 {var}: ({var} in {sort_str}) & ({inner})")
            }
            FormulaF::Forall(var, sort, inner) => {
                let sort_str = match sort {
                    Sort::Label => "L",
                    Sort::Position => "W",
                };
                format!("all1 {var}: ({var} in {sort_str}) => ({inner})")
            }
            FormulaF::And(left, right) => format!("({} & {})", left, right),
            FormulaF::Or(left, right) => format!("({} | {})", left, right),
            FormulaF::Implies(left, right) => format!("({} => {})", left, right),
            FormulaF::Iff(left, right) => format!("({} <=> {})", left, right),
            FormulaF::Not(inner) => format!("~({})", inner),
            FormulaF::Equal(_, left, right) => format!("{} = {}", left, right),
            FormulaF::LessEqual(left, right) => format!("{} <= {}", left, right),
            FormulaF::LetterAtPos(var, letter) => {
                let letter = letter.to_smtlib();
                format!("{var} in L{letter}")
            }
            FormulaF::EqualConstant(var, value) => format!("{} in D{}", var, value.to_mona()),
        }
    }
}

pub fn map_formula<A, F, S, H, G>(formula: &FormulaF<F, A, S>, f: G) -> FormulaF<H, A, S>
where
    S: Clone,
    A: Clone,
    G: Fn(&F) -> H,
{
    match formula {
        FormulaF::True => FormulaF::True,
        FormulaF::False => FormulaF::False,
        FormulaF::Exists(var, sort, inner) => {
            let new_inner = f(inner);
            FormulaF::Exists(var.clone(), sort.clone(), new_inner)
        }
        FormulaF::Forall(var, sort, inner) => {
            let new_inner = f(inner);
            FormulaF::Forall(var.clone(), sort.clone(), new_inner)
        }
        FormulaF::And(left, right) => {
            let new_left = f(left);
            let new_right = f(right);
            FormulaF::And(new_left, new_right)
        }
        FormulaF::Or(left, right) => {
            let new_left = f(left);
            let new_right = f(right);
            FormulaF::Or(new_left, new_right)
        }
        FormulaF::Implies(left, right) => {
            let new_left = f(left);
            let new_right = f(right);
            FormulaF::Implies(new_left, new_right)
        }
        FormulaF::Iff(left, right) => {
            let new_left = f(left);
            let new_right = f(right);
            FormulaF::Iff(new_left, new_right)
        }
        FormulaF::Not(inner) => {
            let new_inner = f(inner);
            FormulaF::Not(new_inner)
        }
        FormulaF::Equal(sort, left, right) => {
            FormulaF::Equal(sort.clone(), left.clone(), right.clone())
        }
        FormulaF::LessEqual(left, right) => FormulaF::LessEqual(left.clone(), right.clone()),
        FormulaF::LetterAtPos(var, letter) => FormulaF::LetterAtPos(var.clone(), letter.clone()),
        FormulaF::EqualConstant(var, sort) => FormulaF::EqualConstant(var.clone(), sort.clone()),
    }
}

#[derive(Debug)]
pub struct FormulaR<A, S> {
    inside: FormulaF<Box<FormulaR<A, S>>, A, S>,
}

pub fn fold_formula<A, S, T, F>(formula: &FormulaR<A, S>, f: &F) -> T
where
    A: Clone,
    S: Clone,
    F: Fn(&FormulaF<T, A, S>) -> T,
{
    let inner = &formula.inside;
    let induction = map_formula(inner, |inner| fold_formula(inner, f));
    f(&induction)
}

impl<A, S> ToSmtSolver for FormulaR<A, S>
where
    A: ToSmtSolver + Clone,
    S: ToSmtSolver + Clone,
{
    fn to_alt_ergo(&self) -> String {
        fold_formula(self, &|formula| formula.to_alt_ergo())
    }

    fn to_smtlib(&self) -> String {
        fold_formula(self, &|formula| formula.to_smtlib())
    }

    fn to_mona(&self) -> String {
        fold_formula(self, &|formula| formula.to_mona())
    }
}

impl<A, S> FormulaR<A, S> {
    fn and(self, other: FormulaR<A, S>) -> FormulaR<A, S> {
        FormulaR {
            inside: FormulaF::And(Box::new(self), Box::new(other)),
        }
    }

    fn or(self, other: FormulaR<A, S>) -> FormulaR<A, S> {
        FormulaR {
            inside: FormulaF::Or(Box::new(self), Box::new(other)),
        }
    }

    fn implies(self, other: FormulaR<A, S>) -> FormulaR<A, S> {
        FormulaR {
            inside: FormulaF::Implies(Box::new(self), Box::new(other)),
        }
    }

    fn iff(self, other: FormulaR<A, S>) -> FormulaR<A, S> {
        FormulaR {
            inside: FormulaF::Iff(Box::new(self), Box::new(other)),
        }
    }

    fn not(self) -> FormulaR<A, S> {
        FormulaR {
            inside: FormulaF::Not(Box::new(self)),
        }
    }

    fn exists(self, var: VarName, sort: Sort) -> FormulaR<A, S> {
        FormulaR {
            inside: FormulaF::Exists(var, sort, Box::new(self)),
        }
    }

    fn forall(self, var: VarName, sort: Sort) -> FormulaR<A, S> {
        FormulaR {
            inside: FormulaF::Forall(var, sort, Box::new(self)),
        }
    }

    fn equal(sort: Sort, left: VarName, right: VarName) -> FormulaR<A, S> {
        FormulaR {
            inside: FormulaF::Equal(sort, left, right),
        }
    }

    fn less_equal(left: VarName, right: VarName) -> FormulaR<A, S> {
        FormulaR {
            inside: FormulaF::LessEqual(left, right),
        }
    }

    fn letter_at_pos(var: VarName, letter: A) -> FormulaR<A, S> {
        FormulaR {
            inside: FormulaF::LetterAtPos(var, letter),
        }
    }

    fn equal_constant(var: VarName, value: S) -> FormulaR<A, S> {
        FormulaR {
            inside: FormulaF::EqualConstant(var, value),
        }
    }

    fn const_true() -> FormulaR<A, S> {
        FormulaR {
            inside: FormulaF::True,
        }
    }

    fn const_false() -> FormulaR<A, S> {
        FormulaR {
            inside: FormulaF::False,
        }
    }
}

/// The alt-ergo printer. We first create a preamble of the following form:
///
/// (* create the various labels *)
/// type label    = D0 | ... | Dn (number of labels)
/// (* create the various letters *)
/// type letter   = E | La | Lb | ... (number of letters)
///
/// (* encode a finite word *)
/// logic len  : int
/// logic word : int -> letter
///
/// (* predicates for every letter in the word *)
/// predicate a(i:int) = word(i) = a
/// predicate b(i:int) = word(i) = b
/// ...
///
/// (* the word has size len *)
/// axiom word_is_finite: forall i:int.
///     (i <= len -> word(i) <> E) and
///     (i > len  -> word(i) = E)  and
///     (i < 0    -> word(i) = E)
///
/// (* The final goal *)
/// goal alt_ergo_goal: { the formula here }
///
pub fn produce_alt_ergo<A, S>(formula: &FormulaR<A, S>, alphabet: &[A], labels: &[S]) -> String
where
    A: ToSmtSolver + Clone,
    S: ToSmtSolver + Clone,
{
    let mut buf = String::new();
    buf.push_str("\n(* create the various labels *)\n");
    buf.push_str("type label    =  ");
    for (i, name) in labels.iter().enumerate() {
        if i > 0 {
            buf.push_str(" | ");
        }
        buf.push_str(name.to_alt_ergo().as_str());
    }

    buf.push_str("\n(* create the various letters *)\n");
    buf.push_str("type letter   = E | ");
    for (i, name) in alphabet.iter().enumerate() {
        if i > 0 {
            buf.push_str(" | ");
        }
        buf.push_str("L");
        buf.push_str(name.to_alt_ergo().as_str());
    }
    buf.push_str("\n");
    buf.push_str("\n");
    buf.push_str("(* encode a finite word *)\n");
    buf.push_str("logic len  : int\n");
    buf.push_str("logic word : int -> letter\n");
    buf.push_str("\n");
    buf.push_str("(* predicates for every letter in the word *)\n");
    for name in alphabet.iter() {
        buf.push_str("predicate is_letter_");
        buf.push_str(name.to_alt_ergo().as_str());
        buf.push_str("(i:int) = word(i) = L");
        buf.push_str(name.to_alt_ergo().as_str());
        buf.push_str("\n");
    }
    buf.push_str("\n");
    buf.push_str("(* the word has size len *)\n");
    buf.push_str("axiom word_is_finite: forall i:int.\n");
    buf.push_str("    (i <= len -> word(i) <> E) and\n");
    buf.push_str("    (i > len  -> word(i) = E)  and\n");
    buf.push_str("    (i < 0    -> word(i) = E)\n");
    buf.push_str("\n");
    buf.push_str("(* The final goal *)\n");
    buf.push_str("goal alt_ergo_goal: ");
    buf.push_str(formula.to_alt_ergo().as_str());
    buf.push_str("\n");
    buf
}

/// The SMT-LIB printer. We first create a preamble of the following form:
///
/// ; Set the logic to be first-order logic with linear arithmetics
/// ; and arrays
/// (set-logic ALL)
/// ; Declare the alphabet
/// (declare-datatype Letter (letter_a letter_b letter_c blank))
/// ; Declare the labels (for the print statements)
/// (declare-datatype Label  (label_a label_b label_c))
/// (declare-const len Int)
/// (declare-const word (Array Int Letter))
/// (assert (>= len 0))
/// ; assert that the word is of length len >= 0
/// ; note that there is no "array-length" function in SMT-LIB
/// ; so we have to assert that the length is equal to len
/// (assert (forall ((x Int)) (= (select word x) (ite (<= x len) (select word x) blank))))
///
/// (assert (>= len 0))
pub fn produce_smtlib<A, S>(formula: &FormulaR<A, S>, alphabet: &[A], labels: &[S]) -> String
where
    A: ToSmtSolver + Clone,
    S: ToSmtSolver + Clone,
{
    let mut buf = String::new();
    buf.push_str("; Set the logic to be first-order logic with linear arithmetics\n");
    buf.push_str("; and arrays\n");
    buf.push_str("(set-logic ALL)\n");
    buf.push_str("; Declare the alphabet \n");
    buf.push_str("(declare-datatype Letter ((blank) ");
    for (i, name) in alphabet.iter().enumerate() {
        if i > 0 {
            buf.push_str(" ");
        }
        buf.push_str("(");
        buf.push_str(name.to_smtlib().as_str());
        buf.push_str(")");
    }
    buf.push_str("))\n");
    buf.push_str("; Declare the labels (for the print statements)\n");
    buf.push_str("(declare-datatype Label  (");
    for (i, name) in labels.iter().enumerate() {
        if i > 0 {
            buf.push_str(" ");
        }
        buf.push_str("(");
        buf.push_str(name.to_smtlib().as_str());
        buf.push_str(")");
    }
    buf.push_str("))\n");
    buf.push_str("(declare-const len Int)\n");
    buf.push_str("(declare-fun word (Int) Letter)\n");
    buf.push_str("; assert that the word is of length len >= 0\n");
    buf.push_str("(assert (>= len 0))\n");
    buf.push_str("; assert that the word contains only letters between 0 and len\n");
    buf.push_str(
        "(assert (forall ((x Int)) (= (word x) (ite (or (< x 0) (>= x len)) (word x) blank))))\n",
    );
    buf.push_str(
        "(assert (forall ((x Int)) (=> (and (<= 0 x) (< x len)) (not (= (word x) blank)))))\n",
    );

    buf.push_str("; now the formula\n");
    buf.push_str("(assert (not ");
    buf.push_str(formula.to_smtlib().as_str());
    buf.push_str("))\n");

    buf.push_str("; check for satisfiability\n");
    buf.push_str("(check-sat)\n");

    buf
}

/// The MONA printer. We first create a preamble of the following form:
///
/// var2 A,B,C;
/// var2 D0,D1,D2;
/// var2 W,L;
///
/// D0 = {1};
/// D1 = {2};
/// D2 = {3};
/// L = D0 union D1 union D2;
/// W = A union B union C;
///
/// restrict (L inter W = empty);
/// restrict (all1 x: (x in L) | (x in W));
///
/// formula;
pub fn produce_mona<A, S>(formula: &FormulaR<A, S>, alphabet: &[A], labels: &[S]) -> String
where
    A: ToSmtSolver + Clone,
    S: ToSmtSolver + Clone,
{
    let mut buf = String::new();
    buf.push_str("m2l-str;\n");
    // Create the alphabet variables
    buf.push_str("var2 ");
    for (i, name) in alphabet.iter().enumerate() {
        if i > 0 {
            buf.push_str(", ");
        }
        buf.push_str("L");
        buf.push_str(name.to_mona().as_str());
    }
    buf.push_str(";\n");
    // create the label variables
    buf.push_str("var2 ");
    for (i, name) in labels.iter().enumerate() {
        if i > 0 {
            buf.push_str(", ");
        }
        buf.push_str(name.to_mona().as_str());
    }
    buf.push_str(";\n");

    // Create the word and label sets
    buf.push_str("var2 W,L;\n");

    // Assert that the labels D{i} = {i}
    for (i, name) in labels.iter().enumerate() {
        buf.push_str("assert (");
        buf.push_str(name.to_mona().as_str());
        buf.push_str(" = {");
        buf.push_str((i).to_string().as_str());
        buf.push_str("});\n");
    }

    // Assert that D = the unions of the Dname
    buf.push_str("assert (L = ");
    for (i, name) in labels.iter().enumerate() {
        if i > 0 {
            buf.push_str(" union ");
        }
        buf.push_str(name.to_mona().as_str());
    }
    buf.push_str(");\n");

    // Assert that W = the unions of the Lname
    buf.push_str("assert (W = ");
    for (i, name) in alphabet.iter().enumerate() {
        if i > 0 {
            buf.push_str(" union ");
        }
        buf.push_str("L");
        buf.push_str(name.to_mona().as_str());
    }
    buf.push_str(");\n");

    // Assert that L inter W = empty
    buf.push_str("assert (L inter W = empty);\n");
    // Assert that all x are either in L or in W
    // buf.push_str("assert (all1 x: (x in L) | (x in W));\n");

    // The formula
    buf.push_str(formula.to_mona().as_str());
    buf.push_str(";\n");

    buf
}

pub fn test_output() -> (
    FormulaR<DefaultAlphabet, DefaultLabel>,
    Vec<DefaultAlphabet>,
    Vec<DefaultLabel>,
) {
    // The first formula is
    // the word is not empty and for every a there is a b afterwards
    let non_empty = FormulaR::equal(Sort::Position, "i".to_string(), "i".to_string())
        .exists("i".to_string(), Sort::Position);

    let no_last_a = FormulaR::less_equal("i".to_string(), "j".to_string())
        .and(FormulaR::letter_at_pos("j".to_string(), DefaultAlphabet::B))
        .exists("j".to_string(), Sort::Position)
        .forall("i".to_string(), Sort::Position);

    let last_b = FormulaR::less_equal("j".to_string(), "i".to_string())
        .forall("j".to_string(), Sort::Position)
        .and(FormulaR::letter_at_pos("i".to_string(), DefaultAlphabet::B))
        .exists("i".to_string(), Sort::Position);

    let formula = non_empty.and(no_last_a).iff(last_b);

    let alphabet = vec![DefaultAlphabet::A, DefaultAlphabet::B, DefaultAlphabet::C];
    let labels = vec![
        DefaultLabel {
            position: vec![0, 0, 1],
        },
        DefaultLabel {
            position: vec![0, 1, 0],
        },
    ];

    (formula, alphabet, labels)
}

#[derive(Debug)]
pub enum SMTSolver {
    AltErgo,
    CVC5,
    Mona,
    Z3,
}

#[derive(Debug, Clone)]
pub enum SMTResult {
    Valid,
    Unknown,
    Invalid,
}

impl SMTSolver {
    pub fn produce_output<A, S>(
        &self,
        formula: &FormulaR<A, S>,
        alphabet: &[A],
        labels: &[S],
    ) -> String
    where
        A: ToSmtSolver + Clone,
        S: ToSmtSolver + Clone,
    {
        match self {
            SMTSolver::AltErgo => produce_alt_ergo(formula, alphabet, labels),
            SMTSolver::CVC5 => produce_smtlib(formula, alphabet, labels),
            SMTSolver::Mona => produce_mona(formula, alphabet, labels),
            SMTSolver::Z3 => produce_smtlib(formula, alphabet, labels),
        }
    }

    pub fn extension(&self) -> &str {
        match self {
            SMTSolver::AltErgo => "ae",
            SMTSolver::CVC5 => "smt2",
            SMTSolver::Mona => "mona",
            SMTSolver::Z3 => "smt2",
        }
    }

    pub fn command(&self) -> &str {
        match self {
            SMTSolver::AltErgo => "alt-ergo",
            SMTSolver::CVC5 => "cvc5",
            SMTSolver::Mona => "mona",
            SMTSolver::Z3 => "z3",
        }
    }

    pub fn parse_output(&self, output: &str) -> SMTResult {
        eprintln!("Output: {}", output);
        match self {
            SMTSolver::AltErgo => {
                if output.contains("Valid") {
                    SMTResult::Valid
                } else if output.contains("Unknown") {
                    SMTResult::Unknown
                } else {
                    SMTResult::Invalid
                }
            }
            SMTSolver::CVC5 => {
                if output.contains("unsat") {
                    SMTResult::Valid
                } else if output.contains("unknown") {
                    SMTResult::Unknown
                } else {
                    SMTResult::Invalid
                }
            }
            SMTSolver::Mona => {
                if output.contains("Formula is valid") {
                    SMTResult::Valid
                } else if output.contains("Formula is unsatisfiable") {
                    SMTResult::Invalid
                } else {
                    SMTResult::Unknown
                }
            }
            SMTSolver::Z3 => {
                if output.contains("unsat") {
                    SMTResult::Valid
                } else if output.contains("unknown") {
                    SMTResult::Unknown
                } else {
                    SMTResult::Invalid
                }
            }
        }
    }

    pub fn solve<A, S>(&self, formula: &FormulaR<A, S>, alphabet: &[A], labels: &[S]) -> SMTResult
    where
        A: ToSmtSolver + Clone,
        S: ToSmtSolver + Clone,
    {
        // create a temporary file and produce the formula
        let tmp_dir = tempdir().unwrap();
        let file_path = tmp_dir
            .path()
            .join("formula_output")
            .with_extension(self.extension());
        let mut file = File::create(&file_path).unwrap();

        // get the path of the file
        let output = self.produce_output(formula, alphabet, labels);
        file.write_all(output.as_bytes()).unwrap();

        // call the correct solver
        eprintln!("Path: {:?}", &file_path);

        let output = std::process::Command::new(self.command())
            .arg(file_path)
            .output()
            .expect(format!("failed to execute process {}", self.command()).as_str());

        let utf8output = String::from_utf8(output.stdout).unwrap();

        self.parse_output(&utf8output)
    }
}

pub fn example() {
    let (f, alph, lbls) = test_output();
    let cvc5 = SMTSolver::CVC5.solve(&f, &alph, &lbls);
    let alt_ergo = SMTSolver::AltErgo.solve(&f, &alph, &lbls);
    let z3 = SMTSolver::Z3.solve(&f, &alph, &lbls);
    let mona = SMTSolver::Mona.solve(&f, &alph, &lbls);

    println!("CVC5: {:?}", cvc5);
    println!("AltErgo: {:?}", alt_ergo);
    println!("Z3: {:?}", z3);
    println!("Mona: {:?}", mona);
}
