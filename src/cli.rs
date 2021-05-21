use tryst::{read};
use rustyline::{self, error::ReadlineError};

//fn rep(input: &str) -> &str { print(eval(read(input))) }

fn run(prompt: &str) {
    let mut rl = rustyline::Editor::<()>::with_config(
        rustyline::Config::builder()
            .auto_add_history(true)
            .build()
    );

    for line in rl.iter(prompt) {
        match line {
            Ok(line) => println!("{:?}", read(&line)),
            Err(ReadlineError::Eof) => return,
            Err(e) => println!("{:?}", e)
        }
    }
}

fn main() {
    run(">> ");
}
