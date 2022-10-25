#[macro_use]
extern crate rocket;
use std::time::{SystemTime};
use std::path::Path;
use polars::prelude::*;
use rocket::http::ContentType;

pub fn csv_to_df(path: &str) -> DataFrame {
    LazyCsvReader::new(Path::new(path))
        .has_header(true)
        .finish()
        .unwrap()
        .slice(0, 50_000)
        .collect()
        .unwrap()
        //.filter(col("bar").gt(lit(100)))
        // .with_row_count("Id", None)
}

pub fn df_to_json(df: &DataFrame) -> String {
    let mut buffer = Vec::new();

    JsonWriter::new(&mut buffer)
        .with_json_format(JsonFormat::Json)
        .finish(&mut df.clone())
        .unwrap();

    String::from_utf8(buffer).unwrap()
}


#[get("/")]
pub async fn get_data() -> (ContentType, String) {
    let now = SystemTime::now();
    let raw_df = csv_to_df("../data.csv");
    let loading = now.elapsed().unwrap();
    let to_return = df_to_json(&raw_df);
    let converting = now.elapsed().unwrap();
    println!("Loading: {:?}, Converting: {:?}", loading, converting - loading);
    (ContentType::JSON, to_return)
}


#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_data])
}
