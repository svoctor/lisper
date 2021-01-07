use lisper;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let env = &mut lisper::create_default_env();

    let exp:String = "(+ (+ 1 1) (*  2 2))".to_string();
    let res:String = evaluate(exp.clone(), env)?;
    println!("{} = {}", exp, res);

    let exp:String = "(/ (* 13 18.3) (% 9 5))".to_string();
    let res:String = evaluate(exp.clone(), env)?;
    println!("{} = {}", exp, res);

    let exp:String = "(* (* (* 10 10) (* 10 10)) (* (* 10 10) (* 10 10))))".to_string();
    let res:String = evaluate(exp.clone(), env)?;
    println!("{} = {}", exp, res);

    Ok(())
}

fn evaluate(exp:String, env: &mut lisper::LisperEnv) -> Result<String, lisper::LisperErr> {
    let tokens:Vec<String> = lisper::tokenize(exp);
    let (parsed_tokens, _) = lisper::parse(&tokens)?;
    let eval_out = lisper::eval(parsed_tokens, env)?;

    Ok(eval_out.to_string())
}