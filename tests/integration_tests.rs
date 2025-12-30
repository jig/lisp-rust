use mal::{initialize_mal_env, mal_env, rep};

#[test]
fn test_rep_addition() {
    let env = mal_env();
    initialize_mal_env(&env, vec![]);

    match rep("(+ 1 1)", &env) {
        Ok(s) => assert_eq!(s, "2"),
        Err(_) => panic!("rep() returned an error"),
    }
}

#[test]
fn test_rep_addition_too_many_args() {
    let env = mal_env();
    initialize_mal_env(&env, vec![]);

    // Should error because + only accepts 2 arguments
    match rep("(+ 1 1 1)", &env) {
        Ok(s) => panic!("Should have returned an error, but got: {}", s),
        Err(_) => (), // Expected error
    }
}

#[test]
fn test_rep_addition_too_few_args() {
    let env = mal_env();
    initialize_mal_env(&env, vec![]);

    // Should error because + requires 2 arguments
    match rep("(+ 1)", &env) {
        Ok(s) => panic!("Should have returned an error, but got: {}", s),
        Err(_) => (), // Expected error
    }
}

#[test]
fn test_rep_divide_by_zero() {
    let env = mal_env();
    initialize_mal_env(&env, vec![]);

    // Should error because division by zero is not allowed
    match rep("(/ 1 0)", &env) {
        Ok(s) => panic!("Should have returned an error, but got: {}", s),
        Err(_) => (), // Expected error
    }
}

#[test]
fn test_rep_str() {
    let env = mal_env();
    initialize_mal_env(&env, vec![]);

    match rep("(str \"Hello, \" \"world!\")", &env) {
        Ok(s) => assert_eq!(s, "\"Hello, world!\""),
        Err(_) => panic!("rep() returned an error"),
    }
}

