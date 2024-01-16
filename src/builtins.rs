//! Contains all the built-in functions for callisp.

use crate::ast::{Ast, LispAtom, LispCallable, LispType};
use crate::env::Environment;
use crate::error::LispError;
use crate::{eval, parser};

use std::collections::HashMap;
use std::fmt::Debug;

macro_rules! fn_map {
    ($($name:literal => $func:ident),+ ,) => {
        {
            let mut map: HashMap<String, Ast> = HashMap::new();
            $(
                map.insert($name.to_string(), Ast::Function(Box::new($func)));
            )+
            map
        }
    };
}

pub(crate) fn builtins_hashmap() -> HashMap<String, Ast> {
    fn_map! {
        "+" => LISP_ADD,
        "-" => LISP_SUB,
        "*" => LISP_MUL,
        "/" => LISP_DIV,
        "eval" => LISP_EVAL,
        "exit" => LISP_EXIT,
        "use" => LISP_USE,
        "putstr" => LISP_PUT_STR,
        "readline" => LISP_READ_LINE,
        "equal?" => LISP_EQUAL,
        ">" => LISP_GT,
        ">=" => LISP_GE,
        "<" => LISP_LT,
        "<=" => LISP_LE,
        "list" => LISP_LIST,
        "list?" => LISP_IS_LIST,
        "empty?" => LISP_IS_EMPTY,
        "count" => LISP_COUNT,
        "type" => LISP_GET_TYPE,
    }
}

fn ast_to_int(ast: &Ast) -> Result<i64, LispError> {
    match ast {
        Ast::Atom(LispAtom::Int(num)) => Ok(*num),
        _ => Err(LispError::TypeError),
    }
}

fn ast_to_float(ast: &Ast) -> Result<f64, LispError> {
    match ast {
        Ast::Atom(LispAtom::Float(num)) => Ok(*num),
        Ast::Atom(LispAtom::Int(num)) => Ok(*num as f64),
        _ => Err(LispError::TypeError),
    }
}

fn ast_to_string(ast: Ast) -> Result<String, LispError> {
    match ast {
        Ast::Atom(LispAtom::String(string)) => Ok(string),
        _ => Err(LispError::TypeError),
    }
}

fn take_first(items: Vec<Ast>) -> Result<Ast, LispError> {
    items.into_iter().next().ok_or(LispError::BadArity)
}

fn get_first(items: &[Ast]) -> Result<&Ast, LispError> {
    items.get(0).ok_or(LispError::BadArity)
}

fn to_list_of_floats(args: Vec<Ast>) -> Result<Vec<f64>, LispError> {
    args.iter().map(|ast| ast_to_float(ast)).collect()
}

fn to_list_of_ints(args: Vec<Ast>) -> Result<Vec<i64>, LispError> {
    args.iter().map(|ast| ast_to_int(ast)).collect()
}

fn one_or_zero(num_args: usize) -> bool {
    return num_args <= 1;
}

fn exactly_zero(num_args: usize) -> bool {
    return num_args == 0;
}

fn exactly_one(num_args: usize) -> bool {
    return num_args == 1;
}

fn at_least_zero(_num_args: usize) -> bool {
    return true;
}

fn at_least_one(num_args: usize) -> bool {
    return num_args >= 1;
}

fn at_least_two(num_args: usize) -> bool {
    return num_args >= 2;
}

/// A Lisp builtin function.
/// TODO: Also used for Rust modules so should probably change name.
#[derive(Clone)]
pub struct LispBuiltin {
    arity: fn(usize) -> bool,
    func: fn(Vec<Ast>, &mut Environment) -> Result<Ast, LispError>,
}

impl Debug for LispBuiltin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Implementation could probably be better
        write!(f, "builtin function")
    }
}

impl LispCallable for LispBuiltin {
    fn arity(&self, num_args: usize) -> bool {
        (self.arity)(num_args)
    }

    fn call(&self, args: Vec<Ast>, env: &mut Environment) -> Result<Ast, LispError> {
        (self.func)(args, env)
    }
}

const LISP_EXIT: LispBuiltin = LispBuiltin {
    arity: one_or_zero,
    func: |args, _env| {
        let code = match args.into_iter().next() {
            Some(code) => ast_to_int(&code)? as i32,
            None => 0,
        };

        std::process::exit(code);
    },
};

const LISP_EVAL: LispBuiltin = LispBuiltin {
    arity: exactly_one,
    func: |args, env| eval::eval_expr(take_first(args)?, env),
};

const LISP_ADD: LispBuiltin = LispBuiltin {
    arity: at_least_one,
    func: |args, _env| {
        if args.len() > 1 {
            let all_ints = args.iter().fold(true, |acc, ast| {
                acc && matches!(ast, Ast::Atom(LispAtom::Int(_)))
            });
            if all_ints {
                let sum = to_list_of_ints(args)?.into_iter().sum();
                Ok(Ast::Atom(LispAtom::Int(sum)))
            } else {
                let sum = to_list_of_floats(args)?.into_iter().sum();
                Ok(Ast::Atom(LispAtom::Float(sum)))
            }
        } else {
            let arg = take_first(args)?;
            if matches!(
                arg,
                Ast::Atom(LispAtom::Float(_)) | Ast::Atom(LispAtom::Int(_))
            ) {
                Ok(arg)
            } else {
                Err(LispError::TypeError)
            }
        }
    },
};

const LISP_SUB: LispBuiltin = LispBuiltin {
    arity: at_least_one,
    func: |args, _env| {
        let difference = if args.len() > 1 {
            to_list_of_floats(args)?
                .into_iter()
                .reduce(|acc, num| acc - num)
                .ok_or(LispError::BadArity)?
        } else {
            -1.0 * get_first(&args).and_then(ast_to_float)?
        };

        Ok(Ast::Atom(LispAtom::Float(difference)))
    },
};

