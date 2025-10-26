use serde::{Deserialize,Serialize};
use std::error::Error;
use serde_json;

#[derive(Deserialize,Serialize)]
struct Paragraph {
    name:String
}
#[derive(Deserialize,Serialize)]
struct Article {
    article:String,
    author:String,
    paragraph:Vec<Paragraph>
}

fn main() {
    let json = r#"
    {
    "article": "The Future of Rust Programming",
    "author": "Danish Ansari",
    "paragraph": [
      {
        "name": "Rust is gaining popularity due to its memory safety and performance."
      },
      {
        "name": "It combines the speed of C++ with modern developer ergonomics."
      },
      {
        "name": "Rust’s ecosystem is growing rapidly with support for web, system, and blockchain development."
      }
    ]
    }"#;
    let parsed = read_json_data(json);
    println!("/n /n this is the first paragraph. {}",parsed.unwrap().paragraph[0].name);

}

fn read_json_data(json_data:&str)->Result<Article, Box<dyn Error>> {
    let parsed:Article  = serde_json::from_str(json_data)?;
    Ok(parsed)
}
