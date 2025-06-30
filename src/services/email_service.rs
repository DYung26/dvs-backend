use reqwest::Client;
use crate::{
    utils::error::AppError,
    dto::email::{
        EmailMessage, SendGridPersonalization, SendGridEmail,
        SendGridContent, SendGridPayload, VerificationEmailContext,
    },
};
use axum::http::StatusCode;
use tera::{Tera, Context};

pub struct EmailService {
    client: Client,
    api_key: String,
    from_email: String,
    from_name: String,
}

impl EmailService {
    pub fn new(api_key: String, from_email: String, from_name: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            from_email,
            from_name,
        }
    }

    pub async fn send_mail(&self, msg: EmailMessage) -> Result<(), AppError> {
        let payload = SendGridPayload {
            personalizations: vec![SendGridPersonalization {
                to: vec![SendGridEmail {
                    email: msg.to.clone(),
                    name: Some(msg.to_name.clone()),
                }],
            }],
            from: SendGridEmail {
                email: self.from_email.clone(),
                name: Some(self.from_name.clone()),
            },
            subject: msg.subject.clone().unwrap(),
            content: vec![
                SendGridContent {
                    content_type: "text/plain".to_string(),
                    value: msg.plain_text.clone().unwrap(),
                },
                SendGridContent {
                    content_type: "text/html".to_string(),
                    value: msg.html_body.clone().unwrap(),
                },
            ],
        };

        let res = self.client
            .post("https://api.sendgrid.com/v3/mail/send")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to send email", Some(e.to_string())))?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("SendGrid failed with status {}: {}", status, text),
                Some(text),
            ));
        }

        Ok(())
    }

    pub async fn send_verification_email(&self, msg: EmailMessage) -> Result<(), AppError> {
        // Create template context
        /*let mut context = Context::new();
        context.insert("first_name", &msg.to_name);
        context.insert("otp", &msg.otp.clone().unwrap_or_default());
        context.insert("email_from", &self.from_name);*/
        let context = Context::from_serialize(VerificationEmailContext {
            first_name: msg.to_name.clone(),
            otp: msg.otp.clone(), // .unwrap_or_default(),
            email_from: self.from_name.clone(),
        }).map_err(|e| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Template serialization failed", Some(e.to_string()))
        })?;

        // Load + render template
        let tera = Tera::new("src/templates/**/*").map_err(|e| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Template parsing failed", Some(e.to_string()))
        })?;

        let html = tera.render("verification_email.html", &context).map_err(|e| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Template rendering failed", Some(e.to_string()))
        })?;

        let subject = "Verify your Email";

        self.send_mail(EmailMessage {
            to: msg.to.clone(),
            to_name: msg.to_name.clone(),
            subject: Some(subject.to_string()),
            plain_text: Some("".to_string()),
            html_body: Some(html),
            otp: msg.otp.clone(),
        }).await
    }
}
