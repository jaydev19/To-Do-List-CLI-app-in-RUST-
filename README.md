# To-Do-List-CLI-app-in-RUST

# minor_project
a rust to-do app , using clap and mongodb

## Introduction

Todo List Management System
This Rust application manages todo lists, allowing users to add, view, update, and remove tasks locally or sync with MongoDB. Below is a detailed README to guide users on how to use and understand the code.

## Features
Add: Add a new todo item to a specific list.
Show: Display todo lists and items based on various filters.
Complete/Incomplete: Mark todo items as complete or incomplete.3
Remove: Remove tasks or entire lists.
Push/Pull: Sync todo lists between local storage and MongoDB.

## Reqirements
  ### crates used 

    clap = { version = "4.0", features = ["derive"] }S
    mongodb = {version = "3.0.1",features = ["sync"]}
    tokio = { version = "1.38.0", features = ["full"] }
   

## Usage
- **View Tasks**: List tasks with various filters.
  - `todo st_name> <item>`: Adds the item to that list.
show`: Shows all the list names.
  - `todo show -a`: Shows all the list names along with the items.
  - `todo show -c`: Shows all the completed items of all lists.
  - `todo show -i`: Shows all the incomplete items of all lists.
  - `todo show <list_name>`: Shows all the items of that list.
  - `todo show <list_name> -c`: Shows all the completed items of that list.
  - `todo show <list_name> -i`: Shows all the incomplete items of that list.

- **Add Tasks**: Add items to your lists.
  - `todo add <li
- **Complete/Incomplete Tasks**: Mark tasks as completed or incomplete.
  - `todo complete <list_name> <item_number>`: Marks an item as completed.
  - `todo incomplete <list_name> <item_number>`: Marks an item as incomplete.

- **Remove Tasks**: Remove tasks or lists.
  - `todo remove`: Removes all lists.
  - `todo remove <list_name>`: Removes that particular list.
  - `todo remove <list_name> <item_number>`: Removes that item from the list.

### Additional Features

- **Cloud Synchronization**: Keep your tasks synced across devices.
  - `todo push`: Syncs local changes with the cloud.
  - `todo pull`: Fetches updates from the cloud.


## main.rs
  1. **`use` Statements**:
    - `use clap::{Arg, Command};`: Imports the `Arg` and `Command` modules from the `clap` crate, which is used for command-line argument parsing.
    - `use std::collections::HashMap;`: Imports the `HashMap` collection from the standard library.
    - `use std::fs::{File, OpenOptions};`: Imports `File` and `OpenOptions` for file handling.
    - `use std::io::{self, BufReader, BufWriter, Write, BufRead};`: Imports various I/O utilities for reading and writing files.
    - `use tokio;`: Imports the `tokio` crate for asynchronous programming.
2. **Modules**:
    - `mod test;`: Declares a module named `test`. The actual content of this module is not provided in the snippet.
    - `mod func;`: Declares a module named `func`. The actual content of this module is not provided in the snippet.
    - `mod mongofn;`: Declares a module named `mongofn`. The actual content of this module is not provided in the snippet.
3. **Constants**:
    - `const FILE_PATH: &str = "todo_list.txt";`: Defines a constant `FILE_PATH` that holds the path to the file where the to-do list is stored.
    - `const MONGO_URI: &str = "mongodb+srv://cse23bcsd60:gOW5NTFJRJofz5jt@cluster0.fxwp8dj.mongodb.net";`: Defines a constant `MONGO_URI` that holds the MongoDB connection URI.
4. **Structs**:
    - `#[derive(Debug)] struct TodoItem`: Defines a struct `TodoItem` with fields `item_number`, `title`, and `complete`.
    - `#[derive(Debug, Default)] struct TodoLists`: Defines a struct `TodoLists` with a field `lists` which is a `HashMap` of to-do lists.
5. **Main Function**:
    - `#[tokio::main] async fn main()`: The main function of the program, marked as asynchronous using the `tokio::main` macro. It handles command-line arguments and executes corresponding actions.
6. **Command Handling**:
    - The `main` function uses the `clap` crate to define and parse various subcommands (`add`, `show`, `complete`, `incomplete`, `remove`, `push`, `pull`). Each subcommand has its own arguments and associated actions.
7. **`TodoLists` Implementation**:
    - `impl TodoLists`: Implements methods for the `TodoLists` struct.
        - `fn new() -> Self`: Creates a new `TodoLists` instance.
        - `fn load() -> io::Result<Self>`: Loads the to-do lists from a file.
        - `fn save(&self) -> io::Result<()>`: Saves the to-do lists to a file.


 **Functions in External Modules**:
    - `func::add_todo_item`: Adds a to-do item to a list.
    - `func::list_items_in_list`: Lists items in a specific to-do list.
    - `func::list_all_items`: Lists all items across all to-do lists.
    - `func::list_all_lists`: Lists all to-do lists.
    - `func::update_item_status`: Updates the status (complete/incomplete) of a to-do item.
    - `func::remove_todo_item`: Removes a specific to-do item.
    - `func::remove_todo_list`: Removes a specific to-do list.
    - `func::remove_all_lists`: Removes all to-do lists.
    - `mongofn::push_to_mongodb`: Pushes the local to-do list to MongoDB.
    - `mongofn::pull_from_mongodb`: Pulls the to-do list from MongoDB to local storage.

