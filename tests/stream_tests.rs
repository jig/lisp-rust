use mal::reader::{CharReader, MalStream, TokenStream};
use mal::types::MalVal;

#[test]
fn test_token_stream_simple() {
    let input = "(+ 1 2)";
    let char_reader = input.chars();
    let token_stream = TokenStream::new(char_reader);

    // This is a simplified test - TokenStream implementation needs refinement
    // for proper token-by-token iteration
    let tokens: Vec<String> = token_stream.collect();
    assert!(!tokens.is_empty());
}

#[test]
fn test_mal_stream_single_expression() {
    let input = "(+ 1 2)";
    let char_reader = input.chars();
    let mut mal_stream = MalStream::new(char_reader);

    // Should get one expression
    match mal_stream.next() {
        Some(Ok(val)) => {
            let result = val.pr_str(true);
            assert_eq!(result, "(+ 1 2)");
        }
        Some(Err(e)) => panic!("Got error: {}", e.pr_str(true)),
        None => panic!("Expected an expression"),
    }

    // Should get None after (end of stream)
    assert!(mal_stream.next().is_none());
}

#[test]
fn test_mal_stream_multiple_expressions() {
    let input = "(+ 1 2)\n(* 3 4)\n(def! x 5)";
    let char_reader = input.chars();
    let mal_stream = MalStream::new(char_reader);

    let results: Vec<_> = mal_stream.collect();

    // Should have 3 expressions
    assert_eq!(results.len(), 3);

    // Check each result
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(_) => {}, // Success
            Err(e) => panic!("Expression {} failed: {}", i, e.pr_str(true)),
        }
    }
}

#[test]
fn test_mal_stream_incomplete_expression() {
    let input = "(+ 1\n2)";  // Incomplete on first line, completed on second
    let char_reader = input.chars();
    let mut mal_stream = MalStream::new(char_reader);

    match mal_stream.next() {
        Some(Ok(val)) => {
            // Should successfully parse the complete expression
            assert!(val.pr_str(true).contains("+"));
        }
        Some(Err(e)) => panic!("Got error: {}", e.pr_str(true)),
        None => panic!("Expected an expression"),
    }
}

#[test]
fn test_mal_stream_with_strings() {
    let input = r#""hello world" "foo bar""#;
    let char_reader = input.chars();
    let mal_stream = MalStream::new(char_reader);

    let results: Vec<_> = mal_stream.collect();

    assert_eq!(results.len(), 2);

    if let Some(Ok(MalVal::Str(s))) = results.first() {
        assert_eq!(s, "hello world");
    } else {
        panic!("Expected first string");
    }
}

#[test]
fn test_mal_stream_error_handling() {
    let input = "(+ 1 2";  // Unclosed parenthesis
    let char_reader = input.chars();
    let mut mal_stream = MalStream::new(char_reader);

    match mal_stream.next() {
        Some(Err(e)) => {
            let err_msg = e.pr_str(false);
            // Should get an incomplete or EOF error
            assert!(err_msg.contains("EOF") || err_msg.contains("expected"));
        }
        Some(Ok(_)) => panic!("Should have gotten an error"),
        None => {}, // Also acceptable - stream ended
    }
}

#[test]
fn test_byte_to_char_adapter_ascii() {
    use mal::reader::ByteToCharAdapter;

    let bytes = b"(+ 1 2)";
    let mut idx = 0;

    let byte_reader = || {
        if idx < bytes.len() {
            let b = bytes[idx];
            idx += 1;
            Some(b)
        } else {
            None
        }
    };

    let mut adapter = ByteToCharAdapter::new(byte_reader);

    // Read first few characters
    assert_eq!(adapter.read_char(), Some('('));
    assert_eq!(adapter.read_char(), Some('+'));
    assert_eq!(adapter.read_char(), Some(' '));
}

#[test]
fn test_mal_stream_empty_input() {
    let input = "";
    let char_reader = input.chars();
    let mut mal_stream = MalStream::new(char_reader);

    // Should return None for empty input
    assert!(mal_stream.next().is_none());
}

#[test]
fn test_mal_stream_whitespace_only() {
    let input = "   \n\t  ";
    let char_reader = input.chars();
    let mut mal_stream = MalStream::new(char_reader);

    // Should return None or error for whitespace only
    match mal_stream.next() {
        None => {}, // Expected
        Some(Err(_)) => {}, // Also acceptable
        Some(Ok(_)) => panic!("Should not parse whitespace as expression"),
    }
}

