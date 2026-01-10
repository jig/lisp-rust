use mal::{initialize_mal_env, mal_env, rep};

#[test]
fn reader_unfinished_expr() {
    let env = mal_env();

    match rep("(+ 1", &env) {
        Ok(s) => panic!("Should have returned an error, but got: {}", s),
        Err(e) => {
            let err_msg = e.pr_str(false);
            if err_msg == "INCOMPLETE:expected ')', got EOF" {
                ()
            } else {
                panic!("Unexpected error message: {}", err_msg);
            }
        },
    }
}

#[test]
fn reader_unfinished_string() {
    let env = mal_env();

    match rep("\"hello", &env) {
        Ok(s) => panic!("Should have returned an error, but got: {}", s),
        Err(e) => {
            let err_msg = e.pr_str(false);
            if err_msg == "INCOMPLETE:expected '\"', got EOF" {
                ()
            } else {
                panic!("Unexpected error message: {}", err_msg);
            }
        },
    }
}

#[test]
fn reader_multiple_expressions() {
    let env = mal_env();

    match rep("\"hello\" \"world\"", &env) {
        Ok(s) => {
            // TODO(jig): lisp must return both expressions ("hello" and "world") till no more expressions are available
            // so rep() must work like a reader that returns one expression at a time in stream
            if s == "\"hello\"" {
                ()
            } else {
                panic!("Unexpected result: {}", s);
            }
        },
        Err(e) => {
            if e.pr_str(false) == "unexpected tokens after first expression" {
                ()
            } else {
                panic!("Unexpected error message: {}", e.pr_str(false));
            }
        },
    }
}

#[test]
fn rep_addition() {
    let env = mal_env();
    initialize_mal_env(&env, vec![]);

    match rep("(+ 1 1)", &env) {
        Ok(s) => assert_eq!(s, "2"),
        Err(_) => panic!("rep() returned an error"),
    }
}

#[test]
fn rep_addition_3_args() {
    let env = mal_env();
    initialize_mal_env(&env, vec![]);

    // Should error because + only accepts 2 arguments
    match rep("(+ 1 1 1)", &env) {
        Ok(s) => assert_eq!(s, "3"),
        Err(_) => panic!("rep() returned an error"),
    }
}

#[test]
fn rep_addition_1_arg() {
    let env = mal_env();
    initialize_mal_env(&env, vec![]);

    // Should error because + requires 2 arguments
    match rep("(+ 1)", &env) {
        Ok(s) => assert_eq!(s, "1"),
        Err(_) => panic!("rep() returned an error"),
    }
}

#[test]
fn rep_addition_0_arg() {
    let env = mal_env();
    initialize_mal_env(&env, vec![]);

    // Should error because + requires 2 arguments
    match rep("(+)", &env) {
        Ok(s) => assert_eq!(s, "0"),
        Err(_) => panic!("rep() returned an error"),
    }
}


#[test]
fn rep_divide_by_zero() {
    let env = mal_env();
    initialize_mal_env(&env, vec![]);

    // Should error because division by zero is not allowed
    match rep("(/ 1 0)", &env) {
        Ok(s) => panic!("Should have returned an error, but got: {}", s),
        Err(_) => (), // Expected error
    }
}

#[test]
fn rep_str() {
    let env = mal_env();
    initialize_mal_env(&env, vec![]);

    match rep("(str \"Hello, \" \"world!\")", &env) {
        Ok(s) => assert_eq!(s, "\"Hello, world!\""),
        Err(_) => panic!("rep() returned an error"),
    }
}

#[test]
fn undefined_function() {
    // Test that calling undefined functions results in an error
    let env = mal_env();
    initialize_mal_env(&env, vec![]);

    // readline is not part of core anymore
    match rep("(readline \"prompt> \")", &env) {
        Ok(_) => panic!("readline should not be defined"),
        Err(_) => (), // Expected error - readline is not defined
    }
}

#[test]
fn eval() {
    let env = mal_env();
    initialize_mal_env(&env, vec![]);

    match rep("(def! x 10)", &env) {
        Ok(s) => assert_eq!(s, "10"),
        Err(_) => panic!("rep() returned an error"),
    }

    match rep("(* x 2)", &env) {
        Ok(s) => assert_eq!(s, "20"),
        Err(_) => panic!("rep() returned an error"),
    }
}

#[test]
fn eval_read_string() {
    let env = mal_env();
    initialize_mal_env(&env, vec![]);

    match rep("(read-string \"(+ 1 10)\")", &env) {
        Ok(s) => assert_eq!(s, "(+ 1 10)"),
        Err(_) => panic!("rep() returned an error"),
    }
}

use mal::types::{MalArgs, MalRet, error, func};
use mal::types::MalVal::{Int,  Str, Nil};
use std::sync::OnceLock;
use std::io::{self, Write};

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

#[macro_use(fn_str)]
extern crate mal;

#[test]
fn eval_eval() {
    use mal::{initialize_mal_env, mal_env, rep, env_sets};
    use mal::printer::pr_seq;

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

    match rep("(eval (read-string \"(+ 1 10)\"))", &env) {
        Ok(s) => assert_eq!(s, "11"),
        Err(_) => panic!("rep() returned an error"),
    }
}

#[test]
fn eval_fn() {
    let env = mal_env();
    initialize_mal_env(&env, vec![]);

    match rep("((fn* (a b) (+ a b)) 5 7)", &env) {
        Ok(s) => assert_eq!(s, "12"),
        Err(_) => panic!("rep() returned an error"),
    }

}

#[test]
fn eval_fn_insuficient_args() {
    let env = mal_env();
    initialize_mal_env(&env, vec![]);

    // Should error because function expects 2 arguments but got 0
    match rep("((fn* (a b) (+ a b)))", &env) {
        Ok(s) => panic!("Should have returned an error, but got: {}", s),
        Err(_) => (), // Expected error
    }
}

#[test]
fn eval_fn_too_many_args() {
    let env = mal_env();
    initialize_mal_env(&env, vec![]);

    // Should error because function expects 2 arguments but got 0
    match rep("((fn* (a b) (+ a b)) 1 2 3)", &env) {
        Ok(s) => panic!("Should have returned an error, but got: {}", s),
        Err(_) => (), // Expected error
    }
}