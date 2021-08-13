use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::{App, Arg};
use regex::Regex;

fn main() {
    let args = App::new("grep-lite")
        .version("0.1.0")
        .about("searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("File to search")
                .takes_value(true)
                .required(true)
        )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let input = args.value_of("input").unwrap();

    let re = Regex::new(pattern).unwrap();
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let l = line.unwrap();
        match re.find(&l) {
            Some(_) => println!("{}", l),
            None => {}
        }
    }
}
