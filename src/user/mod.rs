use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
// for find_one() --> impl Into<Option<Document>>
// for update_many / update_one() --> impl Into<UpdateModifications>
pub struct User {
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

impl User {
    fn set() {
        unimplemented!()
    }

    fn get() {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use bson::doc;
    use dotenv::dotenv;
    use mongodb::{bson::Document, Client, Database};
    use std::env;
}
