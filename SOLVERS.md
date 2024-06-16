# Choices of Solvers

## Alt-Ergo

[Alt-Ergo] is an open-source SMT solver developed by the [Toccata] team at
INRIA. It is used in the [Why3] platform.
Here is the syntax we intend to use to encode the theory of words in Alt-Ergo:

```alt-ergo
(* create the various labels *)
type label    = D0 | ... | Dn (number of labels)
(* create the various letters *)
type letter   = E | La | Lb | ... (number of letters)

(* encode a finite word *)
logic len  : int
logic word : int -> letter

(* predicates for every letter in the word *)
predicate a(i:int) = word(i) = a
predicate b(i:int) = word(i) = b
...

(* the word has size len *)
axiom word_is_finite: forall i:int.
    (i <= len -> word(i) <> E) and
    (i > len  -> word(i) = E)  and
    (i < 0    -> word(i) = E)

(* The final goal *)
goal alt_ergo_goal: { the formula here }
```

[Alt-Ergo]: https://alt-ergo.lri.fr/
[Toccata]: https://toccata.lri.fr/
[Why3]: https://why3.lri.fr/

## SMT-LIB 2

The [SMTLIB] standard is a language for describing problems in the domain of
satisfiability modulo theories. The standard is maintained by the SMT-LIB
consortium. The standard is currently at version 2.6.

Here is an example of how we intend to encode the theory of words
in SMTLIB 2.6:

```smtlib
; Set the logic to be first-order logic with linear arithmetics
; and arrays
(set-logic QF_AUFLIA)
; Declare the alphabet 
(declare-datatype Letter (letter_a letter_b letter_c))
; Declare the labels (for the print statements)
(declare-datatype Label  (label_a label_b label_c))
(declare-const len Int)
(declare-const word (Array  Int Letter))
; assert that the word is of length len >= 0
(assert (= len (array-length word)))
(assert (>= len 0))
; assert that some property holds for the word
; for instance
; exists l : label. forall x : Int. 0 <= x <= len -> (l = label_a -> word[x] = letter_a)
(assert (exists ((l Label)) (forall ((x Int)) (=> (and (<= 0 x) (<= x len)) (=> (= l label_a) (= (select word x) letter_a)))))
```

The advantage of using SMT-LIB 2.6 is that it is a standard that is supported by
many solvers. 

- [CVC4](https://cvc4.github.io/)
- [CVC5](https://cvc5.github.io/)
- [Z3](https://github.com/Z3Prover/z3)
- [Yices](https://yices.csl.sri.com/)

[SMTLIB]: https://smt-lib.org/papers/smt-lib-reference-v2.6-r2021-05-12.pdf


## MONA

[MONA] is a monadic second order logic solver developped by the Aarhus
University. It is used to solve problems in the domain of automata theory.
Because we will essentially be using monadic second order logic (or even,
first-order  logic) on words, this could be a good choice of solver.
Note that MONA does not specifically support multi-sorted logics,
which we do need in our translation. Hence, we will have to encode
the finite type of labels using disjoint letters of the alphabet

```mona

var2 A,B,C;
var2 D0,D1,D2;
var2 W,L;

D0 = {1};
D1 = {2};
D2 = {3};
L = D0 union D1 union D2;
W = A union B union C;

assert (L inter W = empty);
assert (all1 x: (x in L) | (x in W));

formula;
```

However, it seems that MONA is not actively maintained anymore, and has obvious
flaws because according to the documentation 

> Explicit restriction and global assertions to better take advantage of the
> ternary semantics, programmers may use the formula-level construct
> restrict(φ) to turn false into don’t care. (More precisely, if φ evaluates to
> false, restrict(φ) evaluates to don’t care; otherwise, it evaluates to the same
> as φ.) Internally in MONA, all uses of restrictions are reduced to such
> primitive restrict operations. The operation is implemented by converting
> reject states into don’t-care states in the automaton corresponding to φ and
> minimizing the result. 

While we do not use explict `restrict` in the code above, the following
paragraph from the documentation is also concerning:

> Often, the MONA programmer wants to analyze a whole
> program under certain assumptions. For instance, if a program consists of two
> formulas, φ and φ′, one might want to assert that φ holds, so that any
> counter-example that is printed satisfies φ but not φ′. MONA provides a method
> for specifying such assertions. The declaration assert φ; is equivalent to
> restrict(φ); and has the desired effect.

Note that according to this paragraph,
the above code, with "formula = false" should produce a counter-example
that satisfies the assertions. However, it is not the case, as the
output is 

```
MONA v1.4-18 for WS1S/WS2S
Copyright (C) 1997-2016 Aarhus University

PARSING
Time: 00:00:00.00

CODE GENERATION
DAG hits: 44, nodes: 52
Time: 00:00:00.00

REDUCTION
Projections removed: 5 (of 5)
Products removed: 8 (of 8)
Other nodes removed: 1 (of 2)
DAG nodes after reduction: 1
Time: 00:00:00.00

AUTOMATON CONSTRUCTION

                                                                            
100% completed                                                                         
Time: 00:00:00.00

Automaton has 2 states and 1 BDD-node

ANALYSIS
Formula is unsatisfiable

A counter-example of least length (0) is:
A               X 
B               X 
C               X 
D0              X 
D1              X 
D2              X 
W               X 
L               X 

A = {}
B = {}
C = {}
D0 = {}
D1 = {}
D2 = {}
W = {}
L = {}

Total time: 00:00:00.00
```

[MONA]: https://github.com/cs-au-dk/MONA


## Gaston

There is also [Gaston], a solver for the monadic second order logic on finite
words. However it is also not maintained anymore, and has no documentation.
It seems to accept the same syntax as [MONA] based on the example
folder of the github repository.

[Gaston]: https://github.com/tfiedor/gaston