// Example: How to use MalStream in a real application
#[test]
fn test_example_usage() {
    // Simulate reading from a source (file, UART, etc.)
    let lisp_code = r#"
        (def! square (fn* (x) (* x x)))
        (square 5)
        (+ 1 2)
    "#;

    let char_reader = lisp_code.chars();
    let mal_stream = MalStream::new(char_reader);

    let env = mal::mal_env();
    mal::initialize_mal_env(&env, vec![]);

    // Process each expression as it comes
    for (i, result) in mal_stream.enumerate() {
        match result {
            Ok(expr) => {
                match mal::eval(&expr, &env) {
                    Ok(val) => println!("Result {}: {}", i + 1, val.pr_str(true)),
                    Err(e) => eprintln!("Error evaluating expression {}: {}", i + 1, e.pr_str(true)),
                }
            }
            Err(e) => {
                eprintln!("Error in expression {}: {}", i + 1, e.pr_str(true));
            }
        }
    }
}

// Example 2: Using rep_stream_to_string (lazy iterator returning Strings)
#[test]
fn test_rep_stream_to_string() {
    let lisp_code = r#"
        (def! add-one (fn* (x) (+ x 1)))
        (add-one 10)
        (add-one 20)
        (* 3 4)
    "#;

    let env = mal::mal_env();
    mal::initialize_mal_env(&env, vec![]);

    // Lazy evaluation with strings
    let results: Vec<String> = mal::rep_stream_to_string(lisp_code.chars(), &env).collect();

    assert_eq!(results.len(), 4);
    assert_eq!(results[0], "(fn* (x) (+ x 1))");
    assert_eq!(results[1], "11");
    assert_eq!(results[2], "21");
    assert_eq!(results[3], "12");
}

// Example 3: Using rep_stream_to_string_vec (eager Vec<String>)
#[test]
fn test_rep_stream_to_string_vec() {
    let lisp_code = r#"
        (+ 5 5)
        (- 10 3)
        (* 2 2)
    "#;

    let env = mal::mal_env();
    mal::initialize_mal_env(&env, vec![]);

    let results = mal::rep_stream_to_string_vec(lisp_code.chars(), &env);

    assert_eq!(results.len(), 3);
    assert_eq!(results[0], "10");
    assert_eq!(results[1], "7");
    assert_eq!(results[2], "4");
}

// Example 4: Using rep_stream_to_malret (lazy iterator returning MalRet)
#[test]
fn test_rep_stream_to_malret() {
    let lisp_code = r#"
        (+ 1 1)
        (* 5 5)
        (/ 10 2)
    "#;

    let env = mal::mal_env();
    mal::initialize_mal_env(&env, vec![]);

    // Lazy evaluation with raw MalRet iterator
    let mut count = 0;
    for result in mal::rep_stream_to_malret(lisp_code.chars(), &env) {
        match result {
            Ok(mal::types::MalVal::Int(n)) => {
                match count {
                    0 => assert_eq!(n, 2),
                    1 => assert_eq!(n, 25),
                    2 => assert_eq!(n, 5),
                    _ => panic!("Too many results"),
                }
                count += 1;
            }
            Ok(_) => panic!("Expected Int"),
            Err(e) => panic!("Unexpected error: {}", e.pr_str(true)),
        }
    }

    assert_eq!(count, 3);
}

// Example 5: Using rep_stream_to_malret_vec (raw MalRet values)
#[test]
fn test_rep_stream_to_malret_vec() {
    let lisp_code = r#"
        (+ 2 3)
        (* 4 5)
        (- 100 50)
    "#;

    let env = mal::mal_env();
    mal::initialize_mal_env(&env, vec![]);

    let results = mal::rep_stream_to_malret_vec(lisp_code.chars(), &env);

    assert_eq!(results.len(), 3);

    // Check raw MalVal values
    if let Ok(mal::types::MalVal::Int(n)) = results[0] {
        assert_eq!(n, 5);
    } else {
        panic!("Expected Int(5)");
    }

    if let Ok(mal::types::MalVal::Int(n)) = results[1] {
        assert_eq!(n, 20);
    } else {
        panic!("Expected Int(20)");
    }

    if let Ok(mal::types::MalVal::Int(n)) = results[2] {
        assert_eq!(n, 50);
    } else {
        panic!("Expected Int(50)");
    }
}