const LISP_MUL: LispBuiltin = LispBuiltin {
    arity: at_least_one,
    func: |args, _env| {
        let product = to_list_of_floats(args)?
            .into_iter()
            .reduce(|acc, num| acc * num)
            .ok_or(LispError::BadArity)?;

        Ok(Ast::Atom(LispAtom::Float(product)))
    },
};

const LISP_DIV: LispBuiltin = LispBuiltin {
    arity: at_least_one,
    func: |args, _env| {
        let quotient = if args.len() > 1 {
            to_list_of_floats(args)?
                .into_iter()
                .reduce(|acc, num| acc / num)
                .ok_or(LispError::BadArity)?
        } else {
            1.0 / get_first(&args).and_then(ast_to_float)?
        };

        Ok(Ast::Atom(LispAtom::Float(quotient)))
    },
};

const LISP_USE: LispBuiltin = LispBuiltin {
    arity: exactly_one,
    func: |args, env| {
        let file = take_first(args).and_then(ast_to_string)?;
        // convert file to string
        let contents = std::fs::read_to_string(file).map_err(|_| LispError::IOError)?;
        let mut to_parse = contents.as_str();
        let mut res = Ast::List(vec![]);

        while let Ok((rest, expr)) = parser::parse_expr(to_parse) {
            to_parse = rest;
            res = eval::eval_expr(expr, env)?;
        }

        Ok(res)
    },
};

// TODO: be able to write to any file (not just stdout).
const LISP_PUT_STR: LispBuiltin = LispBuiltin {
    arity: exactly_one,
    func: |args, _env| {
        let string = take_first(args).and_then(ast_to_string)?;
        println!("{}", string);
        Ok(Ast::Unspecified)
    },
};

const LISP_READ_LINE: LispBuiltin = LispBuiltin {
    arity: exactly_zero,
    func: |_args, _env| {
        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .map_err(|_| LispError::IOError)?;
        // TODO: This might break compatibility with windows
        buf.pop(); // Remove trailing '\n'
        Ok(Ast::Atom(LispAtom::String(buf)))
    },
};

const LISP_EQUAL: LispBuiltin = LispBuiltin {
    arity: at_least_two,
    func: |args, _env| {
        let mut iter = args.into_iter();
        let first = iter.next().ok_or(LispError::BadArity)?;
        for arg in iter {
            if arg != first {
                return Ok(Ast::Atom(LispAtom::Bool(false)));
            }
        }

        Ok(Ast::Atom(LispAtom::Bool(true)))
    },
};

const LISP_GT: LispBuiltin = LispBuiltin {
    arity: at_least_two,
    func: |args, _env| {
        let nums = to_list_of_floats(args)?;
        let mut result = true;
        for i in 0..nums.len() - 1 {
            result &= nums[i] > nums[i + 1];
        }

        Ok(Ast::Atom(LispAtom::Bool(result)))
    },
};

const LISP_GE: LispBuiltin = LispBuiltin {
    arity: at_least_two,
    func: |args, _env| {
        let nums = to_list_of_floats(args)?;
        let mut result = true;
        for i in 0..nums.len() - 1 {
            result &= nums[i] >= nums[i + 1];
        }

        Ok(Ast::Atom(LispAtom::Bool(result)))
    },
};

const LISP_LT: LispBuiltin = LispBuiltin {
    arity: at_least_two,
    func: |args, _env| {
        let nums = to_list_of_floats(args)?;
        let mut result = true;
        for i in 0..nums.len() - 1 {
            result &= nums[i] < nums[i + 1];
        }

        Ok(Ast::Atom(LispAtom::Bool(result)))
    },
};

const LISP_LE: LispBuiltin = LispBuiltin {
    arity: at_least_two,
    func: |args, _env| {
        let nums = to_list_of_floats(args)?;
        let mut result = true;
        for i in 0..nums.len() - 1 {
            result &= nums[i] <= nums[i + 1];
        }

        Ok(Ast::Atom(LispAtom::Bool(result)))
    },
};

const LISP_LIST: LispBuiltin = LispBuiltin {
    arity: at_least_zero,
    func: |args, _env| Ok(Ast::List(args)),
};

const LISP_IS_LIST: LispBuiltin = LispBuiltin {
    arity: exactly_one,
    func: |args, _env| {
        let is_list = matches!(get_first(&args)?, Ast::List(_));
        Ok(Ast::Atom(LispAtom::Bool(is_list)))
    },
};

const LISP_IS_EMPTY: LispBuiltin = LispBuiltin {
    arity: exactly_one,
    func: |args, _env| {
        let arg = get_first(&args)?;
        let is_empty = match arg {
            Ast::List(items) => !items.is_empty(),
            _ => false,
        };

        Ok(Ast::Atom(LispAtom::Bool(is_empty)))
    },
};

const LISP_COUNT: LispBuiltin = LispBuiltin {
    arity: exactly_one,
    func: |args, _env| {
        let arg = get_first(&args)?;
        let length = match arg {
            Ast::List(items) => items.len(),
            _ => return Err(LispError::TypeError),
        };

        Ok(Ast::Atom(LispAtom::Float(length as f64)))
    },
};

const LISP_GET_TYPE: LispBuiltin = LispBuiltin {
    arity: exactly_one,
    func: |args, _env| {
        let arg = get_first(&args)?;
        Ok(Ast::Type(LispType::from(arg)))
    },
};
