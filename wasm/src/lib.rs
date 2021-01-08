use wasm_bindgen::prelude::*;
use lisper::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello world!");
}

#[wasm_bindgen]
pub fn run(exp: String) -> String {
    // Create lisper environment
    let env = &mut lisper::create_default_env();
    // Evaluate the string as a lisper expression
    match evaluate(exp, env) {
        Ok(res) => res.to_string(),
        Err(e) => match e {
            LisperErr::Reason(msg) => msg.to_string(),
        },
    }
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
    fn run() {
        let expected_result:String = "4".to_string();
        let actual_result:String = super::run("(+ 2 2)".to_string());
        assert_eq!(actual_result, expected_result);
    }
}
