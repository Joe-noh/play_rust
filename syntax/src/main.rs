fn main() {
    let x = 5;
    let (x, y) = (1, 2);
    let x: i32 = 5;

    let mut x = 5;
    x = 10;

    let x: i32 = 17;
    {
        let y: i32 = 3;
        println!("x = {}, y = {}", x, y);
    }
    println!("x = {}, y = {}", x, y);  // y = 2, not 3

    print_number(x);

    print_sum(x, add_one(y));

    let f: fn(i32) -> i32 = add_one;
    print_number(f(3));
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn print_sum(x: i32, y: i32) {
    print_number(x + y);
}

fn add_one(x: i32) -> i32 {
    x + 1 // == return x+1;
}

fn diverges() -> ! {
    panic!("Never returns!");
}
