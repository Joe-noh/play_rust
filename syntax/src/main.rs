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

    let v1 = vec![1, 2, 3];
    let v2 = v1;
    // println!("v1[0] is: {}", v1[0]);

    let v = first(&v2);
    println!("v2[0] is: {}", v);

    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }
    println!("{}", x);  //=> 6

    let mut a = 1;
    let b = &mut a;
    // let c = &mut a;

    let q = 3;
    let p: &i32;  // 逆だとエラー
    p = &q;
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

fn first(v: &Vec<i32>) -> i32 {
    v[0]
}
