use std::env;
use std::fs::{self, File};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    task: String,
    status: String,
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let mut todo: Vec<Task> = File::open("todo.json")
        .map(|file| serde_json::from_reader(file).unwrap())
        .unwrap_or(Vec::new());

    if args.len() > 1 {
        match args[1].as_str() {
            "add" => add_task(&mut todo, args[2].clone()),
            "list" => list_task(&todo),
            "delete" => del_task(&mut todo, args[2].trim().parse::<u8>().unwrap_or(0)),
            "help" => help(),
            _ => println!("todo: unknown option '{}' \nTry 'todo help' for more information", args[1]),
        }
    } else {
        println!("No argument provided!\nTry 'todo help' for more information");
    }

    let file = File::create("todo.json").unwrap();
    serde_json::to_writer(file, &todo).unwrap();

    //println!("\n contents:");
    //list_task(&todo);

}

fn list_task(list: &Vec<Task>) {
    println!("\nContents:");
    for i in 1..=list.len() {
        println!("{i}. {}", list[i-1].task);
    }
}

fn del_task(list: &mut Vec<Task>, index: u8) {
    if index != 0 && usize::from(index) < list.len() {
        list.remove(index.into());
    } else {
        println!("ummm...are u dumbhead or smthing?");
    }
}

fn add_task(list: &mut Vec<Task>, task: String) {

    let the_task = Task {
        task: task,
        status: "ongoing".to_string(),
    };

    list.push(the_task);
}

fn help() {}
