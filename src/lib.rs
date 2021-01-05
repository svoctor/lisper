use std::collections::HashMap;
use core::num::ParseFloatError;
use std::fmt;
use std::error;
use std::io;


// Represents an individual Lisp expresion
#[derive(Clone, Debug)]
pub enum LisperExp {
    Symbol(String),
    Number(f64),
    List(Vec<LisperExp>)
}

// Used for to_string
impl fmt::Display for LisperExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str:String = match self {
            LisperExp::Symbol(s) => s.to_string(),
            LisperExp::Number(n) => n.to_string(),
            LisperExp::List(list) => {
                let xs:Vec<String> = list.iter().map(|x| x.to_string()).collect();
                format!("({})", xs.join(","))
            },
        };
        
        write!(f, "{}", str)
    }
}

// An error type for the Lisp interperter
#[derive(Debug)]
pub enum LisperErr {
    ParseError(io::Error),
    Reason(String)
}

impl error::Error for LisperErr {}

impl fmt::Display for LisperErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing expression")
    }
}

//  Represents the context where a Lisp expression executes
#[derive(Clone)]
pub struct LisperEnv {
    pub data: HashMap<String, fn(&LisperExp) -> LisperExp>
}

// Breaks an input string into separate one character tokens
pub fn tokenize(expr: String) -> Vec<String> {
    expr
        .replace("(", "( ")
        .replace(")", " )")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

// Parses an array of string tokens and creates corresponding LisperExp objects
pub fn parse<'a>(tokens: &'a [String]) -> Result<(LisperExp, &'a [String]), LisperErr> {
    let (first, rest) = tokens.split_first()
        .ok_or(
            LisperErr::Reason("Could not get token".to_string())
        )?;

    let mut parsed_result: Vec<LisperExp> = vec![];

    match first.as_str() {
        "(" => {
            let mut more = rest;
            loop {
                let (next, more_next) = more.split_first()
                    .ok_or(
                        LisperErr::Reason("Error reading token".to_string())
                    )?;
                if next == ")" {
                    return Ok((LisperExp::List(parsed_result), more_next))
                }
                let (exp, new_more) = parse(&more)?;
                parsed_result.push(exp);
                more = new_more;
            }
        },
        ")" => {
            return Err(LisperErr::Reason("Parsing error, unexpected ).".to_string()))
        },
        _ => {
            let parsed_token:LisperExp = parse_token(&first);
            return Ok((parsed_token, rest))
        }
    };
}

// Parses an individual token and creates either a Number of Symbol LisperExp
fn parse_token(token: &str) -> LisperExp {
    let parse_result: Result<f64, ParseFloatError> = token.parse();

    match parse_result {
        Ok(value) => LisperExp::Number(value),
        Err(_) => LisperExp::Symbol(token.to_string().clone())
    }
}

// Create a default environment containing fundamental functions
pub fn create_default_env() -> LisperEnv {
    let mut env_data: HashMap<String, fn(&LisperExp) -> LisperExp> = HashMap::new();

    env_data.insert("+".to_string(), add);
    env_data.insert("-".to_string(), sub);
    env_data.insert("*".to_string(), mul);
    env_data.insert("/".to_string(), div);
    env_data.insert("%".to_string(), modulus);

    LisperEnv {data: env_data}
}

// Evaluates a given Lisp expression, and returns a new one with the result.
pub fn eval(exp: LisperExp, env: LisperEnv) -> Result<LisperExp, LisperErr> {
    match exp {
        LisperExp::List(list) => {
            let (sym, args) = list.split_first()
            .ok_or(
                LisperErr::Reason("Error reading token".to_string())
            )?;
            
            let arg0 = eval(args[0].clone(), env.clone())?;
            let arg1 = eval(args[1].clone(), env.clone())?;

            let lisper_func: &fn(&LisperExp) -> LisperExp = env.data.get(&sym.to_string())
            .ok_or(
                LisperErr::Reason("Error reading token".to_string())
            )?;
            
            Ok(lisper_func(&LisperExp::List(vec![arg0, arg1])))
        },
        LisperExp::Number(num) => {
            Ok(LisperExp::Number(num))
        },
        LisperExp::Symbol(_sym) => {
            Err(LisperErr::Reason("Eval issue, not a real expression".to_string()))
        }
    }
}

