use anyhow::{Error, Result};
use bson::doc;
use dotenv::dotenv;
use mongodb::{bson::Document, Client, Collection, Database};
use std::env;

mod users;

async fn setup() -> Result<Database> {
    dotenv().ok();
    let client_uri = env::var("MONGODB_URI").expect("Please set the MONGODB_URI environment var!");

    let client = Client::with_uri_str(client_uri).await?;

    let db = client.database("fnchart");

    Ok(db)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let client_uri = env::var("MONGODB_URI").expect("Please set the MONGODB_URI environment var!");

    let client = Client::with_uri_str(client_uri).await?;

    let users_collection = client.database("fnchart").collection::<Document>("users");

    let new_user = doc! {
        "name": "Peter Parker"
    };

    let insert_result = users_collection.insert_one(new_user.clone(), None).await?;

    println!("New document ID: {}", insert_result.inserted_id);

    let user = users_collection
        .find_one(
            doc! {
                  "name": "Peter Parker"
            },
            None,
        )
        .await?
        .expect("Error locating user");
    println!("user: {}", user);

    Ok(())
}

#[cfg(test)]

mod tests {
    use bson::doc;
    use dotenv::dotenv;
    use mongodb::{bson::Document, Client, Database};
    use std::env;

    use crate::users;

    #[tokio::test]
    async fn insert_doc() {
        dotenv().ok();
        let client_uri =
            env::var("MONGODB_URI").expect("Please set the MONGODB_URI environment var!");

        let client = Client::with_uri_str(client_uri).await.unwrap();

        let users_collection = client.database("fnchart").collection::<Document>("users");

        println!("TRUE")
    }

    #[tokio::test]
    #[ignore]
    async fn test_users_get() {
        let db = super::setup().await.unwrap();

        let result = db.collection::<Document>("users");

        let user = match result
            .find_one(
                doc! {
                      "name": "Peter Parker"
                },
                None,
            )
            .await
        {
            Ok(res) => res.unwrap(),
            Err(err) => panic!(),
        };

        println!("Result: {:#?}", user); // .Debug() -- not .Display()

        assert!(true)
    }

    #[tokio::test]
    #[ignore]
    async fn test_struct_doc() {
        let db = super::setup().await.unwrap();

        // Get a handle to a collection of `Book`.
        let typed_collection = db.collection::<users::User>("users");

        let users = vec![
            users::User {
                name: "Jim Snow".to_string(),
            },
            users::User {
                name: "Mary Jane".to_string(),
            },
        ];

        typed_collection.insert_many(users, None).await.unwrap();

        let user = match db
            .collection::<users::User>("users")
            .find_one(
                doc! {
                      "name": "Jim Snow"
                },
                None,
            )
            .await
        {
            Ok(res) => res.unwrap(),
            Err(err) => panic!(),
        };

        println!("Result: {:#?}", user); // .Debug() -- not .Display()

        assert!(true)
    }

    #[tokio::test]
    #[ignore]
    async fn update_doc() {
        let db = super::setup().await.unwrap();

        let typed_collection = db.collection::<users::User>("users");

        let user = users::User {
            name: "Bruce Wayne".to_string(),
        };

        typed_collection.insert_one(user, None).await.unwrap();

        let update_result = db
            .collection::<users::User>("users")
            .update_one(
                doc! {
                      "name": "Bruce Wayne"
                },
                doc! {"$set": { "name": "Batman" }},
                None,
            )
            .await;

        println!("Result: {:#?}", update_result); // .Debug() -- not .Display()

        assert!(true)
    }
    #[tokio::test]
    #[ignore]
    async fn delete_doc() {
        let db = super::setup().await.unwrap();

        let typed_collection = db.collection::<users::User>("users");

        let user = users::User {
            name: "To Be Removed".to_string(),
        };

        let temp_doc = typed_collection.insert_one(user, None).await.unwrap();

        println!("New document ID: {}", temp_doc.inserted_id);

        // Delete all documents for movies called "Parasite":
        let delete_result = typed_collection
            .delete_many(
                doc! {
                   "name": "To Be Removed"
                },
                None,
            )
            .await
            .unwrap();
        println!("Deleted {} documents", delete_result.deleted_count);

        // println!("Result: {:#?}", update_result); // .Debug() -- not .Display()

        assert!(true)
    }
}
