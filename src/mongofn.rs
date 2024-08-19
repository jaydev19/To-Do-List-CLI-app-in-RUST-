use super::*;
use mongodb::{Client, bson::{doc, Bson, Document}, Collection};
use futures::stream::StreamExt;

pub async fn push_to_mongodb() -> mongodb::error::Result<()>       //Result type with an empty tuple () as the success value
{
    let client = Client::with_uri_str(MONGO_URI).await?;
    let db = client.database("todo_app");
    let collection: Collection<Document> = db.collection("todo_lists");

    let todo_lists = TodoLists::load().expect("Failed to load local items");

    for (list_name, items) in todo_lists.lists {
        let doc = doc! {
            "list_name": list_name.clone(),
            "items": items.iter().map(|item| {                          // this part constructs the inner documents for each item
                doc! {                                                  // this is to iterate over each item in the items collection
                    "item_number": item.item_number as i32,
                    "title": &item.title,
                    "complete": item.complete
                }
            }).collect::<Vec<_>>(),                                     // this is used to collect the iterated values into a vector 
        };
        collection.insert_one(doc).await?;                              //line inserts the constructed document into the MongoDB collection
    }

    println!("Pushed local items to MongoDB");
    Ok(())                                                              //Ok(()) to indicate successful execution.
}

pub async fn pull_from_mongodb() -> mongodb::error::Result<()> 
{
    let client = Client::with_uri_str(MONGO_URI).await?;
    let db = client.database("todo_app");
    let collection: Collection<Document> = db.collection("todo_lists");

    let mut todo_lists = TodoLists::new();
    let mut cursor = collection.find(doc! {}).await?;

    while let Some(result) = cursor.next().await { // iterates over lists
        if let Ok(document) = result {           //result (which represents a document) is successfully deserialized into a Rust Document type
            let list_name = document.get_str("list_name").unwrap().to_string();
            let items = document.get_array("items").unwrap();       //this line retrieves the value of the “items” field as an array
            let mut todo_items = Vec::new();

            for item in items {                         //iterates over the items 
                if let Bson::Document(doc) = item {
                    let item_number = doc.get_i32("item_number").unwrap() as u32;
                    let title = doc.get_str("title").unwrap().to_string();
                    let complete = doc.get_bool("complete").unwrap();

                    let todo_item = TodoItem {
                        item_number,
                        title,
                        complete,
                    };
                    todo_items.push(todo_item);
                }
            }

            todo_lists.lists.insert(list_name, todo_items);   
        }
    }

    todo_lists.save().expect("Failed to save items to local file");

    println!("Pulled items from MongoDB to local file");
    Ok(())
}