fn add(args: &LisperExp) -> LisperExp {
    let mut sum = 0.0;
    if let LisperExp::List(list) = args {
        if let LisperExp::Number(n0) = list[0] {
            if let LisperExp::Number(n1) = list[1] {
                sum = n0 + n1;
            }
        }
    }
    return LisperExp::Number(sum)
}

fn sub(args: &LisperExp) -> LisperExp {
    let mut sum = 0.0;
    if let LisperExp::List(list) = args {
        if let LisperExp::Number(n0) = list[0] {
            if let LisperExp::Number(n1) = list[1] {
                sum = n0 - n1;
            }
        }
    }
    return LisperExp::Number(sum)
}

fn mul(args: &LisperExp) -> LisperExp {
    let mut sum = 0.0;
    if let LisperExp::List(list) = args {
        if let LisperExp::Number(n0) = list[0] {
            if let LisperExp::Number(n1) = list[1] {
                sum = n0 * n1;
            }
        }
    }
    return LisperExp::Number(sum)
}

fn div(args: &LisperExp) -> LisperExp {
    let mut sum = 0.0;
    if let LisperExp::List(list) = args {
        if let LisperExp::Number(n0) = list[0] {
            if let LisperExp::Number(n1) = list[1] {
                sum = n0 / n1;
            }
        }
    }
    return LisperExp::Number(sum)
}

fn modulus(args: &LisperExp) -> LisperExp {
    let mut sum = 0.0;
    if let LisperExp::List(list) = args {
        if let LisperExp::Number(n0) = list[0] {
            if let LisperExp::Number(n1) = list[1] {
                sum = n0 % n1;
            }
        }
    }
    return LisperExp::Number(sum)
}

#[cfg(test)]
mod tests {

    #[test]
    fn tokenize_expr() {
        use super::*;

        assert_eq!(tokenize("(+ 1 1)".to_string()), ["(", "+", "1", "1", ")"]);
    }

    #[test]
    fn parse_expr() -> Result<(),  Box<dyn std::error::Error>> {
        use super::*;
        
        // Create a set of valid tokens that we can parse
        let mock_tokens = ["(".to_string(), "+".to_string(), "1".to_string(), "1".to_string(), ")".to_string()];
        
        // Parse mock tockens, expect back a LisperExp::List
        let (parsed_tokens, _) = parse(&mock_tokens[..])?;
        match parsed_tokens {
            LisperExp::List(list) => assert_eq!(list.len(), 3),
            _ => assert!(false)
        }
        Ok(())
    }

    #[test]
    fn parse_expr_complex() -> Result<(),  Box<dyn std::error::Error>> {
        use super::*;
        
        // Create a set of valid tokens that we can parse
        let mock_tokens = ["(".to_string(), "+".to_string(), "1".to_string(), ")".to_string(), "(".to_string(), "*".to_string(), "2".to_string(), "2".to_string(), ")".to_string()];
        
        // Parse mock tockens, expect back a LisperExp::List
        let (parsed_tokens, _) = parse(&mock_tokens[..])?;
        match parsed_tokens {
            LisperExp::List(list) => assert_eq!(list.len(), 2),
            _ => assert!(false)
        }
        Ok(())
    }

