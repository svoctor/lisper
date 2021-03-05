use std::{str::Lines};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(exp: String) -> String {
    // Create lisper environment
    let env = &mut lisper::create_default_env();
    
    // Split lines into strings and evaluate as lisper expressions
    let lines: Lines = exp.lines();
    match evaluate_lines(lines, env) {
        Ok(res) => res,
        Err(e) => e.to_string(),
    }
}

fn evaluate_lines(exp_lines:Lines, env: &mut lisper::LisperEnv) -> Result<String, lisper::LisperErr> {
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

fn evaluate(exp:String, env: &mut lisper::LisperEnv) -> Result<String, lisper::LisperErr> {
    let tokens:Vec<String> = lisper::tokenize(exp);
    let (parsed_tokens, _) = lisper::parse(&tokens)?;
    let eval_out = lisper::eval(parsed_tokens, env)?;

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
