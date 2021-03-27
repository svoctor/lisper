use std::fmt;
// Represents an individual Lisp expresion
#[derive(Clone)]
pub enum LisperExp {
    Bool(bool),
    Symbol(String),
    Number(f64),
    List(Vec<LisperExp>),
    Func(fn(&LisperExp) -> LisperExp),
    Lambda(Vec<LisperExp>),
}

// Used for to_string
impl fmt::Display for LisperExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str:String = match self {
            LisperExp::Symbol(s) => s.to_string(),
            LisperExp::Number(n) => n.to_string(),
            LisperExp::Bool(b) => b.to_string(),
            LisperExp::List(list) | LisperExp::Lambda(list) => {
                let items:Vec<String> = list.iter().map(|item| item.to_string()).collect();
                format!("({})", items.join(","))
            },
            LisperExp::Func(_) => "Function".to_string()
        };
        
        write!(f, "{}", str)
    }
}