    #[test]
    fn create_default_env_add() -> Result<(),  Box<dyn std::error::Error>> {
        use super::*;
        
        let env:LisperEnv = create_default_env();

        let lisper_func: &fn(&LisperExp) -> LisperExp = env.data.get("+")
        .ok_or(
            LisperErr::Reason("Error reading token".to_string())
        )?;
        
        let arg0_f64: f64 = 52.0;
        let arg1_f64: f64 = 13.0;

        let arg0:LisperExp = LisperExp::Number(arg0_f64);
        let arg1:LisperExp = LisperExp::Number(arg1_f64);

        if let LisperExp::Number(res) = lisper_func(&LisperExp::List(vec![arg0, arg1])) {
            assert_eq!(res, arg0_f64 + arg1_f64);
        } else {
            assert!(false);
        }

        Ok(())
    }

    #[test]
    fn create_default_env_sub() -> Result<(),  Box<dyn std::error::Error>> {
        use super::*;
        
        let env:LisperEnv = create_default_env();

        let lisper_func: &fn(&LisperExp) -> LisperExp = env.data.get("-")
        .ok_or(
            LisperErr::Reason("Error reading token".to_string())
        )?;
        
        let arg0_f64: f64 = 52.0;
        let arg1_f64: f64 = 13.0;

        let arg0:LisperExp = LisperExp::Number(arg0_f64);
        let arg1:LisperExp = LisperExp::Number(arg1_f64);

        if let LisperExp::Number(res) = lisper_func(&LisperExp::List(vec![arg0, arg1])) {
            assert_eq!(res, arg0_f64 - arg1_f64);
        } else {
            assert!(false);
        }

        Ok(())
    }

    #[test]
    fn create_default_env_mul() -> Result<(),  Box<dyn std::error::Error>> {
        use super::*;
        
        let env:LisperEnv = create_default_env();

        let lisper_func: &fn(&LisperExp) -> LisperExp = env.data.get("*")
        .ok_or(
            LisperErr::Reason("Error reading token".to_string())
        )?;
        
        let arg0_f64: f64 = 52.0;
        let arg1_f64: f64 = 13.0;

        let arg0:LisperExp = LisperExp::Number(arg0_f64);
        let arg1:LisperExp = LisperExp::Number(arg1_f64);

        if let LisperExp::Number(res) = lisper_func(&LisperExp::List(vec![arg0, arg1])) {
            assert_eq!(res, arg0_f64 * arg1_f64);
        } else {
            assert!(false);
        }

        Ok(())
    }

    #[test]
    fn create_default_env_div() -> Result<(),  Box<dyn std::error::Error>> {
        use super::*;
        
        let env:LisperEnv = create_default_env();

        let lisper_func: &fn(&LisperExp) -> LisperExp = env.data.get("/")
        .ok_or(
            LisperErr::Reason("Error reading token".to_string())
        )?;
        
        let arg0_f64: f64 = 52.0;
        let arg1_f64: f64 = 13.0;

        let arg0:LisperExp = LisperExp::Number(arg0_f64);
        let arg1:LisperExp = LisperExp::Number(arg1_f64);

        if let LisperExp::Number(res) = lisper_func(&LisperExp::List(vec![arg0, arg1])) {
            assert_eq!(res, arg0_f64 / arg1_f64);
        } else {
            assert!(false);
        }

        Ok(())
    }

    #[test]
    fn create_default_env_mod() -> Result<(),  Box<dyn std::error::Error>> {
        use super::*;
        
        let env:LisperEnv = create_default_env();

        let lisper_func: &fn(&LisperExp) -> LisperExp = env.data.get("%")
        .ok_or(
            LisperErr::Reason("Error reading token".to_string())
        )?;
        
        let arg0_f64: f64 = 52.0;
        let arg1_f64: f64 = 13.0;

        let arg0:LisperExp = LisperExp::Number(arg0_f64);
        let arg1:LisperExp = LisperExp::Number(arg1_f64);

        if let LisperExp::Number(res) = lisper_func(&LisperExp::List(vec![arg0, arg1])) {
            assert_eq!(res, arg0_f64 % arg1_f64);
        } else {
            assert!(false);
        }

        Ok(())
    }
}