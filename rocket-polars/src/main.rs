#[macro_use]
extern crate rocket;
use polars::prelude::*;
use serde_json::Value;

pub fn csv_to_df(path: &str) -> PolarsResult<DataFrame> {
    CsvReader::from_path(path)?
        .has_header(true)
        .finish()?
        .with_row_count("Id", None)
}

pub fn df_to_json(df: &DataFrame) -> Result<Value, String> {
    let mut buffer = Vec::new();

    if let Err(_) = JsonWriter::new(&mut buffer)
        .with_json_format(JsonFormat::Json)
        .finish(&mut df.clone())
    {
        return Err("Could not write the dataframe to json".to_string());
    }

    match String::from_utf8(buffer) {
        Err(_) => Err("Could not convert the buffer to a string".to_string()),
        Ok(json) => match serde_json::from_str(&json) {
            Err(_) => Err("Could not convert the string to json".to_string()),
            Ok(json) => Ok(json),
        },
    }
}
#[get("/")]
pub async fn get_data() -> Value {
    let raw_df = csv_to_df("../data.csv").unwrap();
    let df = raw_df.slice(0, 50000);
    df_to_json(&df).unwrap()
}


#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_data])
}
