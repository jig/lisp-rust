// Example of using MAL without readline
// This is suitable for embedded environments or simple use cases

use std::io::{self, Write};

/// Simple readline implementation using standard input
/// This doesn't have history or line editing features
pub fn simple_readline(prompt: &str) -> Option<String> {
    print!("{}", prompt);
    io::stdout().flush().ok()?;

    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Ok(0) => None, // EOF
        Ok(_) => {
            // Remove trailing newline
            while line.ends_with('\n') || line.ends_with('\r') {
                line.pop();
            }
            Some(line)
        }
        Err(_) => None,
    }
}

fn main() {
    use mal::{initialize_mal_env, mal_env_with_readline, rep};

    // Create environment with simple readline implementation
    let env = mal_env_with_readline(Some(simple_readline));
    initialize_mal_env(&env, vec![]);

    println!("MAL REPL (simple readline)");
    println!("Press Ctrl+D to exit");

    // REPL loop
    loop {
        match simple_readline("user> ") {
            Some(line) => {
                if line.is_empty() {
                    continue;
                }
                match rep(&line, &env) {
                    Ok(out) => println!("{}", out),
                    Err(e) => println!("Error: {}", e.pr_str(true)),
                }
            }
            None => {
                println!();
                break;
            }
        }
    }
}
