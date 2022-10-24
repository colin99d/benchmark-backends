#[macro_use]
extern crate rocket;
use std::time::{SystemTime};
use std::path::Path;
use polars::prelude::*;
use serde_json::Value;

pub fn csv_to_df(path: &str) -> DataFrame {
    LazyCsvReader::new(Path::new(path))
        .has_header(true)
        // .with_n_rows(Some(50_000))
        .finish()
        .unwrap()
        .slice(0, 50_000)
        .collect()
        .unwrap()
        //.filter(col("bar").gt(lit(100)))
        // .with_row_count("Id", None)
}

pub fn df_to_json(df: &DataFrame) -> Value {
    let mut buffer = Vec::new();

    JsonWriter::new(&mut buffer)
        .with_json_format(JsonFormat::Json)
        .finish(&mut df.clone())
        .unwrap();

    let json_string = String::from_utf8(buffer).unwrap();
    serde_json::from_str(&json_string).unwrap()
}


#[get("/")]
pub async fn get_data() -> Value {
    let now = SystemTime::now();
    let raw_df = csv_to_df("../data.csv");
    let loading = now.elapsed().unwrap();
    let to_return = df_to_json(&raw_df);
    let converting = now.elapsed().unwrap();
    println!("Loading: {:?}, Converting: {:?}", loading, converting - loading);
    to_return
}


#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_data])
}
