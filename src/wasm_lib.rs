#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use yoloxide;

#[wasm_bindgen]
pub fn wasm_execute_line(mut env: Environment, line: String) -> Environment
{
    execute_line(env, line)
}