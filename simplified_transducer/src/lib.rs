pub mod ast;
pub mod interpreter;
pub mod lexer;
pub mod parser;
// pub mod two_sorted_formulas;

// pub use interpreter::Interpreter;
pub use lexer::tokenize;
pub use parser::Parser;
