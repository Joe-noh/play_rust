use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("todo")
        .subcommand(SubCommand::with_name("list").about("List all todos"))
        .subcommand(
            SubCommand::with_name("add")
                .about("Add a new todo")
                .arg(Arg::with_name("TODO").required(true)),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .about("Remove some todos")
                .arg(
                    Arg::with_name("id")
                        .long("id")
                        .short("i")
                        .takes_value(true)
                        .multiple(true)
                        .conflicts_with("all"),
                )
                .arg(
                    Arg::with_name("all")
                        .long("all")
                        .short("a")
                        .conflicts_with("id"),
                ),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("list") => {
            dbg!("list");
        }
        Some("add") => {
            let matches = matches.subcommand_matches("add").unwrap();

            dbg!(matches.value_of("TODO").unwrap());
        }
        Some("remove") => {
            let matches = matches.subcommand_matches("remove").unwrap();

            if matches.is_present("all") {
                dbg!("all!");
            } else if let Some(ids) = matches.values_of("id") {
                dbg!(ids);
            }
        }
        Some(_) => panic!(),
        None => panic!(),
    }
}
