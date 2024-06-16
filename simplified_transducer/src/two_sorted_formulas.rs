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
use std::fmt::{Display, Formatter};
use crate::formula::{Formula as MonoFormula, Interpretation as MonoInterpretation};


type Letter = usize;

/// Let us define an alphabet
/// as a struct so that we can
/// use it.
#[derive(Debug)]
pub struct Alphabet {
    /// Names of every letter for display purposes
    pub names: Vec<String>,
}

impl Alphabet {
    /// Create a new alphabet
    pub fn new() -> Self {
        Alphabet {
            names: vec![],
        }
    }

    /// Add a new letter with a given name
    pub fn add_letter(&mut self, name: String) {
        self.names.push(name);
    }
}

impl Default for Alphabet {
    fn default() -> Self {
        Alphabet {
            names: vec!["a".to_string(), "b".to_string()],
        }
    }
}

impl Display for Alphabet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}}}", self.names.join(", "))
    }
}

/// The first step is to define what is a finite
/// sort L with arities.
#[derive(Debug)]
pub struct Labels {
    /// Names of every variable for display purposes
    pub names: Vec<String>,
    /// Arity of every variable
    pub arity: Vec<usize>,
}

impl Display for Labels {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for (name, arity) in self.names.iter().zip(self.arity.iter()) {
            write!(f, "{}: {}, ", name, arity)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl Labels {
    /// Create a new set of labels
    pub fn new() -> Self {
        Labels {
            names: vec![],
            arity: vec![],
        }
    }

    /// Add a new label with a given name and arity
    pub fn add_label(&mut self, name: String, arity: usize) {
        self.names.push(name);
        self.arity.push(arity);
    }
}

impl Default for Labels {
    fn default() -> Self {
        Labels {
            names: vec!["l1".to_string(), "l2".to_string(), "l3".to_string()],
            arity: vec![3, 2, 1],
        }
    }
}

/// The second step is to define the formulas
/// over a finite set of labels.
type LabelName = usize;
type FormulaId = u16;
type VarName = String;

#[derive(Debug)]
pub enum FormulaExpr {
    /// basic formulas
    True,
    False,
    /// Logical connectives
    And(FormulaId, FormulaId),
    Or(FormulaId, FormulaId),
    Implies(FormulaId, FormulaId),
    Iff(FormulaId, FormulaId),
    Not(FormulaId),
    /// Quantifiers over labels
    ExistsLabel(VarName, FormulaId),
    ForallLabel(VarName, FormulaId),
    /// Quantifiers over positions
    ExistsPos(VarName, FormulaId),
    ForallPos(VarName, FormulaId),
    /// Positional relations
    EqualPos(VarName, VarName),
    LessEqualPos(VarName, VarName),
    LetterAtPos(VarName, Letter),
    /// Label relations
    EqualLabel(VarName, LabelName),
}

pub struct Formula {
    pub alphabet: Alphabet,
    pub labels: Labels,
    pub exprs: Vec<FormulaExpr>,
    pub root: FormulaId,
}

impl Formula {
    pub fn new(alphabet: Alphabet, labels: Labels) -> Self {
        Formula {
            alphabet,
            labels,
            exprs: vec![],
            root: 0,
        }
    }

    pub fn add_expr(&mut self, expr: FormulaExpr) -> FormulaId {
        self.exprs.push(expr);
        (self.exprs.len() - 1) as FormulaId
    }

    pub fn letter_name(&self, letter: Letter) -> &str {
        &self.alphabet.names[letter]
    }

    pub fn label_name(&self, label: LabelName) -> &str {
        &self.labels.names[label]
    }

    pub fn print_formula(&self, buf: &mut String) -> () {
        self.print_formula_rec(self.root, buf)
    }

