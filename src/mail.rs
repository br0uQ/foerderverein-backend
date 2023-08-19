use std::str::FromStr;

use lettre::{
    transport::smtp::{
        authentication::{Credentials, Mechanism},
        PoolConfig,
    },
    Message, SmtpTransport, Transport,
};

pub fn send_mail() -> Result<lettre::transport::smtp::response::Response, lettre::transport::smtp::Error> {
    let email = Message::builder()
        .from("NoBody <nobody@domain.tld>".parse().unwrap())
        .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
        .to("Hei <hei@domain.tld>".parse().unwrap())
        .subject("Happy new year")
        .body(String::from("Be happy!")).unwrap();

    // Create TlS transport on port 587 with STARTTLS
    let sender = SmtpTransport::starttls_relay("smtp.example.com").unwrap()
        // Add credentials for authentication
        .credentials(Credentials::new(
                "username".to_owned(),
                "password".to_owned(),
                ))
        // Configure expected authentication mechanism
        .authentication(vec![Mechanism::Plain])
        // Connection pool settings
        .pool_config(PoolConfig::new().max_size(20))
        .build();

    sender.send(&email)
}
