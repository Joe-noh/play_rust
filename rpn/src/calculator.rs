use std::io::BufRead;

pub fn calc<R: BufRead>(reader: R) {
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}
