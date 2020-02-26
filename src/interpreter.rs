
use {
    std::{
        io::{ self, Write },
        collections::HashMap,
        fmt::{ Display, Formatter, Result as fmt_Result },
    },
    crate::{
        function::Function,
        tree::evaluate,
        token::Token,
    },
    regex::{ Regex },
};

fn call_functions(cmd: &String, ctx: &Context) -> Result<Option<String>, String> {
    // This gets the expression from the input.
    /*
    println!("{:?}", cap.get(2).unwrap().as_str().trim().split(" ").map(|s| s.to_owned()).collect::<Vec<String>>());
    assn: var f = 10 + add 2 3
    ["10", "+", "add", "2", "3"]
    expr: 10 + add 2 3
    ["10", "+", "add", "2", "3"]
    fnassn: function foo a = 10 + bar 2 3
    ["10", "+", "bar", "2", "3"]
    */
    let mut cmd = cmd.to_owned();
    let func_call_regex = Regex::new("(.*=|^)(.*)").unwrap();
    if let Some(caps) = func_call_regex.captures(&cmd) {
        match caps.get(2) {
            Some(cap) => {
                // Iterate over all of the tokens in the match
                let mut tokens = cap.as_str().trim().split(" ");
                eprintln!("{}", cap.as_str().trim());
                let mut idx = 0;
                while let Some(token) = tokens.nth(idx) {
                    if let Some(func) = ctx.functions.get(token) {
                        let argc = func.argc();
                        let args = tokens.take_while(|c| !"+-=/*\n".contains(c)).map(|s| s.to_owned()).collect::<Vec<String>>();


                        match call_functions(&args.join(" "), ctx) {
                            Ok(Some(s)) => {
                                cmd = cmd.replace(&args.join(" "), &s);
                            },
                            Err(e) => return Err(e),
                            _ => (),
                        }
                        
                        let output = match func.call(args.clone()) {
                            Ok(o) => o,
                            Err(e) => return Err(format!("Encountered an error evaluating a function: {}", e)),
                        };
                        
                        let func_replace_str = format!("{} {}", func.ident, args.join(" "));

                        println!("func_replace_str: {}", func_replace_str);
                        
                        let func_replace_regex = match Regex::new(&func_replace_str) {
                            Ok(rgx) => rgx,
                            Err(e) => return Err(format!("Encountered an error evaluating a function: {}", e)),
                        };
                        eprintln!("returning {}", func_replace_regex.replace(&cmd, output.as_str()).into_owned());
                        return Ok(Some(func_replace_regex.replace(&cmd, output.as_str()).into_owned()));
                    }

                    idx += 1;
                }

                eprintln!("1");
                Ok(None)
            },
            None => {
                eprintln!("2");
                Ok(None)
            }
        }
    } else {
        eprintln!("3");
        Ok(None)
    }
}

pub struct Interpreter {
    context: Context,
    buffer: Vec<String>,
} impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            context: Context::new(),
            buffer: Vec::new(),
        }
    }

    pub fn begin(mut self) -> Result<(), String> {
        let mut input = io::stdin();
        let mut cmd = String::new();
        let assign_regex = Regex::new(r#"(var(\s)+)(\D\w*)(\d)?(\s)+=(\s)*(.*)"#).unwrap();
        let func_decl_regex = Regex::new(r#"(function(\s)+)(\D\w+)(\d)?(\s)+(\w(\s)+)*(\s)*=(\s)*(.*)"#).unwrap();
        
        'main: loop {
            print!(">> ");
            io::stdout().flush().unwrap();
            input.read_line(&mut cmd);

            if cmd.ends_with('\n') {
                cmd.pop();
            }

            // Code goes here

            // check for function calls
            match call_functions(&cmd, &self.context) {
                Ok(Some(s)) => cmd = s,
                Err(e) => return Err(e),
                _ => (),
            }

            // Check for variable assignment
            if let Some(caps) = assign_regex.captures(&cmd) {
                continue;
            }
            // Check for function declarations
            else if let Some(caps) = func_decl_regex.captures(&cmd) {

            }
            // evaluate the expression
            else {
                // let eval = match evaluate(cmd.clone()) {
                //     Ok(e) => e,
                //     Err(e) => {
                //         println!("Error: {}", e);
                //         continue;
                //     },
                // };
                // self.context.ans = eval;
                // println!("{}", eval);
            }


            //

            cmd = String::new();
        }

        io::stderr().flush();
        io::stdout().flush();

        Ok(())
    }
}

#[derive(Debug)]
pub struct Context {
    pub(crate) functions: HashMap<String, Function>,
    pub(crate) variables: HashMap<String, f64>,
    pub(crate) ans: f64,
} impl Context {
    pub fn new() -> Self {
        let mut functions = HashMap::new();

        functions.insert("sin".to_owned(), Function {
            ident: "sin".to_owned(),
            args: vec!["_".to_owned()],
            code: vec![Token::Data("##sin".to_owned())],
            local_idents: vec![]
        });
        functions.insert("cos".to_owned(), Function {
            ident: "cos".to_owned(),
            args: vec!["_".to_owned()],
            code: vec![Token::Data("##cos".to_owned())],
            local_idents: vec![]
        });
        functions.insert("tan".to_owned(), Function {
            ident: "tan".to_owned(),
            args: vec!["_".to_owned()],
            code: vec![Token::Data("##tan".to_owned())],
            local_idents: vec![]
        });
        functions.insert("asin".to_owned(), Function {
            ident: "asin".to_owned(),
            args: vec!["_".to_owned()],
            code: vec![Token::Data("##asin".to_owned())],
            local_idents: vec![]
        });
        functions.insert("acos".to_owned(), Function {
            ident: "acos".to_owned(),
            args: vec!["_".to_owned()],
            code: vec![Token::Data("##acos".to_owned())],
            local_idents: vec![]
        });
        functions.insert("atan".to_owned(), Function {
            ident: "atan".to_owned(),
            args: vec!["_".to_owned()],
            code: vec![Token::Data("##atan".to_owned())],
            local_idents: vec![]
        });
        functions.insert("min".to_owned(), Function {
            ident: "min".to_owned(),
            args: vec!["_".to_owned(), "_".to_owned()],
            code: vec![Token::Data("##min".to_owned())],
            local_idents: vec![]
        });
        functions.insert("max".to_owned(), Function {
            ident: "max".to_owned(),
            args: vec!["_".to_owned(), "_".to_owned()],
            code: vec![Token::Data("##max".to_owned())],
            local_idents: vec![]
        });
        functions.insert("sum".to_owned(), Function {
            ident: "sum".to_owned(),
            args: vec!["_".to_owned(), "_".to_owned()],
            code: vec![Token::Data("##sum".to_owned())],
            local_idents: vec![]
        });

        Context {
            functions,
            variables: HashMap::new(),
            ans: 0.0,
        }
    }
} impl Display for Context {
    fn fmt(&self, f: &mut Formatter) -> fmt_Result {
        write!(f, "Context:\n\tfunctions: {}\n\tvariables: {}\n\tans: {}", 
            self.functions.iter().map(|(ident, func)| {
                format!("{:?}", func)
            }).collect::<Vec<String>>().join("\n\t\t"),
            self.variables.iter().map(|(ident, val)| {
                format!("var {} = {}", ident, val)
            }).collect::<Vec<String>>().join("\n\t\t"),
            self.ans,
        )
    }
}