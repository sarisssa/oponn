use anyhow::Result;
use dotenv::dotenv;
use mongodb::{Client, Database};
use std::env;

mod api;
mod datastore;

async fn setup() -> Result<Database> {
    dotenv().ok();
    let client_uri = env::var("MONGODB_URI").expect("Please set the MONGODB_URI environment var!");

    let client = Client::with_uri_str(client_uri).await?;

    let db = client.database("fnchart");

    Ok(db)
}

#[tokio::main]
async fn main() -> Result<()> {
    Ok(())
}

#[cfg(test)]

mod tests {
    use crate::api;
    use anyhow::Result;
    use bson::to_document;
    use dotenv::dotenv;
    use mongodb::bson::Document;
    use std::{env, fs::File, io::Read};

    async fn test_02_json_to_string() {
        let response = r##"
        {
    "id": "abe2c187d35b88402a28c99a113601e9",
    "sport_key": "basketball_nba",
    "sport_title": "NBA",
    "commence_time": "2023-03-04T00:10:00Z",
    "home_team": "Charlotte Hornets",
    "away_team": "Orlando Magic",
    "bookmakers": [
        {
            "key": "draftkings",
            "title": "DraftKings",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:54:02Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 135
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -155
                        }
                    ]
                }
            ]
        },
        {
            "key": "mybookieag",
            "title": "MyBookie.ag",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:55:05Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 139
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -172
                        }
                    ]
                }
            ]
        },
        {
            "key": "fanduel",
            "title": "FanDuel",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:54:02Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 142
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -168
                        }
                    ]
                }
            ]
        },
        {
            "key": "circasports",
            "title": "Circa Sports",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:53:39Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 142
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -173
                        }
                    ]
                }
            ]
        },
        {
            "key": "betrivers",
            "title": "BetRivers",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:54:03Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 133
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -159
                        }
                    ]
                }
            ]
        },
        {
            "key": "sugarhouse",
            "title": "SugarHouse",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:54:03Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 133
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -159
                        }
                    ]
                }
            ]
        },
        {
            "key": "barstool",
            "title": "Barstool Sportsbook",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:55:02Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 133
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -159
                        }
                    ]
                }
            ]
        },
        {
            "key": "twinspires",
            "title": "TwinSpires",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:50:52Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 133
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -159
                        }
                    ]
                }
            ]
        },
        {
            "key": "unibet_us",
            "title": "Unibet",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:53:40Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 133
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -159
                        }
                    ]
                }
            ]
        },
        {
            "key": "foxbet",
            "title": "FOX Bet",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:54:02Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 138
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -182
                        }
                    ]
                }
            ]
        },
        {
            "key": "pointsbetus",
            "title": "PointsBet (US)",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:55:02Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 135
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -170
                        }
                    ]
                }
            ]
        },
        {
            "key": "betus",
            "title": "BetUS",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:55:03Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 140
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -160
                        }
                    ]
                }
            ]
        },
        {
            "key": "williamhill_us",
            "title": "William Hill (US)",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:54:03Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 135
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -160
                        }
                    ]
                }
            ]
        },
        {
            "key": "bovada",
            "title": "Bovada",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:53:39Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 145
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -170
                        }
                    ]
                }
            ]
        },
        {
            "key": "lowvig",
            "title": "LowVig.ag",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:54:26Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 144
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -164
                        }
                    ]
                }
            ]
        },
        {
            "key": "betonlineag",
            "title": "BetOnline.ag",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:55:04Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 144
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -164
                        }
                    ]
                }
            ]
        },
        {
            "key": "wynnbet",
            "title": "WynnBET",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:54:03Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 145
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -175
                        }
                    ]
                }
            ]
        },
        {
            "key": "betmgm",
            "title": "BetMGM",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:55:03Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 145
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -175
                        }
                    ]
                }
            ]
        },
        {
            "key": "superbook",
            "title": "SuperBook",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:53:39Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 150
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -170
                        }
                    ]
                }
            ]
        }
    ]
}
        "##;

        let outcome = serde_json::from_str::<serde_json::Value>(response).unwrap();
        let bookmaker_arr = outcome.get("bookmakers").unwrap();
        let first_bookie_odds = &bookmaker_arr[0];
        println!("{:#?}", first_bookie_odds);
    }

    #[tokio::test]

    async fn test_04_get_game_odds() -> Result<()> {
        dotenv().ok();
        let odds_base_url = "https://api.the-odds-api.com";
        let test_event_id = "abe2c187d35b88402a28c99a113601e9";
        let api_key =
            env::var("ODDS_API_KEY").expect("Please set the ODDS API Key environment var!");
        let date = "2023-03-03T12:00:00Z"; //Chrono crate without offset(UTC)
        let odds_format = "american";
        let odds_url = format!(
            "{}/v4/sports/basketball_nba/events/{}/odds?apiKey={}&regions=us&date={}&oddsFormat={}",
            odds_base_url, test_event_id, api_key, date, odds_format
        );
        println!("{}", odds_url);

        let response = reqwest::get(odds_url).await?.json::<api::Game>().await?;

        println!("{:#?}", response);

        Ok(())
    }

    #[tokio::test]

    async fn test_05_write_game_odds_to_atlas() {
        let db = super::setup().await.unwrap();
        let users_collection = db.collection::<Document>("users");
        let mut json_file = File::open("upcoming.json").expect("Unable to open file");
        let mut contents = String::new();
        json_file
            .read_to_string(&mut contents)
            .expect("Unable to read file");

        let upcoming_games: Vec<api::Game> =
            serde_json::from_str(&contents).expect("Unable to parse JSON");

        for game in &upcoming_games {
            let bson_game = to_document(&game).unwrap();
            println!("{:#?}", bson_game);
            let insert_result = users_collection.insert_one(bson_game, None).await.unwrap();
            println!("New document ID: {}", insert_result.inserted_id);
        }
    }

    #[tokio::test]
    async fn test_06_construct_struct() {
        let db = super::setup().await.unwrap();

        let users_collection = db.collection::<Document>("users");
        let response = r##"
        {
    "id": "abe2c187d35b88402a28c99a113601e9",
    "sport_key": "basketball_nba",
    "sport_title": "NBA",
    "commence_time": "2023-03-04T00:10:00Z",
    "home_team": "Charlotte Hornets",
    "away_team": "Orlando Magic",
    "bookmakers": [
        {
            "key": "draftkings",
            "title": "DraftKings",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:54:02Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 135
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -155
                        }
                    ]
                }
            ]
        },
        {
            "key": "mybookieag",
            "title": "MyBookie.ag",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:55:05Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 139
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -172
                        }
                    ]
                }
            ]
        },
        {
            "key": "fanduel",
            "title": "FanDuel",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:54:02Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 142
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -168
                        }
                    ]
                }
            ]
        },
        {
            "key": "circasports",
            "title": "Circa Sports",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:53:39Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 142
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -173
                        }
                    ]
                }
            ]
        },
        {
            "key": "betrivers",
            "title": "BetRivers",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:54:03Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 133
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -159
                        }
                    ]
                }
            ]
        },
        {
            "key": "sugarhouse",
            "title": "SugarHouse",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:54:03Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 133
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -159
                        }
                    ]
                }
            ]
        },
        {
            "key": "barstool",
            "title": "Barstool Sportsbook",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:55:02Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 133
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -159
                        }
                    ]
                }
            ]
        },
        {
            "key": "twinspires",
            "title": "TwinSpires",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:50:52Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 133
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -159
                        }
                    ]
                }
            ]
        },
        {
            "key": "unibet_us",
            "title": "Unibet",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:53:40Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 133
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -159
                        }
                    ]
                }
            ]
        },
        {
            "key": "foxbet",
            "title": "FOX Bet",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:54:02Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 138
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -182
                        }
                    ]
                }
            ]
        },
        {
            "key": "pointsbetus",
            "title": "PointsBet (US)",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:55:02Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 135
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -170
                        }
                    ]
                }
            ]
        },
        {
            "key": "betus",
            "title": "BetUS",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:55:03Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 140
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -160
                        }
                    ]
                }
            ]
        },
        {
            "key": "williamhill_us",
            "title": "William Hill (US)",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:54:03Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 135
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -160
                        }
                    ]
                }
            ]
        },
        {
            "key": "bovada",
            "title": "Bovada",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:53:39Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 145
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -170
                        }
                    ]
                }
            ]
        },
        {
            "key": "lowvig",
            "title": "LowVig.ag",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:54:26Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 144
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -164
                        }
                    ]
                }
            ]
        },
        {
            "key": "betonlineag",
            "title": "BetOnline.ag",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:55:04Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 144
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -164
                        }
                    ]
                }
            ]
        },
        {
            "key": "wynnbet",
            "title": "WynnBET",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:54:03Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 145
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -175
                        }
                    ]
                }
            ]
        },
        {
            "key": "betmgm",
            "title": "BetMGM",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:55:03Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 145
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -175
                        }
                    ]
                }
            ]
        },
        {
            "key": "superbook",
            "title": "SuperBook",
            "markets": [
                {
                    "key": "h2h",
                    "last_update": "2023-03-03T22:53:39Z",
                    "outcomes": [
                        {
                            "name": "Charlotte Hornets",
                            "price": 150
                        },
                        {
                            "name": "Orlando Magic",
                            "price": -170
                        }
                    ]
                }
            ]
        }
    ]
}
        "##;

        let outcome: api::Game = serde_json::from_str(response).unwrap();

        let outcome_bson = to_document(&outcome).unwrap();

        println!("{:#?}", outcome_bson);

        let insert_result = users_collection
            .insert_one(outcome_bson, None)
            .await
            .unwrap();
        println!("New document ID: {}", insert_result.inserted_id);
    }
}

