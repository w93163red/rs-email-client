#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate futures;
extern crate tokio;

use futures::executor::block_on;
use tokio::prelude::*;

error_chain! {
   foreign_links {
      Io(std::io::Error);
      HttpRequest(reqwest::Error);
   }
}

async fn async_main() -> Result<()> {
   let body = reqwest::get("http://httpbin.org/get").await?.text().await?;
   println!("Body:\n{}", body);

   Ok(())
}

#[tokio::main]
async fn main() {
   async_main().await;
}

