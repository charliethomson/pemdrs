use crate::tree::evaluate;
use crate::token::{ Token, tokenize };
use std::fmt::{ Display, Result as fmt_Result, Formatter };

enum ReplaceError {
    ParseError,
    MismatchedLengths,
}

fn replace_all(src: &Vec<String>, dst: &Vec<String>, code: &String) -> Result<String, ReplaceError> {
    let mut code = code.clone();
    if src.len() != dst.len() {
        return Err(ReplaceError::MismatchedLengths);
    }
    for (s, d) in src.iter().zip(dst.iter()) {
        match d.parse::<f64>() {
            Err(e) => return Err(ReplaceError::ParseError),
            _ => {}
        }
        code = code.replace(s, d);
    }
    Ok(code)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub(crate) ident: String,
    pub(crate) args: Vec<String>,
    pub(crate) code: Vec<Token>,
    pub(crate) local_idents: Vec<Token>,
} impl Function {
    pub fn new(src: String) -> Result<Self, String> {
        let mut spl = src.split_ascii_whitespace().map(|s| s.to_owned());
        match spl.next() {
            Some(kw) => {
                if kw.to_lowercase() == "function" {
                    let ident = spl.next().expect(&format!("Failed to get ident from {}", src));
                    let mut args = Vec::new();
                    while let Some(arg) = spl.next() {
                        if arg == "=" {
                            break;
                        } else {
                            args.push(arg);
                        }
                    }

                    let code = &spl.collect::<Vec<String>>().join(" ");
                    eprintln!("aaa{:?}", code);
                    // let code = tokenize(&src)?;

                    // return Ok(Function {
                    //     ident,
                    //     args,
                    //     code,
                    //     local_idents,
                    // });

                } else {
                    unreachable!()
                }
            },
            None => {

            }
        }
    
        Err("".to_owned())
    }

    pub fn call(&self, args: Vec<String>) -> Result<String, String> {
        let args_copy = args.clone();
        let mut args = args;
        // check for builtins
        if let Some(Token::Data(d)) = self.code.first() {
            if d.starts_with("##") {
                match d.get(2..).expect("What happened.") {
                    "sin" => {
                        match args.pop() {
                            Some(arg) => {
                                match arg.parse::<f64>() {
                                    Ok(float) => Ok(format!("{}", float.sin())),
                                    Err(_) => Err(format!("encountered non number argument in call to {}", self.ident))
                                }
                            },
                            None => Err("Expected 1 argument, recieved 0".to_owned()),
                        }
                    },
                    "cos" => {
                        match args.pop() {
                            Some(arg) => {
                                match arg.parse::<f64>() {
                                    Ok(float) => Ok(format!("{}", float.cos())),
                                    Err(_) => Err(format!("encountered non number argument in call to {}", self.ident))
                                }
                            },
                            None => Err("Expected 1 argument, recieved 0".to_owned()),
                        }
                    },
                    "tan" => {
                        match args.pop() {
                            Some(arg) => {
                                match arg.parse::<f64>() {
                                    Ok(float) => Ok(format!("{}", float.tan())),
                                    Err(_) => Err(format!("encountered non number argument in call to {}", self.ident))
                                }
                            },
                            None => Err("Expected 1 argument, recieved 0".to_owned()),
                        }
                    },
                    "asin" => {
                        match args.pop() {
                            Some(arg) => {
                                match arg.parse::<f64>() {
                                    Ok(float) => Ok(format!("{}", float.asin())),
                                    Err(_) => Err(format!("encountered non number argument in call to {}", self.ident))
                                }
                            },
                            None => Err("Expected 1 argument, recieved 0".to_owned()),
                        }
                    },
                    "acos" => {
                        match args.pop() {
                            Some(arg) => {
                                match arg.parse::<f64>() {
                                    Ok(float) => Ok(format!("{}", float.acos())),
                                    Err(_) => Err(format!("encountered non number argument in call to {}", self.ident))
                                }
                            },
                            None => Err("Expected 1 argument, recieved 0".to_owned()),
                        }
                    },
                    "atan" => {
                        match args.pop() {
                            Some(arg) => {
                                match arg.parse::<f64>() {
                                    Ok(float) => Ok(format!("{}", float.atan())),
                                    Err(_) => Err(format!("encountered non number argument in call to {}", self.ident))
                                }
                            },
                            None => Err("Expected 1 argument, recieved 0".to_owned()),
                        }
                    },
                    "sum" => {
                        Ok(format!("{}", args.iter().map(|s| s.parse::<f64>().unwrap()).sum::<f64>()))
                    },
                    "min" => {
                        match (args.pop(), args.pop()) {
                            (Some(a), Some(b)) => {
                                Ok(format!("{}", a.min(b)))
                            },
                            _ => {
                                Err(format!("Expected 2 arguments, recieved {}", args_copy.len()))
                            }
                        }
                    },
                    "max" => {
                        match (args.pop(), args.pop()) {
                            (Some(a), Some(b)) => {
                                Ok(format!("{}", a.max(b)))
                            },
                            _ => {
                                Err(format!("Expected 2 arguments, recieved {}", args_copy.len()))
                            }
                        }
                    },
                    _ => {
                        Err(format!("Unknown function {:?}", self.code))
                    }
                }
            } else {
                // let code = match replace_all(&self.args, &args, &self.code) {
                //     Ok(c) => c,
                //     Err(e) => match e {
                //         ReplaceError::MismatchedLengths => return Err(format!("Expected {} arguments, recieved {}", self.args.len(), args.len())),
                //         ReplaceError::ParseError => {
                //             let err = args.iter().filter(|arg| arg.parse::<f64>().is_err()).cloned().collect::<Vec<String>>();
                //             let plural = if err.len() >= 2 { "agruments" } else { "argument" };
                //             return Err(format!("Encountered non number {}: {}", plural, err.join(", ")))
                //         },
                //     },
                // };
                // match evaluate(code.clone()) {
                //     Ok(v) => Ok(format!("{}", v)),
                //     Err(e) => Err(format!("Failed to evaluate {}: {}", code, e)), 
                // }
                Ok(String::new())
            }
        } else {
            // let code = match replace_all(&self.args, &args, &self.code) {
            //     Ok(c) => c,
            //     Err(e) => match e {
            //         ReplaceError::MismatchedLengths => return Err(format!("Expected {} arguments, recieved {}", self.args.len(), args.len())),
            //         ReplaceError::ParseError => {
            //             let err = args.iter().filter(|arg| arg.parse::<f64>().is_err()).cloned().collect::<Vec<String>>();
            //             let plural = if err.len() >= 2 { "agruments" } else { "argument" };
            //             return Err(format!("Encountered non number {}: {}", plural, err.join(", ")))
            //         },
            //     },
            // };
            // match evaluate(code.clone()) {
            //     Ok(v) => Ok(format!("{}", v)),
            //     Err(e) => Err(format!("Failed to evaluate {}: {}", code, e)), 
            // }
            Ok(String::new())
        }
    }

    pub fn argc(&self) -> usize {
        self.args.len()
    }
} impl Display for Function {
    fn fmt(&self, f: &mut Formatter) -> fmt_Result {
        write!(f, "{} {} = {:?}", self.ident, self.args.join(" "), self.code)
    }
}


#[test]
fn test_function_new() {
    std::thread::sleep_ms(100);
    let sin = Function::new("function sin a = ##sin".to_owned()).unwrap();
    let foo = Function::new("function foo a b = 2 * a + b".to_owned()).unwrap();
    eprintln!("{:?}", sin.code);
    eprintln!("{:?}", foo.code);
}