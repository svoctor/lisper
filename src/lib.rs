use std::collections::HashMap;
use std::fmt;
use std::error;

// Represents an individual Lisp expresion
#[derive(Clone)]
pub enum LisperExp {
    Bool(bool),
    Symbol(String),
    Number(f64),
    List(Vec<LisperExp>),
    Func(fn(&LisperExp) -> LisperExp)
}

// Used for to_string
impl fmt::Display for LisperExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str:String = match self {
            LisperExp::Symbol(s) => s.to_string(),
            LisperExp::Number(n) => n.to_string(),
            LisperExp::Bool(b) => b.to_string(),
            LisperExp::List(list) => {
                let items:Vec<String> = list.iter().map(|item| item.to_string()).collect();
                format!("({})", items.join(","))
            },
            LisperExp::Func(_) => "Function {}".to_string()
        };
        
        write!(f, "{}", str)
    }
}

// An error type for the Lisp interperter
#[derive(Debug)]
pub enum LisperErr {
    Reason(String)
}

impl error::Error for LisperErr {}

impl fmt::Display for LisperErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LisperErr::Reason(reason) => write!(f, "{}", reason),
        }
    }
}

//  Represents the context where a Lisp expression executes
#[derive(Clone)]
pub struct LisperEnv {
    pub data: HashMap<String, LisperExp>
}

// Breaks an input string into separate one character tokens
// TODO: Handle )( without the spaces
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
                        LisperErr::Reason("Error reading token, missing ).".to_string())
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
            return Err(LisperErr::Reason("Parsing error, found unexpected ).".to_string()))
        },
        _ => {
            let parsed_token:LisperExp = parse_token(&first);
            return Ok((parsed_token, rest))
        }
    };
}

// Parses an individual token and creates either a Number of Symbol LisperExp
fn parse_token(token: &str) -> LisperExp {
    if let Result::Ok(parsed_bool) = token.parse::<bool>() {
        LisperExp::Bool(parsed_bool)
    } else if let Result::Ok(parsed_value) = token.parse::<f64>() {
        LisperExp::Number(parsed_value)
    } else {
        LisperExp::Symbol(token.to_string().clone())
    }
}

// Create a default environment containing fundamental functions
pub fn create_default_env() -> LisperEnv {
    let mut env_data: HashMap<String, LisperExp> = HashMap::new();

    // Basic math functions
    env_data.insert("+".to_string(), LisperExp::Func(add));
    env_data.insert("-".to_string(), LisperExp::Func(sub));
    env_data.insert("sub".to_string(), LisperExp::Func(sub));
    env_data.insert("*".to_string(), LisperExp::Func(mul));
    env_data.insert("mul".to_string(), LisperExp::Func(mul));
    env_data.insert("/".to_string(), LisperExp::Func(div));
    env_data.insert("div".to_string(), LisperExp::Func(div));
    env_data.insert("%".to_string(), LisperExp::Func(modulus));
    env_data.insert("mod".to_string(), LisperExp::Func(modulus));

    // Comparators
    env_data.insert("<".to_string(), LisperExp::Func(less_than));
    env_data.insert(">".to_string(), LisperExp::Func(more_than));
    env_data.insert("=".to_string(), LisperExp::Func(equals));
    env_data.insert("==".to_string(), LisperExp::Func(equals));
    env_data.insert("<=".to_string(), LisperExp::Func(less_or_equal));
    env_data.insert(">=".to_string(), LisperExp::Func(more_or_equal));

    // Trig functions
    env_data.insert("sin".to_string(), LisperExp::Func(sin));
    env_data.insert("cos".to_string(), LisperExp::Func(cos));
    env_data.insert("tan".to_string(), LisperExp::Func(tan));

    // Trig constants
    env_data.insert("pi".to_string(), LisperExp::Number(core::f64::consts::PI));
    env_data.insert("two_pi".to_string(), LisperExp::Number(core::f64::consts::PI * 2.0));
    env_data.insert("e".to_string(), LisperExp::Number(core::f64::consts::E));

    LisperEnv {data: env_data}
}

