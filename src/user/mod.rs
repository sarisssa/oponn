use mongod::{Bson, Mongo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
// for find_one() --> impl Into<Option<Document>>
// for update_many / update_one() --> impl Into<UpdateModifications>
pub struct UserOrig {
    pub id: String,
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
// serde_mongo
// for find_one() --> impl Into<Option<Document>>
// for update_many / update_one() --> impl Into<UpdateModifications>
pub struct Data {
    pub name: String,
}

impl UserOrig {
    fn set() {
        unimplemented!()
    }

    fn get() {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use futures::stream::StreamExt;
    use mongod::{AsFilter, AsUpdate, Collection, Comparator, Updates};
    use mongod::{Bson, Mongo};
    use std::env;

    #[derive(Bson, Mongo, Debug)]
    #[mongo(collection = "userss", field, filter, update)]
    pub struct Userss {
        name: String,
        age: Option<u32>,
        email: Option<String>,
    }

    #[derive(Bson, Mongo, Debug)]
    #[mongo(collection = "users", field, filter, update)]
    pub struct User {
        name: String,
    }

    #[tokio::test]
    async fn test_connection() {
        use futures::stream::StreamExt;
        use mongod::bson::Document;
        use mongod::{AsFilter, AsUpdate, Collection, Comparator, Updates};
        dotenv().ok();

        // Create and async client
        let (client_uri, db) = (
            env::var("MONGODB_URI").expect("Please set the MONGODB_URI environment var!"),
            "fnchart",
        );

        let client = mongod::Client::builder()
            .uri(client_uri)
            .database(db)
            .build()
            .unwrap();

        // Insert a user into the users collection
        let user = Userss {
            name: "foo".to_string(),
            age: None,
            email: None,
        };
        let oid = client.insert_one::<Userss>(user).await.unwrap();
        println!("{:?}", oid);

        // Fetch all users in the users collection
        let mut cursor = client.find::<Userss, _>(None).await.unwrap();
        while let Some(res) = cursor.next().await {
            if let Ok((id, user)) = res {
                println!("{} - {:?}", id, user);
            }
        }

        // Update the user
        let mut filter = Userss::filter();
        filter.name = Some(Comparator::Eq("foo".to_owned()));
        let mut update = Userss::update();
        //update.age = Some(Some(0));
        let updates = Updates {
            set: Some(update),
            ..Default::default()
        };
        let updated = client
            .update::<Userss, _, _>(filter, updates)
            .await
            .unwrap();
        println!("updated {} documents", updated);

        // Delete all users
        let deleted = client.delete::<Userss, _>(None).await.unwrap();
        println!("delete {} documents", deleted);

        assert!(true)
    }

    #[tokio::test]
    async fn test_connection_two() {
        use futures::stream::StreamExt;
        use mongod::bson::Document;
        use mongod::{db::Client, AsFilter, AsUpdate, Collection, Comparator, Updates};
        dotenv().ok();

        // Create and async client
        let (client_uri, db) = (
            env::var("MONGODB_URI").expect("Please set the MONGODB_URI environment var!"),
            "fnchart",
        );

        let client = mongod::Client::builder()
            .uri(client_uri)
            .database(db)
            .build()
            .unwrap();

        let user = User {
            name: "foo".to_owned(),
        };

        let user_collection = User::COLLECTION;

        let result = client.insert_one::<User>(user).await.unwrap();

        println!("Result: {:#?}", result);

        assert!(true)
    }

    #[tokio::test]
    async fn test_connection_three() {
        use futures::stream::StreamExt;
        use mongod::bson::Document;
        use mongod::{db::Client, AsFilter, AsUpdate, Collection, Comparator, Updates};
        dotenv().ok();

        // Create and async client
        let (client_uri, db) = (
            env::var("MONGODB_URI").expect("Please set the MONGODB_URI environment var!"),
            "fnchart",
        );

        let client = Client::with_uri_str(client_uri).await.unwrap();

        let user = User {
            name: "foo".to_owned(),
        };

        let user_collection = User::COLLECTION;

        let result = client
            .database(db)
            .collection(user_collection)
            .insert_one(user.into_document().unwrap(), None)
            .await
            .unwrap();

        println!(
            "Result({:#?}): {:#?}",
            result.inserted_id.to_string().len(),
            result.inserted_id.to_string()
        );

        assert!(result.inserted_id.to_string().len() > 0)
    }
}
