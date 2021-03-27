use std::collections::HashMap;
use crate::exp::LisperExp;
//  Represents the context where a Lisp expression executes
#[derive(Clone)]
pub struct LisperEnv {
    pub data: HashMap<String, LisperExp>
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
    LisperExp::Number(sum)
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
    LisperExp::Number(sum)
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
    LisperExp::Number(sum)
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
    LisperExp::Number(sum)
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
    LisperExp::Number(sum)
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
    LisperExp::Bool(res)
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
    LisperExp::Bool(res)
}

#[allow(clippy::float_cmp)]
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
    LisperExp::Bool(res)
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
    LisperExp::Bool(res)
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
    LisperExp::Bool(res)
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
    fn create_default_env_add() -> Result<(),  Box<dyn std::error::Error>> {
        use super::*;
        use crate::core::LisperErr;

        let env:LisperEnv = create_default_env();

        let func = env.data.get("+").ok_or_else(|| 
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
        use crate::core::LisperErr;
        
        let env:LisperEnv = create_default_env();

        let func = env.data.get("-").ok_or_else(|| 
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
        use crate::core::LisperErr;
        
        let env:LisperEnv = create_default_env();

        let func = env.data.get("*").ok_or_else(|| 
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
        use crate::core::LisperErr;
        
        let env:LisperEnv = create_default_env();

        let func = env.data.get("/").ok_or_else(|| 
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
        use crate::core::LisperErr;
        
        let env:LisperEnv = create_default_env();

        let func = env.data.get("%").ok_or_else(|| 
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
        use crate::core::LisperErr;
        
        let env:LisperEnv = create_default_env();

        let func = env.data.get("<").ok_or_else(|| 
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
        use crate::core::LisperErr;
        
        let env:LisperEnv = create_default_env();

        let func = env.data.get(">").ok_or_else(|| 
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
        use crate::core::LisperErr;
        
        let env:LisperEnv = create_default_env();

        let func = env.data.get("=").ok_or_else(|| 
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
        use crate::core::LisperErr;
        
        let env:LisperEnv = create_default_env();

        let func = env.data.get("<=").ok_or_else(|| 
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
        use crate::core::LisperErr;
        
        let env:LisperEnv = create_default_env();

        let func = env.data.get(">=").ok_or_else(|| 
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
        use crate::core::LisperErr;
        
        let env:LisperEnv = create_default_env();

        let func = env.data.get("sin").ok_or_else(|| 
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
        use crate::core::LisperErr;
        
        let env:LisperEnv = create_default_env();

        let func = env.data.get("cos").ok_or_else(|| 
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
        use crate::core::LisperErr;
        
        let env:LisperEnv = create_default_env();

        let func = env.data.get("tan").ok_or_else(|| 
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
}