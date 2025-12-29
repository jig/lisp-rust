# AGENTS.md

This file provides guidance for AI coding agents working on the Rust implementation of MAL (Make-A-Lisp).

## Project Overview

This is a Rust implementation of MAL, a Lisp interpreter written as part of the MAL project. The implementation follows the step-by-step process defined by the MAL guide, building up from a basic REPL to a full Lisp interpreter with features like tail call optimization, macros, and more.

## Project Structure

- `lib.rs` - Main library containing read-eval-print-loop (REPL) logic and core evaluation
- `types.rs` - MAL value types and core data structures
- `reader.rs` - Parser/reader for MAL expressions
- `printer.rs` - Printer for MAL values
- `env.rs` - Environment implementation for variable bindings
- `core.rs` - Core built-in functions (arithmetic, list operations, etc.)
- `readline.rs` - Readline functionality for REPL
- `main.rs` - Entry point for the executable

## Setup Commands

- Install Rust toolchain: `rustup install stable`
- Build the project: `cargo build`
- Build release version: `cargo build --release`
- Run the REPL: `cargo run` or `./target/release/stepA_mal`
- Run tests: `cargo test`

## Testing Instructions

- Unit tests are located at the bottom of `lib.rs` in the `tests` module
- Run all tests: `cargo test`
- Run tests with output: `cargo test -- --nocaptures`
- Run specific test: `cargo test test_name`
- All tests must pass before committing
- Add tests for any new functionality or bug fixes

## Code Style

- Follow Rust standard conventions (rustfmt formatting)
- Use `snake_case` for functions and variables
- Use `CamelCase` for types and enum variants
- Keep functions focused and modular
- Prefer pattern matching over complex conditionals
- Use meaningful variable names, avoid abbreviations except for well-known cases (e.g., `env` for environment)
- **All code, comments, and documentation must be in English**

## Important Conventions

### Error Handling

- Use `MalRet` (alias for `Result<MalVal, MalVal>`) for functions that can fail
- Use the `error()` helper function to create error values
- Errors are MAL values, not Rust panics

### Memory Management

- Use `Rc` (reference counting) for shared ownership of MAL values
- Use `Rc<RefCell<>>` for mutable shared state (e.g., atoms)
- Avoid cloning unless necessary; prefer borrowing

### Macros

- `fn_t_int_int!` - Creates functions that take exactly 2 integer arguments
- `fn_is_type!` - Creates type-checking predicates
- `fn_str!` - Creates functions that take a string argument
- `list!()` - Macro for creating MAL lists

### Testing

- When adding new core functions, validate argument counts
- Test both success and error cases
- Use pattern matching instead of `.unwrap()` in tests (to avoid Debug trait requirements)
- Example test pattern:
  ```rust
  match rep("(+ 1 1)", &env) {
      Ok(s) => assert_eq!(s, "2"),
      Err(_) => panic!("Unexpected error"),
  }
  ```

## Common Tasks

### Adding a New Core Function

1. Add the function implementation in `core.rs`
2. Add the function to the `ns()` vector at the bottom of `core.rs`
3. Add tests in `lib.rs` to verify the function works correctly
4. Ensure proper argument validation

### Fixing Argument Validation Issues

- Most core functions should validate their argument count
- Use early returns with `error()` for invalid inputs
- The `fn_t_int_int!` macro validates exactly 2 integer arguments

### Working with MAL Values

- Always pattern match on `MalVal` variants
- Use helper functions like `list()`, `vector()`, `hash_map()` for construction
- Use `pr_str(true)` to print values with readability enabled

## Security Considerations

- File I/O is limited to the `slurp` function
- No direct system command execution beyond what's in `core.rs`
- User input is parsed through the reader; no `eval()` of arbitrary Rust code

## Known Issues and TODOs

- Check the GitHub issues for the main MAL repository
- This is a reference implementation; focus on clarity over performance
- Some edge cases in error handling may need improvement

## Deployment

This is a reference implementation and not typically deployed as a standalone application. Users typically:
1. Build with `cargo build --release`
2. Run the resulting binary `./target/release/stepA_mal`
3. Or use it as a library by depending on the crate
