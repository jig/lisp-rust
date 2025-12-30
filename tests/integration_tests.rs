use mal::{initialize_mal_env, mal_env, mal_env_with_readline, rep};

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
fn rep_addition_too_many_args() {
    let env = mal_env();
    initialize_mal_env(&env, vec![]);

    // Should error because + only accepts 2 arguments
    match rep("(+ 1 1 1)", &env) {
        Ok(s) => panic!("Should have returned an error, but got: {}", s),
        Err(_) => (), // Expected error
    }
}

#[test]
fn rep_addition_too_few_args() {
    let env = mal_env();
    initialize_mal_env(&env, vec![]);

    // Should error because + requires 2 arguments
    match rep("(+ 1)", &env) {
        Ok(s) => panic!("Should have returned an error, but got: {}", s),
        Err(_) => (), // Expected error
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
fn readline_without_function() {
    // Test that readline is not available when not provided
    let env = mal_env();
    initialize_mal_env(&env, vec![]);

    // readline should not be defined in the environment
    match rep("(readline \"prompt> \")", &env) {
        Ok(_) => panic!("readline should not be available without providing a readline function"),
        Err(_) => (), // Expected error - readline is not defined
    }
}

#[test]
fn readline_with_function() {
    // Mock readline function for testing
    fn mock_readline(prompt: &str) -> Option<String> {
        assert_eq!(prompt, "test> ");
        Some("test input".to_string())
    }

    let env = mal_env_with_readline(Some(mock_readline));
    initialize_mal_env(&env, vec![]);

    // readline should be available and return the mocked value
    match rep("(readline \"test> \")", &env) {
        Ok(s) => assert_eq!(s, "\"test input\""),
        Err(e) => panic!("readline should be available: {}", e.pr_str(true)),
    }
}
