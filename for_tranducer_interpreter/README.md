## Proposed Interpreter Structure
`lexer.rs` shall take in as input the for-transducer and tokenize it into relevant parts
`parser.rs` takes the tokenized input and creates an Abstract Syntax Tree (AST) out of the tokens to help us simulate the run better
`ast.rs` is a file specifying contents and tructure of the AST
`interpreter.rs` runs the given input string on the given AST
`main.rs` joins all these files and executes them one by one

## Current Progress
The current code is not exactly the one we want to create, but a template code to modify and adjust to suit our purposes. The current template code is a python-like interpreter which needs a few modifications