// Evaluates a given Lisp expression, and returns a new one with the result.
pub fn eval(exp: LisperExp, env: &mut LisperEnv) -> Result<LisperExp, LisperErr> {
    match exp {
        LisperExp::List(list) => {
            // Split the symbol from the arguments
            let (first, args) = list.split_first()
            .ok_or(
                LisperErr::Reason("Error reading expression".to_string())
            )?;
            match first {
                LisperExp::List(first) => {
                    match eval(LisperExp::List(first.clone()), env) {
                        Ok(_) => eval(LisperExp::List(args.to_vec()), env),
                        Err(e) => Err(e)
                    }
                },
                LisperExp::Symbol(sym) => {
                    // Catch def, fn and if, and else evalue as a regular env function
                    match sym.to_string().as_str() {
                        "if" => {
                            // It's an if statement
                            // Format: (if (statement[as LisperExp]) (if true[as LisperExp]) (if false[as LisperExp]))
                            Ok(LisperExp::Symbol("N/A".to_string()))
                        },
                        "def" => {
                            // It's a variable definition
                            // Format: (def variable_name[as string] (value[as LisperExp]))

                            // TODO: Figure out if we should block over-writing predefined constants here
                            if args.len() != 2 {
                                return Err(LisperErr::Reason("Syntax error, def only takes 2 arguments, name and an expression.".to_string()));
                            }

                            let variable_name:String = args[0].to_string();
                            let variable_value:LisperExp = eval(args[1].clone(), env)?;
                            
                            env.data.insert(variable_name, variable_value.clone());

                            Ok(variable_value)
                        },
                        "fn" => {
                            // It's a function definition
                            // Format: (def function_name[as string] (argument[as LisperExp list]) (function[as LisperExp]))
                            Ok(LisperExp::Symbol("N/A".to_string()))
                        },
                        _ => {
                            // It's a env function, so evaluate that
                            // Evaluate each argument
                            let mut evaluated_args: Vec<LisperExp> = vec![];
                            for arg in args.iter() {
                                evaluated_args.push(eval(arg.clone(), env)?)
                            }

                            // Get the env function based on the symbol
                            // Run the function with the args, and return the result
                            let func = env.data.get(&sym.to_string()).ok_or(
                                LisperErr::Reason("Error, env function not found.".to_string())
                            )?;
                            match func {
                                LisperExp::Func(lisper_func) => Ok(lisper_func(&LisperExp::List(evaluated_args))),
                                _ => Err(LisperErr::Reason("Error, function not found.".to_string()))
                            }
                        }
                    }
                },
                _ => {
                    Err(LisperErr::Reason("Parsing error.".to_string()))
                }
            }
        },
        LisperExp::Number(num) => {
            // If it's just a number, then return the number
            Ok(LisperExp::Number(num))
        },
        LisperExp::Symbol(sym) => {
            println!("Found symbol: {}", sym);
            let lisper_exp = env.data.get(&sym.to_string())
            .ok_or (
                // We shouldn't be evaluating function symbols here, since they should be
                // wrapped in lists above. Something is wrong, return an error.
                LisperErr::Reason("Eval issue, not a real expression".to_string())
            )?;

            // This is actually a def, so return the value 
            Ok(lisper_exp.clone())
        },
        LisperExp::Bool(b) => {
            Ok(LisperExp::Bool(b))
        },
        LisperExp::Func(_) => Err(LisperErr::Reason("Unexpected function".to_string())),
        // _ => Err(LisperErr::Reason("Unexpected parsing error".to_string()))
    }
}

fn add(args: &LisperExp) -> LisperExp {
    let mut sum = 0.0;
    if let LisperExp::List(list) = args {
        for (i, arg) in list.iter().enumerate() {
            if let LisperExp::Number(n) = arg {
                if i == 0 {
                    sum = *n;
                } else {
                    sum += n;
                }
            }
        }
    }
    return LisperExp::Number(sum)
}

fn sub(args: &LisperExp) -> LisperExp {
    let mut sum = 0.0;
    if let LisperExp::List(list) = args {
        for (i, arg) in list.iter().enumerate() {
            if let LisperExp::Number(n) = arg {
                if i == 0 {
                    sum = *n;
                } else {
                    sum -= n;
                }
            }
        }
    }
    return LisperExp::Number(sum)
}

