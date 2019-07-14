#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;

use crate::environment::Environment;
use crate::execute_line;

#[wasm_bindgen]
pub fn wasm_execute_line(env: JsValue, line: String) -> JsValue
{
    let mut env: Environment = env.into_serde().unwrap();
    execute_line(&mut env, line);

    JsValue::from_serde(&env).unwrap()
}