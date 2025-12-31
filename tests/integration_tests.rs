use mal::{initialize_mal_env, mal_env, rep};

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

#[test]
fn eval_eval() {
    let env = mal_env();
    initialize_mal_env(&env, vec![]);

    match rep("(eval (read-string \"(+ 1 10)\"))", &env) {
        Ok(s) => assert_eq!(s, "11"),
        Err(_) => panic!("rep() returned an error"),
    }
}