use rust_web_app::db::{create_task, establish_connection, query_task, delete_task};
use std::env;

fn help() {
    println!("subcommands:");
    println!("    new<title>: create a new task");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let subcommand = &args[1];
    match subcommand.as_ref() {
        "new" => new_task(&args[2..]),
        "show" => show_tasks(&args[2..]),
        "delete" => delete_tasks(&args[2..]),
        _ => help(),
    }
}

fn new_task(args: &[String]) {
    if args.len() < 1 {
        println!("new: missing <title>");
        help();
        return;
    }

    let conn = establish_connection();
    create_task(&conn, &args[0]);
}


fn show_tasks(args: &[String]) {
    if args.len() > 0 {
        println!("show: unexpected argument");
        help();
        return;
    }

    let conn = establish_connection();
    println!("TASKS\n-----");
    for task in query_task(&conn) {
        println!("{}, {}", task.id, task.title);
    }
}

fn delete_tasks(args: &[String]) {
    if args.len() < 0 {
        println!("delete: unexpected argument");
        help();
        return;
    }

    let conn = establish_connection();
    delete_task(&conn, &args[0].parse::<i32>().unwrap().clone());
    return;
}