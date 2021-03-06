extern crate base64;
extern crate imap;
extern crate native_tls;
extern crate percent_encoding;
extern crate reqwest;

mod oauth2;
mod users;
use native_tls::TlsConnector;
use oauth2::read_authorize_info;
use users::User;

#[derive(Debug)]
struct GmailOAuth2 {
    user: String,
    access_token: String,
}

impl imap::Authenticator for GmailOAuth2 {
    type Response = String;
    #[allow(unused_variables)]
    fn process(&self, data: &[u8]) -> Self::Response {
        format!(
            "user={}\x01auth=Bearer {}\x01\x01",
            self.user, self.access_token
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let auth = oauth2::authorization().await?;
    let mut user = match User::new() {
        Ok(u) => u,
        Err(_err) => {
            panic!("user init failed");
        }
    };
    user.token = String::from(&auth);
    user.write_to_file()?;
    let gmail_auth = GmailOAuth2 {
        user: user.email,
        access_token: user.token,
    };
    let domain = "imap.gmail.com";
    let port = 993;
    let socket_addr = (domain, port);
    let ssl_connector = TlsConnector::builder().build().unwrap();
    let client = imap::connect(socket_addr, domain, &ssl_connector).unwrap();

    let mut imap_session = match client.authenticate("XOAUTH2", &gmail_auth) {
        Ok(c) => c,
        Err((e, _unauth_client)) => {
            panic!("error authenticating: {}", e);
        }
    };

    match imap_session.select("INBOX") {
        Ok(mailbox) => println!("{}", mailbox),
        Err(e) => println!("Error selecting INBOX: {}", e),
    };

    match imap_session.fetch("2", "body[text]") {
        Ok(msgs) => {
            for msg in &msgs {
                print!("{:?}", msg);
            }
        }
        Err(e) => println!("Error Fetching email 2: {}", e),
    };
    imap_session.logout().unwrap();
    Ok(())
}
