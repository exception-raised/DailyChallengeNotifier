
pub mod types;
pub mod constants;

use dotenv::dotenv;
use crate::types::{ Config, QuestionData };
use crate::constants::{ QUERY, URL };
use std::collections::HashMap;
use lettre::{Transport, SmtpTransport, Message};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;

pub fn load_env_variables() -> Config {
    dotenv().ok();

    let smtp_username = std::env::var("SMTP_USERNAME").unwrap();
    let smtp_password = std::env::var("SMTP_PASSWORD").unwrap();
    let smtp_server =   std::env::var("SMTP_SERVER").unwrap();
    let target_email =   std::env::var("TARGET_EMAIL").unwrap();

    Config {
        smtp_username,
        smtp_password,
        smtp_server,
        target_email
    }
}

async fn get_daily_question_data() -> Result<QuestionData, Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("query", QUERY);
    map.insert("operationName", "questionOfToday");

    let client = reqwest::Client::new();
    let resp = client.get(URL)
        .json(&map)
        .send()
        .await?;

    
    let json_body = resp.text().await?;
    
    let question_data: QuestionData = serde_json::from_str(&json_body)?;
    
    Ok(question_data)
}

fn get_color_based_on_difficulty(difficulty: &String) -> &'static str {
    match difficulty.as_str() {
        "Easy" => "green",
        "Medium" => "yellow",
        "Hard" => "red",
        _ => "black",
    }
}

fn format_html(question: QuestionData) -> String {
    let topic_tags_html: String = question.data.active_question.question.topic_tags.iter()
        .map(|tag| format!("<span style=\"display: inline-block; background-color: #ffa116; color: black; padding: 5px 10px; margin: 5px; border-radius: 5px;\">{}</span>", tag.name))
        .collect::<Vec<String>>()
        .join("");

    let html: String = format!(
        r#"<!DOCTYPE html>
        <html lang="en">
        <body style="background-color:#19191b; font-family: Arial, Helvetica, sans-serif;">
            <div style="display: flex; flex-direction: column; color:white; background-color:#28282b; width:30em; border-radius: 10px; margin: 0 auto;">
                
                <div style="width:100%; text-align:center; border-bottom:5px solid #19191b;">
                    <h1>{}</h1>
                </div>

                <div style="display: flex; justify-content: space-between; padding: 0px 20px;">
                    <h4 style="margin: 10px;">Difficulty: <span style="color: {};">{}</span></h4>
                    <h4 style="margin: 10px;">Acceptance Rate: <span style="color:#ffa116">{}%</span></h4>
                </div>

                <div style="display: flex; justify-content: space-between; padding: 0px 20px;">
                    <h4 style="margin: 10px;">Is Favorite: <span style="color:#ffa116">{}</span></h4>
                    <h4 style="margin: 10px;">Paid Only: <span style="color:#ffa116">{}</span></h4>
                </div>

                <div style="display: flex; justify-content: flex-start; flex-wrap: wrap; padding: 0px 20px;">
                    <h4 style="margin: 10px;">Topic Tags: {}</h4>
                </div>

                <div style="display: flex; justify-content: center; margin-bottom: 20px;">
                    <a href="https://leetcode.com{}" style="display: inline-block; width: 200px; height: 50px; line-height: 50px; text-align: center; text-decoration: none; background-color: #ffa116; border-radius: 10px; font-size: 20px; color: black; border: 2px solid #ffa116;>
                        Go to question
                    </a>
                </div>
            
            </div>
        </body>
        </html>"#,    
            
            question.data.active_question.question.title,
            get_color_based_on_difficulty(&question.data.active_question.question.difficulty),
            question.data.active_question.question.difficulty,
            format!("{:.2}", question.data.active_question.question.ac_rate),
            question.data.active_question.question.is_favor,
            question.data.active_question.question.is_paid_only,
            topic_tags_html,
            question.data.active_question.link
        );

    html
}



fn send_email(config: &Config, html: &str) {
    let smtp_credentials = Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());

    let email = Message::builder()
        .from(format!("Daily Challenge Notifier <{}>", config.target_email).parse().unwrap())
        .to(format!("Daily Challenge Notifier <{}>", config.target_email).parse().unwrap())
        .subject("Leetcode Daily Challenge")
        .header(ContentType::TEXT_HTML)
        .body(String::from(html))
        .unwrap();
    
    let smtp_transport = SmtpTransport::starttls_relay(&config.smtp_server)
        .unwrap()
        .credentials(smtp_credentials)
        .build();

    match smtp_transport.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => eprintln!("Failed to send email: {}", e),
    }
}

pub async fn entrypoint(arg: &str) {
    let config: Config = load_env_variables();
    match get_daily_question_data().await {
        Ok(question_data) => {
            let html = format_html(question_data);

            send_email(&config, &html);
        }
        Err(err) => {
            eprintln!("Error getting daily question data: {}", err);
        }
    }
}

