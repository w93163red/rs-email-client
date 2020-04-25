use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
pub struct User {
    email: String,
    token: String,
}

impl User {
    pub fn new() -> Result<User, Box<dyn std::error::Error>> {
	let mut email: String = String::default();
	let mut token: String = String::default();
	match read_lines("./acc.txt") {
	    Ok(lines) => {
		let mut count: i32 = 0;
		for line in lines {
		    if let Ok(l) = line {
			match count {
			    0 => {email = l;}
			    1 => {token = l;}
			    _ => {}
			}
			count += 1;
		    }
		}
	    }
	    Err(err) => {
		eprintln!("{}", err);
	    }
	};
	Ok(User{email, token})
    }
} 

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


