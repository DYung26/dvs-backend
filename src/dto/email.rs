use serde::Serialize;

#[derive(Debug)]
pub struct EmailMessage {
    pub to: String,
    pub to_name: String,
    pub subject: Option<String>,
    pub plain_text: Option<String>,
    pub html_body: Option<String>,
    pub otp: String,
}

#[derive(Serialize)]
pub struct SendGridPersonalization {
    pub to: Vec<SendGridEmail>,
}

#[derive(Serialize)]
pub struct SendGridEmail {
    pub email: String,
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct SendGridContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub value: String,
}

#[derive(Serialize)]
pub struct SendGridPayload {
    pub personalizations: Vec<SendGridPersonalization>,
    pub from: SendGridEmail,
    pub subject: String,
    pub content: Vec<SendGridContent>,
}

#[derive(serde::Serialize)]
pub struct VerificationEmailContext {
    pub first_name: String,
    pub otp: String,
    pub email_from: String,
}
