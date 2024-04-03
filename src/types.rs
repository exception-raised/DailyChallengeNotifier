
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    pub ac_rate: f64,
    pub difficulty: String,
    pub title: String,
    pub is_paid_only: bool,
    pub is_favor: bool,
    pub topic_tags: Vec<TopicTags>,
}

#[derive(Deserialize)]
pub struct TopicTags {
    pub name: String,
}

#[derive(Deserialize)]
pub struct QuestionDataContainer {
    pub link: String,
    pub question: Question,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(alias = "activeDailyCodingChallengeQuestion")]
    pub active_question: QuestionDataContainer,
}

#[derive(Deserialize)]
pub struct QuestionData {
    pub data: Data,
}

pub struct Config {
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_server: String,
    pub target_email: String,
}
