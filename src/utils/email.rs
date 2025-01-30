use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use anyhow::Result;

pub fn send_email(
    to: &str,
    subject: &str,
    body: &str,
    from: &str,
    smtp_host: &str,
    smtp_username: &str,
    smtp_password: &str,
) -> Result<()> {
    let email = Message::builder()
        .from(from.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body.to_string())?;

    let creds = Credentials::new(smtp_username.to_string(), smtp_password.to_string());
    let mailer = SmtpTransport::relay(smtp_host)?.credentials(creds).build();

    mailer.send(&email)?;
    Ok(())
}