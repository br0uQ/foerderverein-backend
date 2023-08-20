use lettre::{
    transport::smtp::{response::Response, Error},
    Message, SmtpTransport, Transport,
};

static TO_ADDRESS: &str =
    "Förderverein Arche Lichtenstein <info@foerderverein-arche-lichtenstein.de>";
static FROM_ADDRESS: &str = "Website <website@foerderverein-arche-lichtenstein.de>";

pub fn send_mail(subject: String, body: String) -> Result<Response, Error> {
    let email = Message::builder()
        .from(FROM_ADDRESS.parse().unwrap())
        //.reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
        .to(TO_ADDRESS.parse().unwrap())
        .subject(subject.as_str())
        .body(body)
        .unwrap();

    // Create TlS transport on port 587 with STARTTLS
    // let sender = SmtpTransport::starttls_relay("smtp.example.com").unwrap()
    let sender = SmtpTransport::unencrypted_localhost();
    //// Add credentials for authentication
    //.credentials(Credentials::new(
    //        "username".to_owned(),
    //        "password".to_owned(),
    //        ))
    //// Configure expected authentication mechanism
    //.authentication(vec![Mechanism::Plain])

    sender.send(&email)
}
