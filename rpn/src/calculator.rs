use std::io::BufRead;

pub fn calc<R: BufRead>(reader: R) {
    let rpn = Rpn::new();

    for line in reader.lines() {
        let formula = line.unwrap();

        println!("{}", rpn.eval(&formula));
    }
}

struct Rpn();

impl Rpn {
    fn new() -> Self {
        Self()
    }

    fn eval(&self, formula: &str) -> i32 {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        let mut stack = Vec::new();

        while let Some(token) = tokens.pop() {
            if let Ok(n) = token.parse::<i32>() {
                stack.push(n)
            } else {
                let y = stack.pop().expect("invalid formula");
                let x = stack.pop().expect("invalid formula");

                match token {
                    "+" => stack.push(x + y),
                    "-" => stack.push(x - y),
                    "*" => stack.push(x * y),
                    "/" => stack.push(x / y),
                    _ => panic!()
                }
            }
        }

        if stack.len() == 1 {
            stack[0]
        } else {
            panic!()
        }
    }
}

#[test]
fn test_ok() {
    let rpn = Rpn::new();

    assert_eq!(rpn.eval("1 1 +"), 2);
    assert_eq!(rpn.eval("4 1 -"), 3);
    assert_eq!(rpn.eval("2 3 *"), 6);
    assert_eq!(rpn.eval("4 2 /"), 2);
    assert_eq!(rpn.eval("3 5 + 2 / 6 * 5 -"), 19);
}

#[test]
#[should_panic]
fn test_error() {
    let rpn = Rpn::new();

    rpn.eval("1 1 1");
}
