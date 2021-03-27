use std::fmt;
use std::error;

use crate::exp::LisperExp;
use crate::env::{LisperEnv};
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
pub fn parse(tokens: &[String]) -> Result<(LisperExp, &[String]), LisperErr> {
    let (first, rest) = tokens.split_first()
        .ok_or_else(|| LisperErr::Reason("Could not get token".to_string()))?;

    let mut parsed_result: Vec<LisperExp> = vec![];

    match first.as_str() {
        "(" => {
            let mut more = rest;
            loop {
                let (next, more_next) = more.split_first()
                    .ok_or_else(|| 
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
            Err(LisperErr::Reason("Parsing error, found unexpected ).".to_string()))
        },
        _ => {
            let parsed_token:LisperExp = parse_token(&first);
            Ok((parsed_token, rest))
        }
    }
}

// Parses an individual token and creates either a Number of Symbol LisperExp
fn parse_token(token: &str) -> LisperExp {
    if let Result::Ok(parsed_bool) = token.parse::<bool>() {
        LisperExp::Bool(parsed_bool)
    } else if let Result::Ok(parsed_value) = token.parse::<f64>() {
        LisperExp::Number(parsed_value)
    } else {
        LisperExp::Symbol(token.to_string())
    }
}

// Evaluates a given Lisp expression, and returns a new one with the result.
pub fn eval(exp: LisperExp, env: &mut LisperEnv) -> Result<LisperExp, LisperErr> {
    match exp {
        LisperExp::List(list) => {
            eval_list(list, env)
        },
        LisperExp::Number(num) => {
            // If it's just a number, then return the number
            Ok(LisperExp::Number(num))
        },
        LisperExp::Symbol(sym) => {
            let lisper_exp = env.data.get(&sym)
            .ok_or_else(|| 
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
        LisperExp::Lambda(_) => Err(LisperErr::Reason("Unexpected lambda function".to_string())),
    }
}

// Evaluates a list of Lisp expressions, and returns a new one with the result.
fn eval_list(list: Vec<LisperExp>, env: &mut LisperEnv) -> Result<LisperExp, LisperErr> {
    // Split the symbol from the arguments
    let (first, args) = list.split_first()
        .ok_or_else(|| 
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
                    // Format: (if (expression[as LisperExp]) (if true[as LisperExp]) (if false[as LisperExp]))
                    
                    if args.len() != 3 {
                        Err(LisperErr::Reason("Syntax error, if only takes 3 arguments, if expression, true expression, and false expression.".to_string()))
                    } else {
                        let if_exp:LisperExp = eval(args[0].clone(), env)?;
                        match if_exp {
                            LisperExp::Bool(res) => {
                                if res {
                                    Ok(eval(args[1].clone(), env)?)
                                } else {
                                    Ok(eval(args[2].clone(), env)?)
                                }
                            },
                            LisperExp::Number(res) => {
                                if res > 0.0 {
                                    Ok(eval(args[1].clone(), env)?)
                                } else {
                                    Ok(eval(args[2].clone(), env)?)
                                }
                            },
                            _ => Err(LisperErr::Reason("If statement invalid.".to_string()))
                        }
                    }
                },
                "def" => {
                    // It's a variable definition
                    // Format: (def variable_name[as string] (value[as LisperExp]))

                    // TODO: Figure out if we should block over-writing predefined constants here
                    if args.len() != 2 {
                        Err(LisperErr::Reason("Syntax error, def only takes 2 arguments, name and an expression.".to_string()))
                    } else {
                        let variable_name:String = args[0].to_string();
                        let variable_value:LisperExp = eval(args[1].clone(), env)?;
                        
                        env.data.insert(variable_name, variable_value.clone());
    
                        Ok(variable_value)
                    }

                },
                "fn" => {
                    // It's a function definition
                    // Format: (fn function_name[as string] (argument[as LisperExp list]) (function[as LisperExp]))
                    if args.len() != 3 {
                        Err(LisperErr::Reason("Syntax error, fn only takes 3 arguments: function name, argument name, and function expression.".to_string()))
                    } else {
                        let fn_name:String = args[0].to_string();
                        let fn_arg:String = args[1].to_string();
                        let fn_exp:LisperExp = args[2].clone();
    
                        let fn_lisper_exp = LisperExp::Lambda(vec![
                            LisperExp::Symbol(fn_arg),
                            fn_exp
                        ]);
    
                        env.data.insert(fn_name, fn_lisper_exp);
    
                        Ok(LisperExp::Bool(true))
                    }
                },
                _ => {
                    // Get the env function based on the symbol
                    // Run the function with the args, and return the result
                    let func = env.data.get(&sym.to_string()).ok_or_else(|| 
                            LisperErr::Reason("Error, env function not found.".to_string())
                        )?.clone();
                    match func {
                        LisperExp::Func(lisper_func) => {
                            // It's a env function, so evaluate that
                            // Evaluate each argument
                            let mut evaluated_args: Vec<LisperExp> = vec![];
                            for arg in args.iter() {
                                evaluated_args.push(eval(arg.clone(), env)?)
                            }
                            Ok(lisper_func(&LisperExp::List(evaluated_args)))
                        },
                        LisperExp::Lambda(lambda) => {
                            // It's a lamba function, (fn_name arg_value)
                            if args.len() != 1 {
                                Err(LisperErr::Reason("Syntax error, a fn call only takes 1 argument.".to_string()))
                            } else {
                                // Evaluate value for arg
                                let evaluated_arg = eval(args[0].clone(), env)?;
                                
                                // Create new env to have a new sub-scope
                                let mut sub_env = env.clone();
                                
                                // Set the arg value as a sub_env variable
                                let arg_def:String = lambda[0].to_string();
                                sub_env.data.insert(arg_def, evaluated_arg);
    
                                // Get the lambda expression
                                let fn_exp:LisperExp = lambda[1].clone();
    
                                // Evalute lambda function call in new env and return the result
                                Ok(eval(fn_exp, &mut sub_env)?)
                            }
                        },
                        _ => Err(LisperErr::Reason("Error, function not found.".to_string()))
                    }
                }
            }
        },
        _ => {
            Err(LisperErr::Reason("Parsing error.".to_string()))
        }
    }
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
    fn eval_def() -> Result<(), Box<dyn std::error::Error>> {
        use super::*;
        use crate::env::create_default_env;

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

    #[test]
    fn eval_if() -> Result<(), Box<dyn std::error::Error>> {
        use super::*;
        use crate::env::create_default_env;

        // Test if eval handles if
        // Format: (if (if expression[as LisperExp]) (true expression[as LisperExp]) (false expression[as LisperExp]))

        let if_exp:LisperExp = LisperExp::List(vec![
            LisperExp::Symbol("<".to_string()),
            LisperExp::Number(1.0),
            LisperExp::Number(0.0),
        ]);
        let if_stmnt:LisperExp = LisperExp::List(vec![
            LisperExp::Symbol("if".to_string()),
            if_exp,
            LisperExp::Number(1.0),
            LisperExp::Number(2.0),
        ]);
        
        let mut env:LisperEnv = create_default_env();

        if let LisperExp::Number(res) = eval(if_stmnt, &mut env)? {
            assert_eq!(2.0, res);
        } else {
            assert!(false);
        }

        Ok(())
    }

    #[test]
    fn eval_fn() -> Result<(), Box<dyn std::error::Error>> {
        use super::*;
        use crate::env::create_default_env;

        // Test if eval handles fn
        // Format: (fn function_name[as string] (argument[as LisperExp list]) (function[as LisperExp]))

        let def_fn_exp:LisperExp = LisperExp::List(vec![
            LisperExp::Symbol("fn".to_string()),
            LisperExp::Symbol("add-fn".to_string()),
            LisperExp::Symbol("a".to_string()),
            LisperExp::List(vec![
                LisperExp::Symbol("+".to_string()),
                LisperExp::Symbol("a".to_string()),
                LisperExp::Number(1.0)
            ])
        ]);

        let fn_call_exp:LisperExp = LisperExp::List(vec![
            LisperExp::Symbol("add-fn".to_string()),
            LisperExp::Number(1.0)
        ]);
        
        let mut env:LisperEnv = create_default_env();

        if let LisperExp::Bool(_) = eval(def_fn_exp, &mut env)? {
            if let LisperExp::Number(res) = eval(fn_call_exp, &mut env)? {
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