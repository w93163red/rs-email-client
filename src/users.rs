use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
pub struct User {
    pub email: String,
    pub token: String,
}

impl User {
    pub fn new() -> Result<User, Box<dyn std::error::Error>> {
        let file = File::open("./acc.txt")?;
        let reader = BufReader::new(file);
        let u = serde_json::from_reader(reader)?;

        println!("{:?}", u);
        Ok(u)
    }
}
