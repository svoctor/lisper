use std::io;
use std::io::Write;
use lisper;

// Get package version defined in cargo.toml
const PKG_VERSION:&str = env!("CARGO_PKG_VERSION");

fn evaluate(exp:String, env: &mut lisper::LisperEnv) -> Result<String, lisper::LisperErr> {
    let tokens:Vec<String> = lisper::tokenize(exp);
    let (parsed_tokens, _) = lisper::parse(&tokens)?;
    let eval_out = lisper::eval(parsed_tokens, env)?;

    Ok(eval_out.to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    // Create lisper environment
    let env = &mut lisper::create_default_env();
    
    // Welcome message, including current version
    println!("Lisper v{}", PKG_VERSION);

    loop {
        // Input buffer
        let mut input_buffer = String::new();

        let stdin = io::stdin();

        // Input prompt, including flush to force print
        print!("$ ");
        io::stdout().flush().expect("Unable to flush output");

        // Read input string
        stdin.read_line(&mut input_buffer).expect("Unable to read line");
        let expr:String = input_buffer.trim().to_string();

        // Match string to supported commands
        match expr.as_str() {
            "/q" => {
                // Exit the process
                std::process::exit(0);
            },
            _ => {
                // Evaluate the string as a lisper expression
                match evaluate(expr, env) {
                    Ok(res) => println!("{}", res),
                    Err(e) => match e {
                    lisper::LisperErr::Reason(msg) => println!("Error = {}", msg),
                    },
                }
            }
        }
    }
}