fn mul(args: &LisperExp) -> LisperExp {
    let mut sum = 0.0;
    if let LisperExp::List(list) = args {
        for (i, arg) in list.iter().enumerate() {
            if let LisperExp::Number(n) = arg {
                if i == 0 {
                    sum = *n;
                } else {
                    sum *= n;
                }
            }
        }
    }
    return LisperExp::Number(sum)
}

fn div(args: &LisperExp) -> LisperExp {
    let mut sum = 0.0;
    if let LisperExp::List(list) = args {
        for (i, arg) in list.iter().enumerate() {
            if let LisperExp::Number(n) = arg {
                if i == 0 {
                    sum = *n;
                } else {
                    sum /= n;
                }
            }
        }
    }
    return LisperExp::Number(sum)
}

fn modulus(args: &LisperExp) -> LisperExp {
    let mut sum = 0.0;
    if let LisperExp::List(list) = args {
        for (i, arg) in list.iter().enumerate() {
            if let LisperExp::Number(n) = arg {
                if i == 0 {
                    sum = *n;
                } else {
                    sum %= n;
                }
            }
        }
    }
    return LisperExp::Number(sum)
}

fn less_than(args: &LisperExp) -> LisperExp {
    let mut prev = 0.0;
    let mut res = false;
    if let LisperExp::List(list) = args {
        for (i, arg) in list.iter().enumerate() {
            if let LisperExp::Number(n) = arg {
                if i == 0 {
                    prev = *n;
                } else {
                    res = prev < *n;
                    prev = *n;
                }
            }
        }
    }
    return LisperExp::Bool(res)
}

fn more_than(args: &LisperExp) -> LisperExp {
    let mut prev = 0.0;
    let mut res = false;
    if let LisperExp::List(list) = args {
        for (i, arg) in list.iter().enumerate() {
            if let LisperExp::Number(n) = arg {
                if i == 0 {
                    prev = *n;
                } else {
                    res = prev > *n;
                    prev = *n;
                }
            }
        }
    }
    return LisperExp::Bool(res)
}

fn equals(args: &LisperExp) -> LisperExp {
    let mut prev = 0.0;
    let mut res = false;
    if let LisperExp::List(list) = args {
        for (i, arg) in list.iter().enumerate() {
            if let LisperExp::Number(n) = arg {
                if i == 0 {
                    prev = *n;
                } else {
                    res = prev == *n;
                    prev = *n;
                }
            }
        }
    }
    return LisperExp::Bool(res)
}

fn less_or_equal(args: &LisperExp) -> LisperExp {
    let mut prev = 0.0;
    let mut res = false;
    if let LisperExp::List(list) = args {
        for (i, arg) in list.iter().enumerate() {
            if let LisperExp::Number(n) = arg {
                if i == 0 {
                    prev = *n;
                } else {
                    res = prev <= *n;
                    println!("{} <= {} = {}", prev.to_string(), n.to_string(), res.to_string());
                    prev = *n;
                }
            }
        }
    }
    return LisperExp::Bool(res)
}

fn more_or_equal(args: &LisperExp) -> LisperExp {
    let mut prev = 0.0;
    let mut res = false;
    if let LisperExp::List(list) = args {
        for (i, arg) in list.iter().enumerate() {
            if let LisperExp::Number(n) = arg {
                if i == 0 {
                    prev = *n;
                } else {
                    res = prev >= *n;
                    prev = *n;
                }
            }
        }
    }
    return LisperExp::Bool(res)
}

fn sin(args: &LisperExp) -> LisperExp {
    let mut res = 0.0;
    if let LisperExp::List(list) = args {
        if let LisperExp::Number(n) = list[0] {
            res = n.sin();
        }
    }
    LisperExp::Number(res)
}

fn cos(args: &LisperExp) -> LisperExp {
    let mut res = 0.0;
    if let LisperExp::List(list) = args {
        if let LisperExp::Number(n) = list[0] {
            res = n.cos();
        }
    }
    LisperExp::Number(res)
}

