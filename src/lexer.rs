use std::{
    fmt::{self, Debug, Display, Formatter},
    io::{BufRead, BufReader, Read},
    slice::Iter,
};

use crate::{util::IntoChars, Error, Result};

pub enum Command {
    Increment,
    Decrement,
    Left,
    Right,
    Output,
    Input,
    Loop(Commands),
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Command::Increment => write!(f, "+"),
            Command::Decrement => write!(f, "-"),
            Command::Left => write!(f, "<"),
            Command::Right => write!(f, ">"),
            Command::Output => write!(f, "."),
            Command::Input => write!(f, ","),
            Command::Loop(body) => write!(f, "[{}]", body),
        }
    }
}

impl Debug for Command {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

pub struct Commands {
    body: Vec<Command>,
}

impl Commands {
    fn new() -> Self {
        Self { body: Vec::new() }
    }
    pub fn add(&mut self, c: Command) {
        self.body.push(c)
    }
    pub fn iter(&self) -> Iter<'_, Command> {
        self.body.iter()
    }
}

impl Display for Commands {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.body.iter().fold(Ok(()), |result, command| {
            result.and_then(|_| write!(f, "{}", command))
        })
    }
}

impl Debug for Commands {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

pub fn parse(input: &mut dyn Read) -> Result<Commands> {
    let input = BufReader::new(input);
    parse_next(
        &mut input
            .lines()
            .flat_map(|result| match result {
                Ok(l) => IntoChars::new(l).collect(),
                Err(e) => vec![Err(Error::InputFailure(e))],
            })
            .into_iter(),
        false,
    )
}

fn parse_next(iter: &mut dyn Iterator<Item = Result<char>>, inner: bool) -> Result<Commands> {
    let mut ret = Commands::new();
    while let Some(c) = iter.next() {
        match &c? {
            '+' => ret.add(Command::Increment),
            '-' => ret.add(Command::Decrement),
            '<' => ret.add(Command::Left),
            '>' => ret.add(Command::Right),
            '.' => ret.add(Command::Output),
            ',' => ret.add(Command::Input),
            '[' => ret.add(Command::Loop(parse_next(iter, true)?)),
            ']' => {
                return if !inner {
                    Err(Error::UnexpectedClosingBracket)
                } else {
                    Ok(ret)
                }
            }
            _ => (),
        }
    }
    if inner {
        Err(Error::MissingClosingBracket)
    } else {
        Ok(ret)
    }
}
