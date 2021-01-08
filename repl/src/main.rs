use std::io;
use std::io::Write;
use lisper;

const PKG_VERSION:&str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let env = &mut lisper::create_default_env();
    
    println!("Lisper v{}", PKG_VERSION);

    loop {
        let mut input_buffer = String::new();

        let stdin = io::stdin();

        print!("$ ");
        io::stdout().flush().expect("Unable to flush output");

        stdin.read_line(&mut input_buffer).expect("Unable to read line");
        let expr:String = input_buffer.trim().to_string();
        match evaluate(expr, env) {
          Ok(res) => println!("{}", res),
          Err(e) => match e {
            lisper::LisperErr::Reason(msg) => println!("// 🙀 => {}", msg),
          },
        }
      }
}

fn evaluate(exp:String, env: &mut lisper::LisperEnv) -> Result<String, lisper::LisperErr> {
    let tokens:Vec<String> = lisper::tokenize(exp);
    let (parsed_tokens, _) = lisper::parse(&tokens)?;
    let eval_out = lisper::eval(parsed_tokens, env)?;

    Ok(eval_out.to_string())
}