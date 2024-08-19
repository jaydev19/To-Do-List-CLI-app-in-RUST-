use clap::{Arg, Command};

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Write, BufRead};

use tokio;

mod test;
mod func;
mod mongofn;

const FILE_PATH: &str = "todo_list.txt";
const MONGO_URI: &str = "mongodb+srv://cse23bcsd60:gOW5NTFJRJofz5jt@cluster0.fxwp8dj.mongodb.net";

#[derive(Debug)]

pub struct TodoItem 
{
    item_number: u32,
    title: String,
    complete: bool,
}

pub enum Status {
    All,
    Completed,
    Incomplete,
}

#[derive(Debug, Default)]
struct TodoLists 
{
    lists: HashMap<String, Vec<TodoItem>>,
}

#[tokio::main]
async fn main() 
{
    let matches = Command::new("todo")
        .subcommand(Command::new("add")                        // add commands 
            .arg(Arg::new("list_name").index(1))
            .arg(Arg::new("title").index(2)))

        .subcommand(Command::new("show")                        // show commands
            .arg(Arg::new("list_name").index(1))
            .arg(Arg::new("all").short('a'))
            .arg(Arg::new("completed").short('c'))
            .arg(Arg::new("incomplete").short('i')))

        .subcommand(Command::new("complete")                     // complete/incomplete commands
            .arg(Arg::new("list_name").index(1))
            .arg(Arg::new("list_name").index(1))
            .arg(Arg::new("item_number").index(2)))

        .subcommand(Command::new("incomplete")
            .arg(Arg::new("list_name").index(1))
            .arg(Arg::new("item_number").index(2)))

        .subcommand(Command::new("remove")                       //remove commands
            .arg(Arg::new("list_name").index(1))
            .arg(Arg::new("item_number").index(2)))

        .subcommand(Command::new("push"))                      // push to database

        .subcommand(Command::new("pull"))                     // pull from database

        .get_matches();

    match matches.subcommand() 
    {
        Some(("add", arg)) => {                     // checks if both are Some and not None 
            if let (Some(list_name), Some(title)) = (arg.get_one::<String>("list_name"), arg.get_one::<String>("title")) {
                let mut todo_lists = TodoLists::load().unwrap();       // here retreving from cli
                func::add_todo_item(&mut todo_lists, list_name, title);
            } else {
                println!("invalid input");
            }
        }
        Some(("show", arg)) => {                    
            let list_name = arg.get_one::<String>("list_name");
            let show_all = arg.contains_id("all");
            let show_completed = arg.contains_id("completed");
            let show_incomplete = arg.contains_id("incomplete");
    
            let status = if show_all {
                Status::All
            } else if show_completed {
                Status::Completed
            } else if show_incomplete {
                Status::Incomplete
            } else {
                Status::All
            };
    
            match list_name {
                Some(list_name) => func::list_items_in_list(list_name, status),
                None => func::list_all_items(status),
            }
        }
        Some(("complete", arg)) => {
            if let (Some(list_name), Some(item_number)) = (arg.get_one::<String>("list_name"), arg.get_one::<String>("item_number")) {
                let item_number = item_number.parse::<u32>().unwrap();
                func::update_item_status(list_name, item_number, true);
            } else {
                println!("invalid input");
            }
        }
        Some(("incomplete", arg)) => {
            if let (Some(list_name), Some(item_number)) = (arg.get_one::<String>("list_name"), arg.get_one::<String>("item_number")) {
                let item_number = item_number.parse::<u32>().unwrap();
                func::update_item_status(list_name, item_number, false);
            } else {
                println!("invalid input");
            }
        }
        Some(("remove", arg)) => {                                         // parse extract specific information from a formatted text.
            let list_name = arg.get_one::<String>("list_name");   //This attempts to get a value associated with the key "list_name" from arg
            let item_number = arg.get_one::<String>("item_number").map(|s| s.parse::<u32>().unwrap());

            match (list_name, item_number) {
                (Some(list_name), Some(item_number)) => func::remove_todo_item(list_name, item_number),
                (Some(list_name), None) => func::remove_todo_list(list_name),
                (None, _) => func::remove_all_lists(),
            }
        }
        Some(("push", _)) => {
            mongofn::push_to_mongodb().await.unwrap();
        }
        Some(("pull", _)) => {
            mongofn::pull_from_mongodb().await.unwrap();
        }
        _ => {
            println!("invalid command ");
        }
    }
}

impl TodoLists 
{
    fn new() -> Self 
    {
        Self {
            lists: HashMap::new(),
        }
    }

    fn load() -> io::Result<Self> 
    {
        let file = File::open(FILE_PATH)?;
        let reader = BufReader::new(file);
        let mut lists = TodoLists::new();

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect();
            if let [list_name, item_number, title, complete] = &parts[..] {
                let list_name = list_name.to_string();
                let item_number = item_number.parse().unwrap();
                let title = title.to_string();
                let complete = complete.trim() == "true"; // Assuming "true" or "false"

                let item = TodoItem {
                    item_number,
                    title,
                    complete,
                };

                lists.lists.entry(list_name).or_insert_with(Vec::new).push(item);
            }
        }

        Ok(lists)
    }

    fn save(&self) -> io::Result<()> 
    {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(FILE_PATH)?;

        let mut writer = BufWriter::new(file);

        for (list_name, items) in &self.lists {
            for item in items {
                writeln!(
                    &mut writer,
                    "{},{},{},{}",
                    list_name,
                    item.item_number,
                    item.title,
                    item.complete
                )?;
            }
        }

        writer.flush()?; // Ensure all buffered data is written to file

        Ok(())
    }
}
