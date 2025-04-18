use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};
use anyhow::{Error, Result};
use crate::config::Config;
use crate::models::user::{Email, Name};
use crate::models::user::ValidationCode;

pub async fn send_email(config: &Config, name: &Name, email_address_to: &Email, validation_code: &ValidationCode) -> Result<()> {

    let email = Message::builder()
        .from(config.email_from_address.parse()?)
        .reply_to(format!("noreply <{}>", config.email_from_address).parse()?)
        .to(format!("{} <{}>", name, &email_address_to).parse()?)
        .subject("Here is your validation code for the requested action on the Quoridor server")
        .body(String::from(format!("Validation code: {}", validation_code)))?;

    // Open a remote connection using STARTTLS
    let creds = Credentials::new(config.email_smtp_username.clone(), config.email_smtp_password.clone());
    let mailer: AsyncSmtpTransport<Tokio1Executor> =
    AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.email_smtp_host)?
        .credentials(creds)
        .build();

    // Send the email
    mailer.send(email).await?;
    
    Ok(())
}