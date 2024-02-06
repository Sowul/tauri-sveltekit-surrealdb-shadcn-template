use std::path::Path;

use crate::model::base::{Base, ResponseBase};

use crate::error::Error;

use tauri::State;

use surrealdb::engine::local::{Db, File};
use surrealdb::opt::auth::Database;
use surrealdb::Surreal;

#[derive(Clone)]
pub struct DbCxn {
    pub client: Surreal<Db>,
}

impl DbCxn {
    pub async fn init(filename: &str) -> Result<Self, Error> {
        let db_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(filename);
        println!("DB Path: {:?}", db_path);

        let client = Surreal::new::<File>(db_path).await?;

        client
            .signin(Database {
                username: "root",
                password: "root",
                database: "test.db",
                namespace: "test",
            })
            .await?;

        Ok(DbCxn { client })
    }
}

#[tauri::command]
pub async fn query(sql: &str, state: State<'_, DbCxn>) -> Result<Vec<Base>, Error> {
    println!("pub async fn query");
    let client = &state.client;
    let mut resp = client.query(sql).await?;
    println!("resp: {:?}", resp);

    // Convert the response into a Vec<Response>
    let temp_res: Vec<ResponseBase> = resp.take(0)?;

    // Convert Vec<Response> into Vec<Base>
    let res: Vec<Base> = temp_res
        .into_iter()
        .map(|r| Base {
            id: r.id.to_raw(),
            name: r.name,
            created_at: r.created_at,
            last_updated: r.last_used,
        })
        .collect();

    println!("res: {:?}", res);

    Ok(res)
}
