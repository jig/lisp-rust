# Readline Dependency Injection

The MAL library now uses dependency injection for the `readline` functionality, allowing you to provide different implementations depending on your environment.

## Why Dependency Injection?

Instead of having `readline` as an optional feature, the library now allows clients to inject their own `readline` implementation. This provides maximum flexibility:

- **Desktop/Server environments**: Use `rustyline` for full readline features (history, editing, etc.)
- **Embedded environments** (e.g., Raspberry Pi Pico): Use a simple stdin-based implementation
- **Minimal environments**: Use no readline at all for non-interactive use cases
- **Custom implementations**: Provide your own readline implementation for specific needs

## Usage

### Type Definition

The readline function type is defined as:

```rust
pub type ReadlineFn = fn(&str) -> Option<String>;
```

This takes a prompt string and returns `Some(input)` when the user enters a line, or `None` on EOF.

### Creating a MAL Environment

#### Without readline (non-interactive)

```rust
use mal::{mal_env, initialize_mal_env, rep};

let env = mal_env();
initialize_mal_env(&env, vec![]);

// The readline function will not be available
match rep("(+ 1 2)", &env) {
    Ok(result) => println!("{}", result),
    Err(e) => println!("Error: {}", e.pr_str(true)),
}
```

#### With readline

```rust
use mal::{mal_env_with_readline, initialize_mal_env, rep, ReadlineFn};

fn my_readline(prompt: &str) -> Option<String> {
    // Your implementation here
    Some("user input".to_string())
}

let env = mal_env_with_readline(Some(my_readline));
initialize_mal_env(&env, vec![]);

// Now readline is available in MAL code
rep("(readline \"prompt> \")", &env);
```

## Examples

The repository includes three example implementations:

### 1. Rustyline (Full-featured)

File: `examples/rustyline_readline.rs`

Uses the `rustyline` crate for a full-featured REPL with:
- Line editing
- Command history
- History persistence to `.mal-history`

Run with: `cargo run --example rustyline_readline`

### 2. Simple Readline (Minimal)

File: `examples/simple_readline.rs`

Uses standard input/output for a basic readline implementation:
- No line editing
- No history
- Suitable for simple environments

Run with: `cargo run --example simple_readline`

### 3. No Readline (Non-interactive)

File: `examples/no_readline.rs`

Demonstrates using MAL without any readline functionality:
- Evaluates expressions programmatically
- Suitable for embedded systems or scripting

Run with: `cargo run --example no_readline`

## Implementing Your Own Readline

For embedded environments like the Raspberry Pi Pico, you would implement your own readline function:

```rust
// Example for embedded environment
fn embedded_readline(prompt: &str) -> Option<String> {
    // Use your embedded UART or USB serial interface
    // to read user input
    my_embedded_uart_read(prompt)
}

let env = mal_env_with_readline(Some(embedded_readline));
```

## Testing

The test suite includes tests for both modes:

- `test_readline_without_function` - Verifies readline is not available when not provided
- `test_readline_with_function` - Verifies readline works when provided

Run tests with: `cargo test`

## Migration Notes

If you were previously using the library with the `readline` feature:

**Before:**
```rust
// With feature "readline" enabled by default
let env = mal_env();
// readline was always available
```

**After:**
```rust
// Option 1: No readline
let env = mal_env();
// readline is NOT available

// Option 2: With readline
fn my_readline(prompt: &str) -> Option<String> { /* ... */ }
let env = mal_env_with_readline(Some(my_readline));
// readline IS available
```

## Dependencies

The main `mal` library has no dependency on `rustyline`. The `rustyline` crate is now a dev-dependency, only used in the examples.

This makes the library suitable for:
- `no_std` environments (with appropriate modifications)
- Embedded systems
- WebAssembly
- Any environment where `rustyline` cannot be used