// #[tokio::test]
// async fn test() {
//     let db = super::setup().await.unwrap();

//     let users_collection = db.collection::<Document>("users");
//     #[derive(Serialize, Deserialize)]
//     struct Person {
//         name: String,
//         age: i32,
//     }
//     let person = Person {
//         name: "Alice".to_string(),
//         age: 25,
//     };

//     let bson_doc = to_document(&person).unwrap();

//     let insert_result = users_collection.insert_one(bson_doc, None).await.unwrap();
//     println!("New document ID: {}", insert_result.inserted_id);
// }

// #[tokio::test]
// async fn test_04_insert_doc() {
//     let db = super::setup().await.unwrap();

//     let users_collection = db.collection::<Document>("users");

//     let new_doc = doc! {
//        "title": "Parasite",
//        "year": 2020,
//        "plot": "A poor family, the Kims, con their way into becoming the servants of a rich family, the Parks. But their easy life gets complicated when their deception is threatened with exposure.",
//     };

//     let insert_result = users_collection
//         .insert_one(new_doc.clone(), None)
//         .await
//         .unwrap();
//     println!("New document ID: {}", insert_result.inserted_id);
// }
// #[tokio::test]
// #[ignore]
// async fn test_users_get() {
//     let db = super::setup().await.unwrap();

