use std::env;
use std::fs;

use yoloxide::tokenizer;
use yoloxide::parser;

use yoloxide::types::Token;
use yoloxide::types::Statement as Stat;
use yoloxide::types::Expression as Expr;
use yoloxide::types::Operator as Op;

fn main()
{
    let args: Vec<String> = env::args().collect();

    // Take the first argument as a file path and read it for yolol code
    let yolol_code = fs::read_to_string(&args[1]).unwrap();

    println!("Original code:");
    println!("{}", yolol_code);
    // println!("Basic tokens:");
    let tokens = tokenizer::basic::tokenize_basic(yolol_code).unwrap();
    // println!("{:?}", tokens);
    let statements = parser::parse(tokens).unwrap();
    // let tokens = tokenizer::extended::tokenize_extended(tokens);
    // println!("Parsed statements:");
    // println!("{:#?}", statements);
    println!("Re-codified AST:");

    for statement in statements
    {
        println!("{}", statement);
    }

    // let test: Stat = Stat::Assignment(Token::Caret, Op::Assign, Box::new(Expr::Value(Token::Caret)));

    // println!("Testing thing...");
    // println!("{}", test);

    // println!("Adding 5: {}", yolol_num + YololNumber::from(5));
}
