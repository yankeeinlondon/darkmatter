#![allow(unused)]
pub mod database;

use database::Database;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use surrealdb::sql::{thing, Datetime, Object, Thing, Value};
use surrealdb::{Datastore, Response, Session};

type DB = (Datastore, Session);

#[derive(Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
enum SourceState {
    Initialized,
    Done,
}

struct Source {
    /// The unique id for a source. When the source is a file then
    /// it should be a relative path to the file from the base of
    /// the repository.
    id: String,

    title: String,
    /// The payload/content of the file
    content: String,
    hash: Option<i64>,

    state: SourceState,
}

async fn add_source(&(ref ds, ref ses): &DB, title: &str, content: &str) -> Result<String> {
    let sql = "CREATE source CONTENT $data";
    let data: BTreeMap<String, Value> = [
        ("title".into(), title.into()),
        ("content".into(), content.into()),
        ("state".into(), "initialize".into()),
    ]
    .into();

    let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();
    let resp = ds.execute(sql, ses, Some(vars), false).await?;

    // for object in into_iter_objects(resp)? {
    //     println!("record {}", object?);
    // }

    into_iter_objects(resp)?
        .next()
        .transpose()?
        .and_then(|obj| obj.get("id").map(|id| id.to_string()))
        .ok_or_else(|| anyhow!("No ID was returned!"))
}

fn into_iter_objects(resp: Vec<Response>) -> Result<impl Iterator<Item = Result<Object>>> {
    let r = resp
        .into_iter()
        .next() //
        .map(|item| item.result)
        .transpose()?;

    match r {
        Some(Value::Array(arr)) => {
            let it = arr.into_iter().map(|v| match v {
                Value::Object(object) => Ok(object),
                _ => Err(anyhow!("record was not an object!")),
            });
            Ok(it)
        }
        _ => Err(anyhow!("No records found.")),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");
    let db: &DB = &(
        Datastore::new("memory").await?,
        Session::for_db("my_ns", "my_db"),
    );

    // let sql = "CREATE task:1 SET title = 'Task 1', priority = 10";
    // let resp = ds.execute(sql, ses, None, false).await?;
    // let sql = "CREATE task:2 SET title = 'Task 2', priority = 5";
    // let resp = ds.execute(sql, ses, None, false).await?;

    // let sql = "CREATE task:2 SET title = 'Task 2', priority = 5";
    // let resp = ds.execute(sql, ses, None, false).await?;

    // // Select
    // let sql = "SELECT * from task";
    // let resp = ds.execute(sql, ses, None, false).await?;
    // println!("{resp:?}");

    let s1 = add_source(db, "foobar.md", "once upon a time").await?;
    let s2 = add_source(db, "baz_bat.md", "they lived happily ever after").await?;

    // let u1 = update_source(db, s1);

    println!("{s1}, {s2}");

    // let table = create_table("foobar", struct, db);
    // table.add(rp1, rp2, rp3, options);
    // table.delete(id);
    // table.merge()

    let db2 = Database::From("darkmatter.db");

    Ok(())
}
