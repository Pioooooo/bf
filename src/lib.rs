use std::io::{Read, Write};

pub use error::{Error, Result};
pub use interpreter::Interpreter;

mod error;
mod interpreter;
mod lexer;
mod util;

pub fn run(program: &mut dyn Read, input: &mut dyn Read, output: &mut dyn Write) -> Result<()> {
    let program = lexer::parse(program)?;
    let mut interpreter = Interpreter::new(&program);
    interpreter.run(input, output)
}
