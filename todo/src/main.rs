mod todo_list;

use clap::{App, Arg, SubCommand};
use todo_list::TodoList;

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
        .subcommand(
            SubCommand::with_name("toggle")
                .about("Toggle todo status")
                .arg(
                    Arg::with_name("id")
                        .long("id")
                        .short("i")
                        .takes_value(true)
                        .multiple(true),
                ),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("list") => {
            let list = TodoList::load().unwrap();

            println!("{}", list);
        }
        Some("add") => {
            let matches = matches.subcommand_matches("add").unwrap();
            let body = matches.value_of("TODO").unwrap();
            let mut list = TodoList::load().unwrap();

            list.add(body);
            list.save().unwrap();
        }
        Some("remove") => {
            let matches = matches.subcommand_matches("remove").unwrap();
            let mut list = TodoList::load().unwrap();

            if matches.is_present("all") {
                list.remove_all();
                list.save().unwrap();
            } else if let Some(ids) = matches.values_of("id") {
                let ids = ids.map(|id| id.parse().unwrap()).collect();

                list.remove(ids);
                list.save().unwrap();
            }
        }
        Some("toggle") => {
            let matches = matches.subcommand_matches("toggle").unwrap();
            let mut list = TodoList::load().unwrap();

            if let Some(ids) = matches.values_of("id") {
                let ids = ids.map(|id| id.parse().unwrap()).collect();

                list.toggle(ids);
                list.save().unwrap();
            }
        }
        Some(_) => panic!(),
        None => panic!(),
    }
}
