use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;

#[derive(Debug, Deserialize, Serialize)]
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

    pub fn write_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create("./acc.txt")?;
        let writer = BufWriter::new(file);
        let temp = &self;
        serde_json::to_writer(writer, &self)?;
        println!("{:#?}", &self);
        println!("write to file");
        Ok(())
    }
}
