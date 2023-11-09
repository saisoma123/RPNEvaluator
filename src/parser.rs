/*
 * Reverse Polish Notation: parser.rs
 * See `rpn.md` for the overview.
 */

extern crate rand;

use std::io::{self, Write};

use super::rpn;

pub fn rpn_repl() -> rpn::Result<()> {
    let mut stack = rpn::Stack::new();
    let mut input = String::new();

    // Read-eval-print loop
    loop {
        // Clear the input buffer
        input.clear();

        // Prompt the user
        print!("> ");
        io::stdout().flush().map_err(rpn::Error::IO)?;

        // Read a line and evaluate it
        io::stdin().read_line(&mut input).map_err(rpn::Error::IO)?;
        evaluate_line(&mut stack, &input)?;

        // A successful run should end with a stack with a exactly one item: the result
        let res = stack.pop()?;
        if stack.empty() {
            println!("Reply> {:?}", res);
        } else {
            return Err(rpn::Error::Extra);
        }
    }
}

pub fn evaluate_line(stack: &mut rpn::Stack, buf: &str) -> rpn::Result<()> {
    let tokens = buf.trim().split_whitespace();

    for tok in tokens {
        match tok {
            "+" => stack.eval(rpn::Op::Add)?,
            "=" => stack.eval(rpn::Op::Eq)?,
            "!" => stack.eval(rpn::Op::Neg)?,
            "<->" => stack.eval(rpn::Op::Swap)?,
            "#" => stack.eval(rpn::Op::Rand)?,
            "?" => stack.eval(rpn::Op::Cond)?,
            "quit" => return Err(rpn::Error::Quit),
            "true" => stack.push(rpn::Item::Bool(true))?,
            "false" => stack.push(rpn::Item::Bool(false))?,
            _ => match tok.parse::<i32>() {
                Ok(num) => stack.push(rpn::Item::Int(num))?,
                Err(_) => return Err(rpn::Error::Syntax),
            },
        }
    }
    

    Ok(())
}
