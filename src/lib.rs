pub mod types;
pub mod environment;

pub mod parser;
pub mod tokenizer;
pub mod interpreter;

pub mod wasm_lib;

use environment::Environment;

pub fn execute_line(mut env: Environment, line: String) -> Environment
{
    let tokens = match tokenizer::tokenize(line)
    {
        Ok(tokens) => tokens,
        Err(error) => {
            env.error = error.to_string();
            return env;
        }
    };

    let mut window = types::VecWindow::new(&tokens, 0);
    let line = match parser::parse_line(&mut window)
    {
        Ok(line) => line,
        Err(error) => {
            env.error = error.to_string();
            return env;
        }
    };

    if let Err(error) = interpreter::evaluate_line(&mut env, &line)
    {
        env.error = error.to_string();
        return env;
    }

    env
}