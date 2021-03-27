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