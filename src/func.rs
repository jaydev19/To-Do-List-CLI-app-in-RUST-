use super::{TodoLists, TodoItem};
use super::*;

pub fn add_todo_item(todo_lists: &mut TodoLists, list_name: &str, title: &str) {
    let items = todo_lists.lists.entry(list_name.to_string()).or_insert_with(Vec::new);
    let new_item = TodoItem {
        item_number: items.len() as u32 + 1,
        title: title.to_string(),
        complete: false,
    };
    items.push(new_item);
    todo_lists.save().expect("Failed to save items");
    println!("Added to-do item: {} to list: {}", title, list_name);
}


pub fn list_all_lists() 
{
    let todo_lists = TodoLists::load().unwrap();
    for list_name in todo_lists.lists.keys() {
        println!("{}", list_name);
    }
}

pub fn list_all_items(status: Status) {
    let todo_lists = TodoLists::load().unwrap();
    for (list_name, items) in &todo_lists.lists {
        println!("List: {}", list_name);
        for item in items {
            match status {
                Status::All => println!("  {}. {} [{}]", item.item_number, item.title, if item.complete { "x" } else { " " }),
                Status::Completed if item.complete => println!("  {}. {} [{}]", item.item_number, item.title, if item.complete { "x" } else { " " }),
                Status::Incomplete if !item.complete => println!("  {}. {} [{}]", item.item_number, item.title, if item.complete { "x" } else { " " }),
                _ => {},
            }
        }
    }
}
    
    pub fn list_items_in_list(list_name: &str, status: Status) {
        let todo_lists = TodoLists::load().unwrap();
        if let Some(items) = todo_lists.lists.get(list_name) {
            for item in items {
                match status {
                    Status::All => println!("  {}. {} [{}]", item.item_number, item.title, if item.complete { "x" } else { " " }),
                    Status::Completed if item.complete => println!("  {}. {} [{}]", item.item_number, item.title, if item.complete { "x" } else { " " }),
                    Status::Incomplete if !item.complete => println!("  {}. {} [{}]", item.item_number, item.title, if item.complete { "x" } else { " " }),
                    _ => {},
                }
            }
        } else {
            println!("List '{}' not found", list_name);
        }
    }
    pub fn update_item_status(list_name: &str, item_number: u32, complete: bool) {
        let mut todo_lists = TodoLists::load().unwrap();
        if let Some(items) = todo_lists.lists.get_mut(list_name) {
            if let Some(item) = items.iter_mut().find(|item| item.item_number == item_number) {
                item.complete = complete;
                todo_lists.save().expect("Failed to save items");
                println!("Marked item {} in list {} as {}", item_number, list_name, if complete { "complete" } else { "incomplete" });
            } else {
                println!("No item found with ID {} in list {}", item_number, list_name);
            }
        } else {
            println!("No list found with name {}", list_name);
        }
    }
    

    pub fn remove_todo_item(list_name: &str, item_number: u32) 
    {
        let mut todo_lists = TodoLists::load().unwrap();
        if let Some(items) = todo_lists.lists.get_mut(list_name) {
            if let Some(index) = items.iter().position(|item| item.item_number == item_number) {
                items.remove(index);
                todo_lists.save().expect("Failed to save items");
                println!("Removed item {} from list {}", item_number, list_name);
            } else {
                println!("No item found with ID {} in list {}", item_number, list_name);
            }
        } else {
            println!("No list found with name {}", list_name);
        }
    }

    pub fn remove_todo_list(list_name: &str) 
    {
        let mut todo_lists = TodoLists::load().unwrap();
        if todo_lists.lists.remove(list_name).is_some() {
            todo_lists.save().expect("Failed to save items");
            println!("Removed list {}", list_name);
        } else {
            println!("No list found with name {}", list_name);
        }
    }

    pub fn remove_all_lists() {
        let mut todo_lists = TodoLists::load().unwrap();
        todo_lists.lists.clear();
        todo_lists.save().expect("Failed to save items");
        println!("Removed all lists");
    }
    