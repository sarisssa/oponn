use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    //address: Address,
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
