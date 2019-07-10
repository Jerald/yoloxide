use std::env;
use std::fs;

use yoloxide::environment::Environment;

use yoloxide::tokenizer;
use yoloxide::parser;
use yoloxide::interpreter;


fn main()
{
    let args: Vec<String> = env::args().collect();

    // Take the first argument as a file path and read it for yolol code
    let yolol_code = fs::read_to_string(&args[1]).unwrap();

    println!("Original code:");
    println!("{}", yolol_code);

    let tokens = tokenizer::tokenize(yolol_code, true).unwrap();
    println!("Tokens:");
    println!("{:?}", tokens);


    let statements = parser::parse(tokens).unwrap();
    println!("AST:");
    println!("{:?}", statements);

    let mut test_env = Environment::new("Test Env");

    println!("Re-codified AST:");
    for statement in statements
    {
        println!("{}", statement);
        let eval_output = interpreter::evaluate_statement(&mut test_env, statement.clone());

        eval_output.unwrap_or_else(|error| {
            println!("{}", statement);
            println!("{}", error);
        });
    }

    println!("{}", test_env);
}
