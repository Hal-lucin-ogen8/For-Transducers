# For Transducer Interpreter

## Overview

This project is an interpreter designed to process a **For_Transducer** with specific features. The interpreter consists of several modules, each responsible for a distinct part of the process, from lexical analysis to execution. Below is a brief description of each component and the current capabilities of the interpreter.

## Project Structure

1. **`lexer.rs`**: 
   - This module is responsible for lexical analysis. It takes the input source code and tokenizes it into relevant parts.
   
2. **`parser.rs`**:
   - The parser module takes the tokenized input from the lexer and constructs an Abstract Syntax Tree (AST) out of the tokens. The AST helps in better simulation and execution of the code.
   
3. **`ast.rs`**:
   - This file defines the structure and contents of the AST. It specifies how the different elements of the language are represented in the tree form.
   
4. **`interpreter.rs`**:
   - The interpreter module takes the AST and runs the given input string on it. This is where the actual execution of the code happens.
   
5. **`main.rs`**:
   - This is the main entry point of the program. It integrates all the modules (`lexer`, `parser`, `ast`, and `interpreter`) and executes them in sequence.

## Current Progress

The interpreter currently supports the following functionalities:

- **For Loop Parsing**:
  - Both forward and reverse for loops can be parsed and processed correctly.
  
- **Execution of If Statements**:
  - The interpreter can execute `if` statements, although this functionality currently does not include handling of labels.

## To be Done
- Taking input word from user
- Processing Arithmetic Operations
- Processing Boolean Variables

## How to Run

1. Ensure you have Rust installed on your system.
2. Clone the repository.
3. Navigate to the project directory.
4. Run the project using Cargo:
   ```sh
   cargo run
