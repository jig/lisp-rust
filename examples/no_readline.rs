// Example of using MAL without any readline functionality
// This is suitable for embedded environments without interactive input

fn main() {
    use mal::{initialize_mal_env, mal_env, rep};

    // Create environment WITHOUT readline
    let env = mal_env();
    initialize_mal_env(&env, vec![]);

    // Example: evaluate some MAL code programmatically
    let expressions = vec![
        "(+ 1 2)",
        "(def! x 42)",
        "(* x 2)",
        "(list 1 2 3)",
    ];

    println!("MAL - Non-interactive mode");
    println!("No readline functionality\n");

    for expr in expressions {
        println!("> {}", expr);
        match rep(expr, &env) {
            Ok(out) => println!("{}\n", out),
            Err(e) => println!("Error: {}\n", e.pr_str(true)),
        }
    }
}