fn tan(args: &LisperExp) -> LisperExp {
    let mut res = 0.0;
    if let LisperExp::List(list) = args {
        if let LisperExp::Number(n) = list[0] {
            res = n.tan();
        }
    }
    LisperExp::Number(res)
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
    fn parse_number_expr() -> Result<(),  Box<dyn std::error::Error>> {
        use super::*;
        
        // Create a set of valid tokens that we can parse
        let mock_token = "99";
        
        // Parse mock tockens, expect back a LisperExp::List
        match parse_token(&mock_token) {
            LisperExp::Number(num) => assert_eq!(num, 99.0),
            _ => assert!(false)
        }
        Ok(())
    }

    #[test]
    fn parse_symbol_expr() -> Result<(),  Box<dyn std::error::Error>> {
        use super::*;
        
        // Create a set of valid tokens that we can parse
        let mock_token = "+";
        
        // Parse mock tockens, expect back a LisperExp::List
        match parse_token(&mock_token) {
            LisperExp::Symbol(sym) => assert_eq!(sym.to_string(), "+".to_string()),
            _ => assert!(false)
        }
        Ok(())
    }

    #[test]
    fn parse_bool_expr() -> Result<(),  Box<dyn std::error::Error>> {
        use super::*;
        
        // Create a set of valid tokens that we can parse
        let mock_token = "true";
        
        // Parse mock tockens, expect back a LisperExp::List
        match parse_token(&mock_token) {
            LisperExp::Bool(b) => assert!(b),
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

        let func = env.data.get("+").ok_or(
            LisperErr::Reason("Error, function not found.".to_string())
        )?;

        match func {
            LisperExp::Func(f) => {
                let arg0_f64: f64 = 52.0;
                let arg1_f64: f64 = 13.0;
        
                let arg0:LisperExp = LisperExp::Number(arg0_f64);
                let arg1:LisperExp = LisperExp::Number(arg1_f64);
        
                if let LisperExp::Number(res) = f(&LisperExp::List(vec![arg0, arg1])) {
                    assert_eq!(res, arg0_f64 + arg1_f64);
                } else {
                    assert!(false);
                }
            },
            _ => assert!(false)
        }

        Ok(())
    }

    #[test]
    fn create_default_env_sub() -> Result<(),  Box<dyn std::error::Error>> {
        use super::*;
        
        let env:LisperEnv = create_default_env();

        let func = env.data.get("-").ok_or(
            LisperErr::Reason("Error, function not found.".to_string())
        )?;

        match func {
            LisperExp::Func(f) => {
                let arg0_f64: f64 = 52.0;
                let arg1_f64: f64 = 13.0;
        
                let arg0:LisperExp = LisperExp::Number(arg0_f64);
                let arg1:LisperExp = LisperExp::Number(arg1_f64);
        
                if let LisperExp::Number(res) = f(&LisperExp::List(vec![arg0, arg1])) {
                    assert_eq!(res, arg0_f64 - arg1_f64);
                } else {
                    assert!(false);
                }
            },
            _ => assert!(false)
        }
        
        Ok(())
    }

    #[test]
    fn create_default_env_mul() -> Result<(),  Box<dyn std::error::Error>> {
        use super::*;
        
        let env:LisperEnv = create_default_env();

        let func = env.data.get("*").ok_or(
            LisperErr::Reason("Error, function not found.".to_string())
        )?;

        match func {
            LisperExp::Func(f) => {
                let arg0_f64: f64 = 52.0;
                let arg1_f64: f64 = 13.0;
        
                let arg0:LisperExp = LisperExp::Number(arg0_f64);
                let arg1:LisperExp = LisperExp::Number(arg1_f64);
        
                if let LisperExp::Number(res) = f(&LisperExp::List(vec![arg0, arg1])) {
                    assert_eq!(res, arg0_f64 * arg1_f64);
                } else {
                    assert!(false);
                }
            },
            _ => assert!(false)
        }

        Ok(())
    }

    #[test]
    fn create_default_env_div() -> Result<(),  Box<dyn std::error::Error>> {
        use super::*;
        
        let env:LisperEnv = create_default_env();

        let func = env.data.get("/").ok_or(
            LisperErr::Reason("Error, function not found.".to_string())
        )?;

        match func {
            LisperExp::Func(f) => {
                let arg0_f64: f64 = 52.0;
                let arg1_f64: f64 = 13.0;
        
                let arg0:LisperExp = LisperExp::Number(arg0_f64);
                let arg1:LisperExp = LisperExp::Number(arg1_f64);
        
                if let LisperExp::Number(res) = f(&LisperExp::List(vec![arg0, arg1])) {
                    assert_eq!(res, arg0_f64 / arg1_f64);
                } else {
                    assert!(false);
                }
            },
            _ => assert!(false)
        }

        Ok(())
    }

    #[test]
    fn create_default_env_mod() -> Result<(),  Box<dyn std::error::Error>> {
        use super::*;
        
        let env:LisperEnv = create_default_env();

        let func = env.data.get("%").ok_or(
            LisperErr::Reason("Error, function not found.".to_string())
        )?;

        match func {
            LisperExp::Func(f) => {
                let arg0_f64: f64 = 52.0;
                let arg1_f64: f64 = 13.0;
        
                let arg0:LisperExp = LisperExp::Number(arg0_f64);
                let arg1:LisperExp = LisperExp::Number(arg1_f64);
        
                if let LisperExp::Number(res) = f(&LisperExp::List(vec![arg0, arg1])) {
                    assert_eq!(res, arg0_f64 % arg1_f64);
                } else {
                    assert!(false);
                }
            },
            _ => assert!(false)
        }

        Ok(())
    }

    #[test]
    fn create_default_env_less_than() -> Result<(),  Box<dyn std::error::Error>> {
        use super::*;
        
        let env:LisperEnv = create_default_env();

        let func = env.data.get("<").ok_or(
            LisperErr::Reason("Error, function not found.".to_string())
        )?;

        match func {
            LisperExp::Func(f) => {
                let arg0_f64: f64 = 5.0;
                let arg1_f64: f64 = 13.0;
        
                let arg0:LisperExp = LisperExp::Number(arg0_f64);
                let arg1:LisperExp = LisperExp::Number(arg1_f64);
        
                if let LisperExp::Bool(res) = f(&LisperExp::List(vec![arg0, arg1])) {
                    assert_eq!(res, arg0_f64 < arg1_f64);
                } else {
                    assert!(false);
                }
            },
            _ => assert!(false)
        }

        Ok(())
    }

    #[test]
    fn create_default_env_more_than() -> Result<(),  Box<dyn std::error::Error>> {
        use super::*;
        
        let env:LisperEnv = create_default_env();

        let func = env.data.get(">").ok_or(
            LisperErr::Reason("Error, function not found.".to_string())
        )?;

        match func {
            LisperExp::Func(f) => {
                let arg0_f64: f64 = 5.0;
                let arg1_f64: f64 = 13.0;
        
                let arg0:LisperExp = LisperExp::Number(arg0_f64);
                let arg1:LisperExp = LisperExp::Number(arg1_f64);
        
                if let LisperExp::Bool(res) = f(&LisperExp::List(vec![arg0, arg1])) {
                    assert_eq!(res, arg0_f64 > arg1_f64);
                } else {
                    assert!(false);
                }
            },
            _ => assert!(false)
        }
        
        Ok(())
    }

    #[test]
    fn create_default_env_equals() -> Result<(),  Box<dyn std::error::Error>> {
        use super::*;
        
        let env:LisperEnv = create_default_env();

        let func = env.data.get("=").ok_or(
            LisperErr::Reason("Error, function not found.".to_string())
        )?;

        match func {
            LisperExp::Func(f) => {
                let arg0_f64: f64 = 5.0;
                let arg1_f64: f64 = 5.0;
        
                let arg0:LisperExp = LisperExp::Number(arg0_f64);
                let arg1:LisperExp = LisperExp::Number(arg1_f64);
        
                if let LisperExp::Bool(res) = f(&LisperExp::List(vec![arg0, arg1])) {
                    assert_eq!(res, arg0_f64 == arg1_f64);
                } else {
                    assert!(false);
                }
            },
            _ => assert!(false)
        }

        Ok(())
    }

    #[test]
    fn create_default_env_less_or_equal() -> Result<(),  Box<dyn std::error::Error>> {
        use super::*;
        
        let env:LisperEnv = create_default_env();

        let func = env.data.get("<=").ok_or(
            LisperErr::Reason("Error, function not found.".to_string())
        )?;

        match func {
            LisperExp::Func(f) => {
                let arg0_f64: f64 = 6.0;
                let arg1_f64: f64 = 5.0;
        
                let arg0:LisperExp = LisperExp::Number(arg0_f64);
                let arg1:LisperExp = LisperExp::Number(arg1_f64);
        
                if let LisperExp::Bool(res) = f(&LisperExp::List(vec![arg0, arg1])) {
                    assert_eq!(res, arg0_f64 <= arg1_f64);
                } else {
                    assert!(false);
                }
            },
            _ => assert!(false)
        }

        Ok(())
    }

    #[test]
    fn create_default_env_more_or_equal() -> Result<(),  Box<dyn std::error::Error>> {
        use super::*;
        
        let env:LisperEnv = create_default_env();

        let func = env.data.get(">=").ok_or(
            LisperErr::Reason("Error, function not found.".to_string())
        )?;

        match func {
            LisperExp::Func(f) => {
                let arg0_f64: f64 = 3.0;
                let arg1_f64: f64 = 5.0;
        
                let arg0:LisperExp = LisperExp::Number(arg0_f64);
                let arg1:LisperExp = LisperExp::Number(arg1_f64);
        
                if let LisperExp::Bool(res) = f(&LisperExp::List(vec![arg0, arg1])) {
                    assert_eq!(res, arg0_f64 >= arg1_f64);
                } else {
                    assert!(false);
                }
            },
            _ => assert!(false)
        }

        Ok(())
    }

    #[test]
    fn create_default_env_sin() -> Result<(),  Box<dyn std::error::Error>> {
        use super::*;
        
        let env:LisperEnv = create_default_env();

        let func = env.data.get("sin").ok_or(
            LisperErr::Reason("Error, function not found.".to_string())
        )?;

        match func {
            LisperExp::Func(f) => {
                let arg0_f64: f64 = core::f64::consts::PI;

                let arg0:LisperExp = LisperExp::Number(arg0_f64);
        
                if let LisperExp::Number(res) = f(&LisperExp::List(vec![arg0])) {
                    assert_eq!(res, arg0_f64.sin());
                } else {
                    assert!(false);
                }
            },
            _ => assert!(false)
        }

        Ok(())
    }

    #[test]
    fn create_default_env_cos() -> Result<(),  Box<dyn std::error::Error>> {
        use super::*;
        
        let env:LisperEnv = create_default_env();

        let func = env.data.get("cos").ok_or(
            LisperErr::Reason("Error, function not found.".to_string())
        )?;

        match func {
            LisperExp::Func(f) => {
                let arg0_f64: f64 = core::f64::consts::PI;

                let arg0:LisperExp = LisperExp::Number(arg0_f64);
        
                if let LisperExp::Number(res) = f(&LisperExp::List(vec![arg0])) {
                    assert_eq!(res, arg0_f64.cos());
                } else {
                    assert!(false);
                }
            },
            _ => assert!(false)
        }

        Ok(())
    }

    #[test]
    fn create_default_env_tan() -> Result<(),  Box<dyn std::error::Error>> {
        use super::*;
        
        let env:LisperEnv = create_default_env();

        let func = env.data.get("tan").ok_or(
            LisperErr::Reason("Error, function not found.".to_string())
        )?;

        match func {
            LisperExp::Func(f) => {
                let arg0_f64: f64 = core::f64::consts::PI;

                let arg0:LisperExp = LisperExp::Number(arg0_f64);
        
                if let LisperExp::Number(res) = f(&LisperExp::List(vec![arg0])) {
                    assert_eq!(res, arg0_f64.tan());
                } else {
                    assert!(false);
                }
            },
            _ => assert!(false)
        }

        Ok(())
    }

    #[test]
    fn eval_def() -> Result<(), Box<dyn std::error::Error>> {
        use super::*;

        // Test if eval handles def
        // Format: (def variable_name[as LisperExp::Symbol] (value[as LisperExp]))

        let def_exp:LisperExp = LisperExp::List(vec![
            LisperExp::Symbol("def".to_string()),
            LisperExp::Symbol("a".to_string()),
            LisperExp::Number(1.0)
        ]);

        let add_exp:LisperExp = LisperExp::List(vec![
            LisperExp::Symbol("+".to_string()),
            LisperExp::Symbol("a".to_string()),
            LisperExp::Number(1.0)
        ]);
        
        let mut env:LisperEnv = create_default_env();

        if let LisperExp::Number(_) = eval(def_exp, &mut env)? {
            if let LisperExp::Number(res) = eval(add_exp, &mut env)? {
                assert_eq!(2.0, res);
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        Ok(())
    }
}