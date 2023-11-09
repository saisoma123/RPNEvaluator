/*
 * Reverse Polish Notation: rpn.rs
 * See `rpn.md` for the overview.
 */

use std::io;
use rand::Rng;
     

// Stacks will work with Items, which either either integers or booleans
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Item {
    Int(i32),
    Bool(bool),
}

// List of possible errors
#[derive(Debug)]
pub enum Error {
    Empty,         // Tried to pop empty stack
    Extra,         // Stack ended with extra 0
    Type,          // Type mismatch
    Syntax,        // Syntax error, didn't recognize op
    IO(io::Error), // Some kind of IO error
    Quit,          // User quitting
}

// Base operations supported by calculator, see rpn.md
#[derive(Debug)]
pub enum Op {
    Add,
    Eq,
    Neg,
    Swap,
    Rand,
    Cond,
    Quit,
}

// We'll define a result type for our calculator: either a valid value, or a calculator Error
pub type Result<T> = std::result::Result<T, Error>;

// Define a type for Stacks
#[derive(Debug)]
pub struct Stack(Vec<Item>);

// Implement the following functions on Stacks
impl Stack {
    // Make a new Stack
    pub fn new() -> Self {
        Stack(Vec::new())
    }

    // Check if a Stack is empty
    pub fn empty(&self) -> bool {
        if self.0.len() == 0 {
            return true
        }
        return false
    }

    // Push an item onto a stack (should never error)
    pub fn push(&mut self, item: Item) -> Result<()> {
        self.0.push(item);
        Ok(())
    }

    // Pop an item off the Stack; may result in Empty error
    pub fn pop(&mut self) -> Result<Item> {
        if self.empty() {
            Err(Error::Empty)
        } else {
            self.0.pop().ok_or(Error::Empty)
        }
    }
    

    /*
     * Main evaluation function: apply an operation to a Stack
     *
     * Complete each of the cases. 
     *
     * Hint: You'll probably want to use the "question-mark" syntax quite a bit; see `rpn.md`.
     */
     
     pub fn eval(&mut self, op: Op) -> Result<()> {
        match op {
            Op::Add => {
                if self.0.len() < 2 {
                    return Err(Error::Empty);
                }
                let b = self.pop()?;
                let a = self.pop()?;
                match (a, b) {
                    (Item::Int(x), Item::Int(y)) => self.push(Item::Int(x + y)),
                    _ => Err(Error::Type),
                }
            },
            Op::Eq => {
                if self.0.len() < 2 {
                    return Err(Error::Empty);
                }
                let b = self.pop()?;
                let a = self.pop()?;
                match (a, b) {
                    (Item::Int(x), Item::Int(y)) => self.push(Item::Bool(x == y)),
                    (Item::Bool(x), Item::Bool(y)) => self.push(Item::Bool(x == y)),
                    _ => Err(Error::Type),
                }
            },
            Op::Neg => {
                if self.empty() {
                    return Err(Error::Empty);
                }
                let value = self.pop()?;
                match value {
                    Item::Bool(b) => self.push(Item::Bool(!b)),
                    _ => Err(Error::Type),
                }
            },
            Op::Swap => {
                if self.0.len() < 2 {
                    return Err(Error::Empty);
                }
                let b = self.pop()?;
                let a = self.pop()?;
                self.push(b)?;
                self.push(a)
            },
            Op::Rand => {
                if self.empty() {
                    return Err(Error::Empty);
                }
                
                match self.pop()? {
                    Item::Int(max) if max > 0 => {
                        let mut rng = rand::thread_rng();
                        let number = rng.gen_range(0,max);
                        // Push the generated number onto the stack
                        self.push(Item::Int(number))
                    },
                    _ => Err(Error::Type),
                }
            },            
            Op::Cond => {
                if self.0.len() < 3 {
                    return Err(Error::Empty);
                }
                let c = self.pop()?;
                let b = self.pop()?;
                let a = self.pop()?;
                match a {
                    Item::Bool(cond) => {
                        if cond {
                            self.push(b)
                        } else {
                            self.push(c)
                        }
                    }
                    _ => Err(Error::Type),
                }
            },
            Op::Quit => Err(Error::Quit),
        }
    }
    
    
}