    fn print_formula_rec(&self, fid: FormulaId, buf: &mut String) -> () {
        match &self.exprs[fid as usize] {
            FormulaExpr::True => buf.push_str("T"),
            FormulaExpr::False => buf.push_str("F"),
            FormulaExpr::And(left, right) => {
                buf.push_str("(");
                self.print_formula_rec(*left, buf);
                buf.push_str(" ∧ ");
                self.print_formula_rec(*right, buf);
                buf.push_str(")");
            }
            FormulaExpr::Or(left, right) => {
                buf.push_str("(");
                self.print_formula_rec(*left, buf);
                buf.push_str(" ∨ ");
                self.print_formula_rec(*right, buf);
                buf.push_str(")");
            }
            FormulaExpr::Implies(left, right) => {
                buf.push_str("(");
                self.print_formula_rec(*left, buf);
                buf.push_str(" → ");
                self.print_formula_rec(*right, buf);
                buf.push_str(")");
            }
            FormulaExpr::Iff(left, right) => {
                buf.push_str("(");
                self.print_formula_rec(*left, buf);
                buf.push_str(" ↔ ");
                self.print_formula_rec(*right, buf);
                buf.push_str(")");
            }
            FormulaExpr::Not(inner) => {
                buf.push_str("¬");
                self.print_formula_rec(*inner, buf);
            }
            FormulaExpr::ExistsLabel(var, inner) => {
                buf.push_str("∃");
                buf.push_str(var);
                buf.push_str(":L. ");
                self.print_formula_rec(*inner, buf);
            }
            FormulaExpr::ForallLabel(var, inner) => {
                buf.push_str("∀");
                buf.push_str(var);
                buf.push_str(":L. ");
                self.print_formula_rec(*inner, buf);
            }
            FormulaExpr::ExistsPos(var, inner) => {
                buf.push_str("∃");
                buf.push_str(var);
                buf.push_str(":P. ");
                self.print_formula_rec(*inner, buf);
            }
            FormulaExpr::ForallPos(var, inner) => {
                buf.push_str("∀");
                buf.push_str(var);
                buf.push_str(":P. ");
                self.print_formula_rec(*inner, buf);
            }
            FormulaExpr::EqualPos(left, right) => {
                buf.push_str(left);
                buf.push_str("=");
                buf.push_str(right);
            }
            FormulaExpr::LessEqualPos(left, right) => {
                buf.push_str(left);
                buf.push_str("<=");
                buf.push_str(right);
            }
            FormulaExpr::EqualLabel(var, label) => {
                buf.push_str(var);
                buf.push_str("=");
                buf.push_str(self.label_name(*label));
            }
            FormulaExpr::LetterAtPos(var, letter) => {
                buf.push_str(var);
                buf.push_str("=");
                buf.push_str(self.letter_name(*letter));
            }
        }
    }

    pub fn write_to_alt_ergo(&self, buf: &mut String) {
        self.write_to_alt_ergo_rec(self.root, buf)
    }

    //
    //  It is almost the same as print_formula_rec,
    //  except that
    //  in alt-ergo, we will quantify as follows
    //  exists x: int. (x >= 0 and x < len) /\ φ
    //  forall x: int. (x >= 0 and x < len) -> φ
    //  forall x: label. φ
    //  exists x: label. φ
    //
    fn write_to_alt_ergo_rec(&self, fid: FormulaId, buf: &mut String) {
        match &self.exprs[fid as usize] {
            FormulaExpr::True => buf.push_str("T"),
            FormulaExpr::False => buf.push_str("F"),
            FormulaExpr::And(left, right) => {
                buf.push_str("(");
                self.write_to_alt_ergo_rec(*left, buf);
                buf.push_str(" and ");
                self.write_to_alt_ergo_rec(*right, buf);
                buf.push_str(")");
            }
            FormulaExpr::Or(left, right) => {
                buf.push_str("(");
                self.write_to_alt_ergo_rec(*left, buf);
                buf.push_str(" or ");
                self.write_to_alt_ergo_rec(*right, buf);
                buf.push_str(")");
            }
            FormulaExpr::Implies(left, right) => {
                buf.push_str("(");
                self.write_to_alt_ergo_rec(*left, buf);
                buf.push_str(" -> ");
                self.write_to_alt_ergo_rec(*right, buf);
                buf.push_str(")");
            }
            FormulaExpr::Iff(left, right) => {
                buf.push_str("(");
                self.write_to_alt_ergo_rec(*left, buf);
                buf.push_str(" <-> ");
                self.write_to_alt_ergo_rec(*right, buf);
                buf.push_str(")");
            }
            FormulaExpr::Not(inner) => {
                buf.push_str("not ");
                self.write_to_alt_ergo_rec(*inner, buf);
            }
            FormulaExpr::ExistsLabel(var, inner) => {
                buf.push_str("(");
                buf.push_str("exists ");
                buf.push_str(var);
                buf.push_str(": label. ");
                self.write_to_alt_ergo_rec(*inner, buf);
                buf.push_str(")");
            }
            FormulaExpr::ForallLabel(var, inner) => {
                buf.push_str("(");
                buf.push_str("forall ");
                buf.push_str(var);
                buf.push_str(": label. ");
                self.write_to_alt_ergo_rec(*inner, buf);
                buf.push_str(")");
            }
            FormulaExpr::ExistsPos(var, inner) => {
                buf.push_str("(");
                buf.push_str("exists ");
                buf.push_str(var);
                buf.push_str(": int. (");
                buf.push_str(var);
                buf.push_str(" >= 0 and ");
                buf.push_str(var);
                buf.push_str(" < len) and \n");
                self.write_to_alt_ergo_rec(*inner, buf);
                buf.push_str(")");
            }
            FormulaExpr::ForallPos(var, inner) => {
                buf.push_str("(");
                buf.push_str("forall ");
                buf.push_str(var);
                buf.push_str(": int. (");
                buf.push_str(var);
                buf.push_str(" >= 0 and ");
                buf.push_str(var);
                buf.push_str(" < len) -> \n");
                self.write_to_alt_ergo_rec(*inner, buf);
                buf.push_str(")");
            }
            FormulaExpr::EqualPos(left, right) => {
                buf.push_str(left);
                buf.push_str("=");
                buf.push_str(right);
            }
            FormulaExpr::LessEqualPos(left, right) => {
                buf.push_str(left);
                buf.push_str("<=");
                buf.push_str(right);
            }
            FormulaExpr::EqualLabel(var, label) => {
                buf.push_str(var);
                buf.push_str(" = D");
                buf.push_str(self.label_name(*label));
            }
            FormulaExpr::LetterAtPos(var, letter) => {
                buf.push_str(var);
                buf.push_str(" = L");
                buf.push_str(self.letter_name(*letter));
            }
        }
    }