## func.rs 
  1. **`add_todo_item`**:
    - **Purpose**: Adds a new to-do item to a specified list.
    - **Parameters**:
        - `todo_lists`: A mutable reference to `TodoLists`.
        - `list_name`: The name of the list to which the item will be added.
        - `title`: The title of the new to-do item.
    - **Behavior**:
        - Retrieves or creates a list with the given name.
        - Creates a new `TodoItem` with the next item number, the given title, and marks it as incomplete.
        - Adds the new item to the list.
        - Saves the updated `TodoLists`.
        - Prints a confirmation message.
2. **`list_all_lists`**:
    - **Purpose**: Lists all the to-do lists.
    - **Parameters**: None.
    - **Behavior**:
        - Loads the `TodoLists`.
        - Iterates over the list names and prints each one.
3. **`list_all_items`**:
    - **Purpose**: Lists all items in all lists, optionally filtered by completion status.
    - **Parameters**:
        - `filter`: An optional boolean to filter items by their completion status.
    - **Behavior**:
        - Loads the `TodoLists`.
        - Iterates over each list and its items.
        - Prints each item, optionally filtered by the completion status.
4. **`list_items_in_list`**:
    - **Purpose**: Lists all items in a specified list, optionally filtered by completion status.
    - **Parameters**:
        - `list_name`: The name of the list to display items from.
        - `filter`: An optional boolean to filter items by their completion status.
    - **Behavior**:
        - Loads the `TodoLists`.
        - Retrieves the specified list.
        - Prints each item in the list, optionally filtered by the completion status.
        - Prints a message if the list does not exist.
5. **`update_item_status`**:
    - **Purpose**: Updates the completion status of a specified item in a specified list.
    - **Parameters**:
        - `list_name`: The name of the list containing the item.
        - `item_number`: The number of the item to update.
        - `complete`: The new completion status.
    - **Behavior**:
        - Loads the `TodoLists`.
        - Retrieves the specified list and item.
        - Updates the item's completion status.
        - Saves the updated `TodoLists`.
        - Prints a confirmation message.
        - Prints a message if the list or item does not exist.
6. **`remove_todo_item`**:
    - **Purpose**: Removes a specified item from a specified list.
    - **Parameters**:
        - `list_name`: The name of the list containing the item.
        - `item_number`: The number of the item to remove.
    - **Behavior**:
        - Loads the `TodoLists`.
        - Retrieves the specified list and item.
        - Removes the item from the list.
        - Saves the updated `TodoLists`.
        - Prints a confirmation message.
        - Prints a message if the list or item does not exist.
7. **`remove_todo_list`**:
    - **Purpose**: Removes a specified list.
    - **Parameters**:
        - `list_name`: The name of the list to remove.
    - **Behavior**:
        - Loads the `TodoLists`.
        - Removes the specified list.
        - Saves the updated `TodoLists`.
        - Prints a confirmation message.
        - Prints a message if the list does not exist.
8. **`remove_all_lists`**:
    - **Purpose**: Removes all to-do lists.
    - **Parameters**: None.
    - **Behavior**:
        - Creates a new, empty `TodoLists`.
        - Saves the empty `TodoLists`.
        - Prints a confirmation message.



## mongodb.rs

  1. **`push_to_mongodb`**:
    - **Purpose**: This function pushes local todo list items to a MongoDB collection.
    - **Steps**:
        1. Connects to the MongoDB client using the URI specified by `MONGO_URI`.
        2. Accesses the `todo_app` database and the `todo_lists` collection.
        3. Loads local todo lists using the `TodoLists::load` method.
        4. Iterates over each todo list and its items, creating a BSON document for each list.
        5. Inserts each BSON document into the MongoDB collection.
        6. Prints a confirmation message and returns `Ok(())` if successful.
2. **`pull_from_mongodb`**:
    - **Purpose**: This function pulls todo list items from a MongoDB collection and saves them locally.
    - **Steps**:
        1. Connects to the MongoDB client using the URI specified by `MONGO_URI`.
        2. Accesses the `todo_app` database and the `todo_lists` collection.
        3. Initializes an empty `TodoLists` object.
        4. Queries the collection for all documents.
        5. Iterates over the query results, extracting and converting BSON documents into local `TodoItem` objects.
        6. Inserts the extracted items into the `TodoLists` object.
        7. Saves the `TodoLists` object locally using the `save` method.
        8. Prints a confirmation message and returns `Ok(())` if successful.

### Macros

1. **`doc!`**:
    - **Purpose**: This macro is used to create BSON documents in a concise and readable manner.
    - **Usage in `push_to_mongodb`**:This creates a BSON document representing a todo list with its items.
        
        ```rust
        let doc = doc! {
            "list_name": list_name.clone(),
            "items": items.iter().map(|item| {
                doc! {
                    "item_number": item.item_number as i32,
                    "title": &item.title,
                    "complete": item.complete
                }
            }).collect::<Vec<_>>(),
        };
        
        ```
        
    - **Usage in `pull_from_mongodb`**:This creates an empty BSON document used to query all documents in the collection.
        
        ```rust
        let mut cursor = collection.find(doc! {}).await?;
        
        ```
        

### Additional Notes

- **`MONGO_URI`**: This is a constant that should be defined elsewhere in the code, containing the MongoDB connection string.
- **`TodoLists` and `TodoItem`**: These are custom types that should be defined elsewhere in the code. `TodoLists` likely contains a method `load` to load local items and a method `save` to save items locally. `TodoItem` represents individual todo items.