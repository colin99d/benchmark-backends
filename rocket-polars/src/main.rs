#[macro_use]
extern crate rocket;
use std::time::{Duration, SystemTime};
use polars::prelude::*;
use serde_json::Value;

pub fn csv_to_df(path: &str) -> PolarsResult<DataFrame> {
    CsvReader::from_path(path)?
        .has_header(true)
        .finish()?
        .with_row_count("Id", None)
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
    let raw_df = csv_to_df("../data.csv").unwrap();
    let loading = now.elapsed();
    let df = raw_df.slice(0, 50000);
    let slicing = now.elapsed();
    let to_return = df_to_json(&df);
    let converting = now.elapsed();
    println!("Loading: {:?}, Slicing: {:?}, Converting: {:?}", loading, slicing, converting);
    to_return
}


#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_data])
}
