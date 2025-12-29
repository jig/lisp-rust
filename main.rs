#![allow(non_snake_case)]

use mal::{env_sets, initialize_mal_env, mal_env, re, rep, types, Env};

use mal::readline;

thread_local! {
    static REPL_ENV: Env = {
        let repl_env = mal_env();

        // Setup eval with proper REPL_ENV closure
        env_sets(&repl_env, "eval", types::func(|a| {
            REPL_ENV.with(|e| mal::eval(&a[0], e))
        }));

        repl_env
    };
}

fn main() {
    REPL_ENV.with(|repl_env| {
        let mut args = std::env::args();
        let arg1 = args.nth(1);

        // Collect remaining args for *ARGV*
        let argv: Vec<String> = args.collect();

        // Initialize the MAL environment
        initialize_mal_env(repl_env, argv);

        if let Some(f) = arg1 {
            // Invoked with arguments - load and run file
            re(&format!("(load-file \"{}\")", f), repl_env);
            std::process::exit(0);
        }

        // main repl loop
        re("(println (str \"Mal [\" *host-language* \"]\"))", repl_env);
        while let Some(ref line) = readline::readline("user> ") {
            if !line.is_empty() {
                match rep(line, repl_env) {
                    Ok(ref out) => println!("{}", out),
                    Err(ref e) => println!("Error: {}", mal::print(e)),
                }
            }
        }
        println!();
    });
}
