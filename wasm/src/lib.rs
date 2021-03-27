use std::{str::Lines};
use wasm_bindgen::prelude::*;

use lisper::env::{ LisperEnv, create_default_env };
use lisper::core::{ 
    tokenize,
    parse,
    eval,
    LisperErr
};

#[wasm_bindgen]
pub fn run(exp: String) -> String {
    // Create lisper environment
    let env = &mut create_default_env();
    
    // Split lines into strings and evaluate as lisper expressions
    let lines: Lines = exp.lines();
    match evaluate_lines(lines, env) {
        Ok(res) => res,
        Err(e) => e.to_string(),
    }
}

fn evaluate_lines(exp_lines:Lines, env: &mut LisperEnv) -> Result<String, LisperErr> {
    let results = exp_lines
            .map(|l| {
                match evaluate(l.to_string(), env) {
                    Ok(res) => res,
                    Err(e) => e.to_string()
                }
            })
            .collect::<Vec<String>>();
    Ok(results.last().unwrap().to_string())
}

fn evaluate(exp:String, env: &mut LisperEnv) -> Result<String, LisperErr> {
    let tokens:Vec<String> = tokenize(exp);
    let (parsed_tokens, _) = parse(&tokens)?;
    let eval_out = eval(parsed_tokens, env)?;

    Ok(eval_out.to_string())
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_exp() {
        let expected_result:String = "4".to_string();
        let actual_result:String = super::run("(+ 2 2)".to_string());
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn multiline_exp() {
        let expected_result:String = "4".to_string();
        let actual_result:String = super::run("(def w 2)\n(+ 2 w)".to_string());
        assert_eq!(actual_result, expected_result);
    }
}
