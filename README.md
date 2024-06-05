# For_Transducer Interpreter

## Overview

This project is an interpreter designed to process a **For_Transducer** with specific features. The interpreter consists of several modules, each responsible for a distinct part of the process, from lexical analysis to execution. Below is a brief description of each component and the current capabilities of the interpreter.

## Project Structure

1. **`lexer.rs`**: 
   - Responsible for lexical analysis. It takes the input source code and tokenizes it into relevant parts.
   
2. **`parser.rs`**:
   - Takes the tokenized input from the lexer and constructs an Abstract Syntax Tree (AST) out of the tokens. The AST helps in better simulation and execution of the code.
   
3. **`ast.rs`**:
   - Defines the structure and contents of the AST. It specifies how the different elements of the language are represented in the tree form.
   
4. **`interpreter.rs`**:
   - Takes the AST and runs the given input string on it. This is where the actual execution of the code happens.
   
5. **`main.rs`**:
   - The main entry point of the program. It integrates all the modules (`lexer`, `parser`, `ast`, and `interpreter`) and executes them in sequence.

## Current Progress

The interpreter currently supports the following functionalities:

- **For Loop Parsing**:
  - The loop input can only be in one of the following two forms:
    - `for var_name in 0..n`
    - `for var_name in n..0`
  - In these forms, `var_name` can be any string representing a variable name.
  - The notation `0..n` indicates a forward loop (from first to last), while `n..0` indicates a reverse loop (from last to first).
  - Here, `n` represents the size of the word, which is not known at compile time but is determined after the user provides the input word.
  - Both forward and reverse for loops can be parsed and processed correctly.
  
- **Execution of If Statements**:
  - The interpreter can execute `if` statements with the following restrictions:
    - Comparisons between two variables that are in the hashmap.
    - Comparisons of the type `i.label == "some_char"`.
  - Disallowed comparisons include:
    - Comparisons between two labels, e.g., `i.label == j.label`.
    - Comparisons involving constants and variables, e.g., `i < 3` or `3 < 5`.
  - This ensures that `if` statements are used in a controlled and meaningful manner, avoiding invalid or redundant comparisons.

- **Input string accepted from user at runtime**
   - Input string is specified at runtime through the usage `cargo run <transducer> <input>`.

**To do**
Generate an executable so that tranducer is not compiled everytime multiple string inputs are run (?)

## How to Run

1. Ensure you have Rust installed on your system.
2. Clone the repository:
   ```sh
   git clone <repository-url>
   ```
   or download the zip file and extract it.

3. Navigate to cloned directory:
   ```sh
   cd for_transducer_interpreter
   ```
4. Run the program:
   ```sh
   cargo run <tranducer_file> <input_string>  # file_name is the name of the txt file containing the For_Transducer code
   ```
5. The program will execute and display the output. (Note: the current implementation does not take input from the user. The input word is hardcoded in the code. This will be updated in the future.)
