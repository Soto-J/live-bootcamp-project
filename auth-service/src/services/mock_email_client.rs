use crate::domain::{Email, EmailClient};
use color_eyre::eyre;
use color_eyre::eyre::Ok;
use secrecy::ExposeSecret;

pub struct MockEmailClient;

#[async_trait::async_trait]
impl EmailClient for MockEmailClient {
    #[tracing::instrument(name = "Send_Email", skip_all)]
    async fn send_email(
        &self,
        recipient: &Email,
        subject: &str,
        content: &str,
    ) -> eyre::Result<String> {
        tracing::debug!(
            "Sending email to {} with subject: {} and content: {}",
            recipient.as_ref().expose_secret(),
            subject,
            content
        );

        Ok("mock-email-sent".into())
    }
}
