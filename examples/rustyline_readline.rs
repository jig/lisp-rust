// Example readline implementation using rustyline
// This shows how to use the MAL library with a rustyline-based readline function

extern crate rustyline;

use std::cell::RefCell;

struct ReadlineState {
    e: rustyline::Editor<(), rustyline::history::DefaultHistory>,
}

impl Drop for ReadlineState {
    fn drop(&mut self) {
        let _ = self.e.save_history(".mal-history");
    }
}

thread_local! {
    static ED: RefCell<ReadlineState> = {
        let mut e = rustyline::Editor::new().unwrap();
        if e.load_history(".mal-history").is_err() {
            println!("No previous history.");
        }
        RefCell::new(ReadlineState { e })
    }
}

/// Readline implementation using rustyline
pub fn rustyline_readline(prompt: &str) -> Option<String> {
    ED.with_borrow_mut(|s| {
        let r = s.e.readline(prompt);
        if let Err(rustyline::error::ReadlineError::Eof) = r {
            None
        } else {
            let mut line = r.unwrap();
            // Remove any trailing \n or \r\n
            while line.ends_with('\n') || line.ends_with('\r') {
                line.pop();
            }
            if !line.is_empty() {
                let _ = s.e.add_history_entry(&line);
            }
            Some(line.to_string())
        }
    })
}

fn main() {
    use mal::{initialize_mal_env, mal_env_with_readline, rep};

    // Create environment with rustyline readline implementation
    let env = mal_env_with_readline(Some(rustyline_readline));
    initialize_mal_env(&env, vec![]);

    // REPL loop
    loop {
        match rustyline_readline("user> ") {
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
