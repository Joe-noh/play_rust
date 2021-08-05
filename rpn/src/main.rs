mod calculator;

use std::io::stdin;

fn main() {
    let stdin = stdin();
    let reader = stdin.lock();

    calculator::calc(reader)
}
