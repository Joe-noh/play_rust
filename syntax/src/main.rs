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
}