    pub fn write_to_smtlib(&self, buf: &mut String) {
        self.write_to_smtlib_rec(self.root, buf)
    }

    //
    // ExistsPos x. φ ->
    // (exists ((x Int)) (and (>= x 0) (< x len) (phi))
    //
    // ForallPos x. φ ->
    // (forall ((x Int)) (=> (and (>= x 0) (< x len)) (phi))
    //
    // ExistsLabel x. φ ->
    // (exists ((x Label)) (phi))
    //
    // ForallLabel x. φ ->
    // (forall ((x Label)) (phi))
    //
    //
    fn write_to_smtlib_rec(&self, fid: FormulaId, buf: &mut String) {
        match &self.exprs[fid as usize] {
            FormulaExpr::True => buf.push_str("true"),
            FormulaExpr::False => buf.push_str("false"),
            FormulaExpr::And(left, right) => {
                buf.push_str("(and ");
                self.write_to_smtlib_rec(*left, buf);
                buf.push_str(" ");
                self.write_to_smtlib_rec(*right, buf);
                buf.push_str(")");
            },
            FormulaExpr::Or(left, right) => {
                buf.push_str("(or ");
                self.write_to_smtlib_rec(*left, buf);
                buf.push_str(" ");
                self.write_to_smtlib_rec(*right, buf);
                buf.push_str(")");
            },
            FormulaExpr::Implies(left, right) => {
                buf.push_str("(=> ");
                self.write_to_smtlib_rec(*left, buf);
                buf.push_str(" ");
                self.write_to_smtlib_rec(*right, buf);
                buf.push_str(")");
            },
            FormulaExpr::Iff(left, right) => {
                buf.push_str("(= ");
                self.write_to_smtlib_rec(*left, buf);
                buf.push_str(" ");
                self.write_to_smtlib_rec(*right, buf);
                buf.push_str(")");
            },
            FormulaExpr::Not(inner) => {
                buf.push_str("(not ");
                self.write_to_smtlib_rec(*inner, buf);
                buf.push_str(")");
            },
            FormulaExpr::ExistsLabel(var, inner) => {
                buf.push_str("(exists (");
                buf.push_str("(");
                buf.push_str(var);
                buf.push_str(" Label)) ");
                self.write_to_smtlib_rec(*inner, buf);
                buf.push_str(")");
            },
            FormulaExpr::ForallLabel(var, inner) => {
                buf.push_str("(forall (");
                buf.push_str("(");
                buf.push_str(var);
                buf.push_str(" Label)) ");
                self.write_to_smtlib_rec(*inner, buf);
                buf.push_str(")");
            },
            FormulaExpr::ExistsPos(var, inner) => {
                buf.push_str("(exists (");
                buf.push_str("(");
                buf.push_str(var);
                buf.push_str(" Int)) ");
                buf.push_str("(and (>= ");
                buf.push_str(var);
                buf.push_str(" 0) (< ");
                buf.push_str(var);
                buf.push_str(" len)) ");
                self.write_to_smtlib_rec(*inner, buf);
                buf.push_str(")");
            },
            FormulaExpr::ForallPos(var, inner) => {
                buf.push_str("(forall (");
                buf.push_str("(");
                buf.push_str(var);
                buf.push_str(" Int)) ");
                buf.push_str("(=> (and (>= ");
                buf.push_str(var);
                buf.push_str(" 0) (< ");
                buf.push_str(var);
                buf.push_str(" len)) ");
                self.write_to_smtlib_rec(*inner, buf);
                buf.push_str(")");
            },
            FormulaExpr::EqualPos(left, right) => {
                buf.push_str("(= ");
                buf.push_str(left);
                buf.push_str(" ");
                buf.push_str(right);
                buf.push_str(")");
            },
            FormulaExpr::LessEqualPos(left, right) => {
                buf.push_str("(<= ");
                buf.push_str(left);
                buf.push_str(" ");
                buf.push_str(right);
                buf.push_str(")");
            },
            FormulaExpr::EqualLabel(var, label) => {
                buf.push_str("(= ");
                buf.push_str(var);
                buf.push_str(" D");
                buf.push_str(self.label_name(*label));
                buf.push_str(")");
            },
            FormulaExpr::LetterAtPos(var, letter) => {
                buf.push_str("(= (select word ");
                buf.push_str(var);
                buf.push_str(") L");
                buf.push_str(self.letter_name(*letter));
                buf.push_str(")");
            },
        }
    }
}

impl Display for Formula {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Alphabet: {}\nLabels: {}\n", self.alphabet, self.labels)?;
        let mut buf = String::new();
        self.print_formula(&mut buf);
        write!(f, "{}", buf)
    }
}

// TODO: print to alt-ergo
//
// (* create the various labels *)
// type label    = D0 | ... | Dn (number of labels)
// (* create the various letters *)
// type letter   = E | La | Lb | ... (number of letters)
//
// (* encode a finite word *)
// logic len  : int
// logic word : int -> letter
//
// (* predicates for every letter in the word *)
// predicate a(i:int) = word(i) = a
// predicate b(i:int) = word(i) = b
// ...
//
// (* the word has size len *)
// axiom word_is_finite: forall i:int.
//     (i <= len -> word(i) <> E) and
//     (i > len  -> word(i) = E)  and
//     (i < 0    -> word(i) = E)
//
// (* The final goal *)
// goal alt_ergo_goal: { the formula here }
//

pub fn write_to_alt_ergo(formula: &Formula) -> String {
    let mut buf = String::new();
    buf.push_str("\n(* create the various labels *)\n");
    buf.push_str("type label    =  ");
    for (i, name) in formula.labels.names.iter().enumerate() {
        if i > 0 {
            buf.push_str(" | ");
        }
        buf.push_str("D");
        buf.push_str(name);
    }

    buf.push_str("\n(* create the various letters *)\n");
    buf.push_str("type letter   = E | ");
    for (i, name) in formula.alphabet.names.iter().enumerate() {
        if i > 0 {
            buf.push_str(" | ");
        }
        buf.push_str("L");
        buf.push_str(name);
    }
    buf.push_str("\n");
    buf.push_str("\n");
    buf.push_str("(* encode a finite word *)\n");
    buf.push_str("logic len  : int\n");
    buf.push_str("logic word : int -> letter\n");
    buf.push_str("\n");
    buf.push_str("(* predicates for every letter in the word *)\n");
    for name in formula.alphabet.names.iter() {
        buf.push_str("predicate ");
        buf.push_str(name);
        buf.push_str("(i:int) = word(i) = L");
        buf.push_str(name);
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
    formula.write_to_alt_ergo(&mut buf);
    buf.push_str("\n");
    buf
}


// Write to SMTLIB
//
// ```smtlib
// ; Set the logic to be first-order logic with linear arithmetics
// ; and arrays
// (set-logic QF_AUFLIA)
// ; Declare the alphabet 
// (declare-datatype Letter (letter_a letter_b letter_c))
// ; Declare the labels (for the print statements)
// (declare-datatype Label  (label_a label_b label_c))
// (declare-const len Int)
// (declare-const word (Array  Int Letter))
// ; assert that the word is of length len >= 0
// (assert (= len (array-length word)))
// (assert (>= len 0))
// ; assert that some property holds for the word
// ; for instance
// ; exists l : label. forall x : Int. 0 <= x <= len -> (l = label_a -> word[x] = letter_a)
// (assert (exists ((l Label)) (forall ((x Int)) (=> (and (<= 0 x) (<= x len)) (=> (= l label_a) (= (select word x) letter_a)))))
// ```
fn write_to_smtlib(formula: &Formula) -> String {
    let mut buf = String::new();
    buf.push_str("; Set the logic to be first-order logic with linear arithmetics\n");
    buf.push_str("; and arrays\n");
    buf.push_str("(set-logic QF_AUFLIA)\n");
    buf.push_str("; Declare the alphabet \n");
    buf.push_str("(declare-datatype Letter (");
    for (i, name) in formula.alphabet.names.iter().enumerate() {
        if i > 0 {
            buf.push_str(" ");
        }
        buf.push_str("L");
        buf.push_str(name);
    }
    buf.push_str("))\n");
    buf.push_str("; Declare the labels (for the print statements)\n");
    buf.push_str("(declare-datatype Label  (");
    for (i, name) in formula.labels.names.iter().enumerate() {
        if i > 0 {
            buf.push_str(" ");
        }
        buf.push_str("D");
        buf.push_str(name);
    }
    buf.push_str("))\n");
    buf.push_str("(declare-const len Int)\n");
    buf.push_str("(declare-const word (Array  Int Letter))\n");
    buf.push_str("; assert that the word is of length len >= 0\n");
    buf.push_str("(assert (= len (array-length word)))\n");
    buf.push_str("(assert (>= len 0))\n");
    buf.push_str("; assert that some property holds for the word\n");
    buf.push_str("; for instance\n");
    formula.write_to_smtlib(&mut buf);
    buf
}


// Write to MONA
// -> do the same thing.
//
pub fn test_altergo() {
    // test the two sorted formula module
    let alphabet = Alphabet::default(); // {a,b}
    println!("Alphabet: {}", alphabet);
    let labels = Labels::default();
    println!("Labels: {}", labels);
    let mut formula = Formula::new(alphabet, labels);
    let a = formula.add_expr(FormulaExpr::EqualLabel("x".into(), 0));
    let b = formula.add_expr(FormulaExpr::EqualLabel("y".into(), 1));
    let c = formula.add_expr(FormulaExpr::And(a, b));
    let d = formula.add_expr(FormulaExpr::ExistsPos("x".into(), c));
    let e = formula.add_expr(FormulaExpr::ExistsPos("y".into(), d));
    formula.root = e;
    eprintln!("Formula: {}", formula);
    eprintln!("Formula: {}\n\n", write_to_alt_ergo(&formula));
    eprintln!("Formula: {}", write_to_smtlib(&formula));
}

pub fn test_formula() {
    let mut alphabet = Alphabet::new();
    alphabet.add_letter("a".to_string());
    alphabet.add_letter("b".to_string());

    let mut labels = Labels::new();
    labels.add_label("l1".to_string(), 3);
    labels.add_label("l2".to_string(), 2);
    labels.add_label("l3".to_string(), 1);

    let mut formula = Formula::new(alphabet, labels);

    let l1 = formula.add_expr(FormulaExpr::EqualLabel("x".to_string(), 0));
    let l2 = formula.add_expr(FormulaExpr::EqualLabel("y".to_string(), 1));
    let l3 = formula.add_expr(FormulaExpr::EqualLabel("z".to_string(), 2));

    let l4 = formula.add_expr(FormulaExpr::And(l1, l2));
    let l5 = formula.add_expr(FormulaExpr::And(l4, l3));

    formula.root = l5;

    let mut buf = String::new();
    formula.print_formula(&mut buf);

    let alt_ergo = write_to_alt_ergo(&formula);
    println!("{}", alt_ergo);
}
