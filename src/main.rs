use anyhow::{Error, Result};
use bson::doc;
use dotenv::dotenv;
use mongodb::{bson::Document, Client, Collection, Database};
use std::env;

async fn setup() -> Result<Collection<Document>> {
    dotenv().ok();
    let client_uri = env::var("MONGODB_URI").expect("Please set the MONGODB_URI environment var!");

    let client = Client::with_uri_str(client_uri).await?;

    let users_collection = client.database("fnchart").collection::<Document>("users");
    Ok(users_collection);
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
    use dotenv::dotenv;
    use mongodb::{bson::Document, Client, Database};
    use std::env;
    use tokio::test;

    #[tokio::test]
    async fn insert_doc() {
        dotenv().ok();
        let client_uri =
            env::var("MONGODB_URI").expect("Please set the MONGODB_URI environment var!");

        let client = Client::with_uri_str(client_uri).await.unwrap();

        let users_collection = client.database("fnchart").collection::<Document>("users");

        println!("TRUE")
    }
}