//     let users_collection = db.collection::<Document>("users");

//     let user = match users_collection
//         .find_one(
//             doc! {
//                   "name": "Peter Parker"
//             },
//             None,
//         )
//         .await
//     {
//         Ok(res) => res.unwrap(),
//         Err(err) => panic!(),
//     };

//     println!("Result: {:#?}", user); // .Debug() -- not .Display()
// }

// #[tokio::test]
// #[ignore]
// async fn test_struct_doc() {
//     let db = super::setup().await.unwrap();

//     // Get a handle to a collection of `Book`.
//     let typed_collection = db.collection::<user::Data>("users");

//     let users = vec![
//         user::Data {
//             // id: ...
//             name: "Jim Snow".to_string(),
//         },
//         user::Data {
//             // id: ...
//             name: "Mary Jane".to_string(),
//         },
//     ];

//     // typed_collection.insert_many(users, None).await.unwrap();

//     // let user = match db
//     //     .collection::<user::Data>("users")
//     //     .find_one(
//     //         doc! {
//     //               "name": "Jim Snow"
//     //         },
//     //         //Some(users[0].data),
//     //         None,
//     //     )
//     //     .await
//     // {
//     //     Ok(res) => res.unwrap(),
//     //     Err(err) => panic!(),
//     // };

//     // println!("Result: {:#?}", user); // .Debug() -- not .Display()
// }

// #[tokio::test]
// #[ignore]
// async fn update_doc() {
//     let db = super::setup().await.unwrap();

//     let typed_collection = db.collection::<user::Data>("users");

//     let user = user::Data {
//         name: "Bruce Wayne".to_string(),
//     };

//     /*
//         1. Start with a struct, push struct to mongo
//         2a. Obtain the id from mongo for the new record created in #1 above, and update local struct for #3
//         2b. Query mongo and obtain the struct (query using regular find)
//         3. Update struct, and push it to mongo
//         4. Query mongo and obtain updated struct
//         5. Send mongo a delete command using the struct
//     */
//     // typed_collection.insert_one(user, None).await.unwrap();

//     let update_result = db
//         .collection::<user::Data>("users")
//         .update_many(
//             /*
//                 original_user.into<Option<Document>>().unwrap() --> Document,
//                 updated_user,
//             */
//             doc! {
//                   "name": "Bruce Wayne"
//             },
//             doc! {"$set": { "name": "Batman" }},
//             None,
//         )
//         .await
//         .unwrap();

//     println!("Result: {:#?}", update_result); // .Debug() -- not .Display()

//     assert!(update_result.modified_count > 0)
// }
// #[tokio::test]
// #[ignore]
// async fn delete_doc() {
//     let db = super::setup().await.unwrap();

//     let typed_collection = db.collection::<user::Data>("users");

//     let user = user::Data {
//         name: "To Be Removed".to_string(),
//     };

//     // let temp_doc = typed_collection.insert_one(user, None).await.unwrap();

//     // println!("New document ID: {}", temp_doc.inserted_id);

//     // Delete all documents for movies called "Parasite":
//     let delete_result = typed_collection
//         .delete_many(
//             doc! {
//                "name": "To Be Removed"
//             },
//             None,
//         )
//         .await
//         .unwrap();
//     println!("Deleted {} documents", delete_result.deleted_count);

//     // println!("Result: {:#?}", update_result); // .Debug() -- not .Display()
// }
