use std::env;
use std::fs;

use yoloxide::tokenizer;

fn main()
{
    let args: Vec<String> = env::args().collect();

    // Take the first argument as a file path and read it for yolol code
    let yolol_code = fs::read_to_string(args[1].clone()).unwrap();

    println!("{}", yolol_code);
    println!();
    println!("{:?}", tokenizer::tokenize(yolol_code));
}
