// Example template for embedded environments (Raspberry Pi Pico, ESP32, etc.)
//
// IMPORTANT: This example uses std for demonstration purposes on development machines.
// When deploying to actual embedded hardware, you would:
//
// 1. Add #![no_std] and #![no_main] at the top
// 2. Use a platform-specific global allocator:
//    #[global_allocator]
//    static ALLOCATOR: embedded_alloc::Heap = embedded_alloc::Heap::empty();
//
// 3. Implement panic handler for your platform:
//    #[panic_handler]
//    fn panic(info: &PanicInfo) -> ! {
//        // Log to UART, flash LED, reset device, etc.
//        loop {}
//    }
//
// 4. Use platform-specific entry point (e.g., #[entry] from cortex-m-rt)
// 5. Replace println! with UART/serial output
// 6. Add Cargo.toml target configuration for your embedded platform
//
// See documentation for examples of real embedded deployments.

static mut GLOBAL_ENV: Option<&'static mal::Env> = None;

use mal::types::{MalArgs, MalRet, error, func};

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

    println!("MAL Embedded Template");
    println!("Library is no_std compatible - ready for embedded deployment\n");

    // Initialize MAL environment
    let env = mal_env();
    initialize_mal_env(&env, vec![]);

    // env_sets(&env, "time-ns", func(time_ns));

    // Leak env to get a 'static reference, then set eval
    let env_static: &'static mal::Env = Box::leak(Box::new(env.clone()));
    unsafe { GLOBAL_ENV = Some(env_static); }
    env_sets(&env, "eval", func(eval_wrapper));

    // Example expressions to evaluate
    // In embedded systems, these might come from:
    // - Flash memory configuration
    // - Sensor data formatted as MAL expressions
    // - Network/serial commands
    let expressions = vec![
        "(f* 1.4142135 1.4142135)",
        "(f* 3.14 2)",
        "(f/ 1 3)",
        "(/ 1 3)",
        "(f/ 1 0)",
        "(f/ 1 0.0)",
        "(/ 1 0)",
        "(+ 1 2)",
        "(def! x 42)",
        "(* x 2)",
        "(list 1 2 3)",
        "(eval (read-string \"(+ 10 20)\"))",
    ];

    for expr in expressions {
        println!("> {}", expr);
        match rep(expr, &env) {
            Ok(result) => {
                println!("{}\n", result);
                // In embedded: uart.write_str(&result), display.print(&result), etc.
            }
            Err(e) => {
                println!("Error: {}\n", e.pr_str(true));
                // In embedded: log error, set LED, store in buffer, etc.
            }
        }
    }

    println!("Evaluation complete.");
    println!("\nFor embedded deployment:");
    println!("- The MAL library itself is already no_std compatible");
    println!("- Add embedded-specific allocator and runtime");
    println!("- Configure target in Cargo.toml for your hardware");
    println!("- Replace I/O with platform-specific functions");
}
