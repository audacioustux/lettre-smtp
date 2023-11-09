use clap::Parser;
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use lettre_smtp::config::Config;

fn main() {
    let Config {
        smtp_from,
        smtp_host,
        smtp_password,
        smtp_username,
        subject,
        body,
        mail_to,
    } = Config::parse();

    let creds = Credentials::new(smtp_username.into(), smtp_password.into());

    let email = Message::builder()
        .from(smtp_from.parse().unwrap())
        .to(mail_to.parse().unwrap())
        .subject(subject)
        .body(body)
        .unwrap();

    let mailer = SmtpTransport::relay(&smtp_host)
        .unwrap()
        .credentials(creds)
        .build();

    mailer.send(&email).unwrap();
}
