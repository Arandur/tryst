use rustyline::{self, error::ReadlineError};

pub struct Repl;

impl Repl {
    pub fn new() -> Self { Default::default() }

    fn read<'a>(&self, input: &'a str) -> &'a str {
        input
    }

    fn eval<'a>(&self, input: &'a str) -> &'a str {
        input
    }

    fn print<'a>(&self, input: &'a str) -> &'a str {
        input
    }

    fn rep<'a>(&self, input: &'a str) -> &'a str {
        self.print(self.eval(self.read(input)))
    }

    pub fn run(&self) {
        let mut rl = rustyline::Editor::<()>::with_config(
            rustyline::Config::builder()
                .auto_add_history(true)
                .build()
        );

        for line in rl.iter(">> ") {
            match line {
                Ok(line) => println!("{}", self.rep(&line)),
                Err(ReadlineError::Eof) => return,
                Err(e) => println!("{:?}", e)
            }
        }
    }
}

impl Default for Repl { fn default() -> Self { Repl } }