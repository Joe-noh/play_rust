use std::io::BufRead;
use anyhow::{bail, Result, Context};

pub fn calc<R: BufRead>(reader: R) {
    let rpn = Rpn::new();

    for line in reader.lines() {
        let formula = line.unwrap();

        match rpn.eval(&formula) {
            Ok(result) => println!("{}", result),
            Err(error) => println!("{}", error),
        }
    }
}

struct Rpn();

impl Rpn {
    fn new() -> Self {
        Self()
    }

    fn eval(&self, formula: &str) -> Result<i32> {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        let mut stack = Vec::new();
        let mut pos = 0;

        while let Some(token) = tokens.pop() {
            pos += 1;

            if let Ok(n) = token.parse::<i32>() {
                stack.push(n)
            } else {
                let y = stack.pop().with_context(|| format!("Syntax error at {}", pos))?;
                let x = stack.pop().with_context(|| format!("Syntax error at {}", pos))?;

                match token {
                    "+" => stack.push(x + y),
                    "-" => stack.push(x - y),
                    "*" => stack.push(x * y),
                    "/" => stack.push(x / y),
                    _ => bail!("Invalid token '{}' at {}", token, pos),
                }
            }
        }

        if stack.len() == 1 {
            Ok(stack[0])
        } else {
            bail!("Syntax error")
        }
    }
}

#[test]
fn test_ok() {
    let rpn = Rpn::new();

    assert_eq!(rpn.eval("1 1 +").unwrap(), 2);
    assert_eq!(rpn.eval("4 1 -").unwrap(), 3);
    assert_eq!(rpn.eval("2 3 *").unwrap(), 6);
    assert_eq!(rpn.eval("4 2 /").unwrap(), 2);
    assert_eq!(rpn.eval("3 5 + 2 / 6 * 5 -").unwrap(), 19);
}

#[test]
fn test_error() {
    let rpn = Rpn::new();

    assert!(rpn.eval("1 1 1").is_err());
    assert!(rpn.eval("1 & 1").is_err());
    assert!(rpn.eval("").is_err());
}
