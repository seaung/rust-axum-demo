use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Lang {
    text: String,
    version: f32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();

    println!("serialized = {}", serialized);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    println!("deserialized = {:?}", deserialized);

    let lang = Lang{ text: "rust-lang".to_owned(), version: 1.77};

    let json_lang = serde_json::to_string(&lang).unwrap();

    println!("the lang string {}", json_lang);

    let lang_obj: Lang = serde_json::from_str(&json_lang).unwrap();
    println!("the lang json object {:?}", lang_obj);
}
