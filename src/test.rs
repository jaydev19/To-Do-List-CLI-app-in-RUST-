#[cfg(test)]
use super::*;
use super::func::*;
use super::mongofn::*;
use std::fs;
use crate::FILE_PATH;

fn setup_test_file() {
    let _ = fs::remove_file(FILE_PATH);
}

fn read_file_content() -> String {
    fs::read_to_string(FILE_PATH).unwrap_or_default()
}

#[test]
fn test_add_todo_item() {
    setup_test_file();
    
    let mut todo_lists = TodoLists::new();

    let list_name = "Work";
    let title = "Finish project";

    add_todo_item(&mut todo_lists, list_name, title);

    // Verify the item was added
    assert!(todo_lists.lists.contains_key(list_name));
    let items = todo_lists.lists.get(list_name).unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].title, title);
    assert_eq!(items[0].item_number, 1);
    assert_eq!(items[0].complete, false);

    // Add another item to the same list
    let title2 = "Start new project";
    add_todo_item(&mut todo_lists, list_name, title2);

    // Verify the second item was added
    let items = todo_lists.lists.get(list_name).unwrap();
    assert_eq!(items.len(), 2);
    assert_eq!(items[1].title, title2);
    assert_eq!(items[1].item_number, 2);
    assert_eq!(items[1].complete, false);
}

// Lists all todo lists when there are multiple lists
#[test]
fn lists_all_todo_lists_with_multiple_lists() {
    setup_test_file();

    use std::collections::HashMap;
    
    let mut lists = HashMap::new();
    lists.insert("Work".to_string(), vec![]);
    lists.insert("Personal".to_string(), vec![]);
    
    let todo_lists = TodoLists { lists };
    TodoLists::save(&todo_lists).unwrap();
    
    list_all_lists();
}



#[test]
fn test_remove_todo_item() {
    setup_test_file();
    let mut todo_lists = TodoLists::new();
    add_todo_item(&mut todo_lists, "test_list", "test_item");
    remove_todo_item("test_list", 1);
    let content = read_file_content();
    assert!(!content.contains("test_list,1,test_item,false"));
}

#[tokio::test]
async fn test_push_to_mongodb() {
    setup_test_file();
    let mut todo_lists = TodoLists::new();
    add_todo_item(&mut todo_lists, "test_list", "test_item");
    let result = push_to_mongodb().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_pull_from_mongodb() {
    setup_test_file();
    let mut todo_lists = TodoLists::new();
    add_todo_item(&mut todo_lists, "test_list", "test_item");
    push_to_mongodb().await.unwrap();
    setup_test_file(); // Clear local file to test pull
    let result = pull_from_mongodb().await;
    assert!(result.is_ok());
    let content = read_file_content();
    assert!(content.contains("test_list,1,test_item,false"));
}

#[test]
fn test_remove_all_lists() {
    setup_test_file();
    let mut todo_lists = TodoLists::new();
    add_todo_item(&mut todo_lists, "test_list1", "test_item1");
    add_todo_item(&mut todo_lists, "test_list2", "test_item2");
    remove_all_lists();
    let content = read_file_content();
    assert!(content.is_empty());
}

#[test]
fn test_remove_todo_list() {
    setup_test_file();
    let mut todo_lists = TodoLists::new();
    add_todo_item(&mut todo_lists, "test_list", "test_item");
    remove_todo_list("test_list");
    let content = read_file_content();
    assert!(!content.contains("test_list"));
}
