// Example of using MAL without readline
// This is suitable for embedded environments or simple use cases


use std::io::{self, Write};

#[macro_use(fn_str)]
extern crate mal;
extern crate rustyline;

// use std::cell::RefCell;
use std::sync::OnceLock;

use mal::types::MalVal::{Int,  Str, Nil};
use mal::types::{MalArgs, MalRet, error, func};
use mal::printer::pr_seq;

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

fn slurp(a: MalArgs) -> MalRet {
    if a.len() != 1 {
        return error("read-file expects 1 argument");
    }
    match a[0] {
        Str(ref s) => {
            match std::fs::read_to_string(s).map_err(|_| error("failed to read file")) {
                Err(e) => {e}
                Ok(content) => {Ok(Str(content))}
            }
        }
        _ => error("file name must be string"),
    }
}

static BOOT_TIME: OnceLock<std::time::Instant> = OnceLock::new();

fn time_ns(a: MalArgs) -> MalRet {
    if a.len() != 0 {
        return error("time/ns/us/ms/s expect 0 arguments");
    }
    let boot = BOOT_TIME.get().unwrap();
    let elapsed = boot.elapsed();
    let ns = elapsed.as_secs() as i64 * 1_000_000_000 + elapsed.subsec_nanos() as i64;
    Ok(Int(ns))
}

fn time_us(a: MalArgs) -> MalRet {
    let Int(ns) = time_ns(a)? else { unreachable!() };
    Ok(Int(ns / 1_000i64))
}

fn time_ms(a: MalArgs) -> MalRet {
    let Int(ns) = time_ns(a)? else { unreachable!() };
    Ok(Int(ns / 1_000_000i64))
}

fn time_s(a: MalArgs) -> MalRet {
    let Int(ns) = time_ns(a)? else { unreachable!() };
    Ok(Int(ns / 1_000_000_000i64))
}

fn readline(p: &str) -> MalRet {
    match simple_readline(p) {
        Some(s) => Ok(Str(s)),
        None => Ok(Nil),
    }
}

// SAFETY: This is safe because:
// 1. This is a single-threaded application (no concurrent access)
// 2. GLOBAL_ENV is only written once during initialization in main()
// 3. After initialization, it's only read (immutable access)
// 4. We need a global variable because func() only accepts function pointers
//    without captures, so eval_wrapper cannot capture the environment directly
static mut GLOBAL_ENV: Option<&'static mal::Env> = None;

fn eval_wrapper(a: MalArgs) -> MalRet {
    if a.len() != 1 {
        return error("eval requires exactly 1 argument");
    }
    // SAFETY: Safe because GLOBAL_ENV is initialized before eval_wrapper is called
    let env = unsafe { GLOBAL_ENV.unwrap() };
    mal::eval(&a[0], env)
}

fn main() {
    use mal::{initialize_mal_env, mal_env, rep, env_sets};

    // Create environment - readline is no longer part of core
    let env = mal_env();
    initialize_mal_env(&env, vec![]);

    env_sets(&env, "slurp", func(slurp));
    env_sets(&env, "time/ns", func(time_ns));
    env_sets(&env, "time/ms", func(time_ms));
    env_sets(&env, "time/us", func(time_us));
    env_sets(&env, "time/s", func(time_s));
    env_sets(&env, "prn", func(|a| {
                println!("{}", pr_seq(&a, true, "", "", " "));
                Ok(Nil)
            }));
    env_sets(&env, "println", func(|a| {
                println!("{}", pr_seq(&a, false, "", "", " "));
                Ok(Nil)
            }),);
    env_sets(&env, "readline", func(fn_str!(readline)));

    // Leak env to get a 'static reference, then set eval
    let env_static: &'static mal::Env = Box::leak(Box::new(env.clone()));
    unsafe { GLOBAL_ENV = Some(env_static); }
    env_sets(&env, "eval", func(eval_wrapper));

    println!("MAL REPL (simple readline)");

    // REPL loop
    loop {
        match simple_readline("> ") {
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
