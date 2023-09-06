use std::{
    io::{ErrorKind, Read, Write},
    ops::{Index, IndexMut},
    slice::Iter,
};

use crate::{
    lexer::{Command, Commands},
    Error, Result,
};

const DEFAULT_MEM_SIZE: usize = 30_000;

#[derive(Debug)]
struct Memory {
    mem: Vec<u8>,
    capacity: usize,
    pointer: usize,
}

#[derive(Debug)]
pub struct Interpreter<'a> {
    program: &'a Commands,
    stack: Vec<(&'a Commands, Iter<'a, Command>)>,
    mem: Memory,
}

impl Index<usize> for Memory {
    type Output = u8;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        self.mem.index(index)
    }
}

impl IndexMut<usize> for Memory {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.mem.index_mut(index)
    }
}

impl Memory {
    fn new() -> Self {
        Self::with_size(DEFAULT_MEM_SIZE)
    }
    fn with_size(capacity: usize) -> Self {
        Self {
            mem: vec![0; capacity],
            capacity: capacity,
            pointer: 0,
        }
    }
    fn get(&self) -> &u8 {
        &self.mem[self.pointer]
    }
    fn get_mut(&mut self) -> &mut u8 {
        &mut self.mem[self.pointer]
    }
    fn left(&mut self) {
        self.pointer = (self.pointer + self.capacity - 1) % self.capacity;
    }
    fn right(&mut self) {
        self.pointer = (self.pointer + 1) % self.capacity;
    }
}

impl<'a> Interpreter<'a> {
    pub fn new(program: &'a Commands) -> Self {
        Self {
            program,
            stack: Vec::new(),
            mem: Memory::new(),
        }
    }
    #[allow(dead_code)]
    pub fn with_size(program: &'a Commands, capacity: usize) -> Self {
        Self {
            program,
            stack: Vec::new(),
            mem: Memory::with_size(capacity),
        }
    }
    pub fn run(&mut self, input: &'a mut dyn Read, output: &'a mut dyn Write) -> Result<()> {
        self.stack.push((&self.program, self.program.iter()));
        while let Some((_, i)) = self.stack.last_mut() {
            if let Some(command) = i.next() {
                match command {
                    Command::Increment => *self.mem.get_mut() = self.mem.get().wrapping_add(1),
                    Command::Decrement => *self.mem.get_mut() = self.mem.get().wrapping_sub(1),
                    Command::Left => self.mem.left(),
                    Command::Right => self.mem.right(),
                    Command::Output => {
                        output
                            .write(&[*self.mem.get()])
                            .or_else(|e| Err(Error::OutputFailure(e)))?;
                    }
                    Command::Input => {
                        *self.mem.get_mut() = {
                            let mut buf = [0u8];
                            input.read_exact(&mut buf).or_else(|e| {
                                if e.kind() == ErrorKind::UnexpectedEof {
                                    Ok(())
                                } else {
                                    Err(Error::InputFailure(e))
                                }
                            })?;
                            buf[0]
                        }
                    }
                    Command::Loop(body) => {
                        if *self.mem.get() != 0 {
                            self.stack.push((&body, body.iter()));
                        }
                    }
                }
            } else {
                // loop end
                if self.stack.len() > 1 && *self.mem.get() != 0 {
                    let (commands, i) = self.stack.last_mut().expect("lenth checked");
                    *i = commands.iter();
                } else {
                    self.stack.pop();
                }
            }
        }
        Ok(())
    }
}
