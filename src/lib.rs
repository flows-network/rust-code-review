use slack_flows::{listen_to_channel, send_message_to_channel};
use flowsnet_platform_sdk::write_error_log;
use openai_flows::{chat_completion, ChatOptions};
use std::env;
use dotenv::dotenv;

#[no_mangle]
pub fn run() {
    dotenv().ok();
    let workspace: String = match env::var("workspace") {
        Err(_) => "secondstate".to_string(),
        Ok(name) => name,
    };

    let channel: String = match env::var("channel") {
        Err(_) => "rust-code-review".to_string(),
        Ok(name) => name,
    };
    let openai_key_name: String = match env::var("openai_key_name") {
        Err(_) => "chatmichael".to_string(),
        Ok(name) => name,
    };


    listen_to_channel(&workspace, &channel, |sm| {
        write_error_log!("Received: ".to_string() + sm.text.to_lowercase().as_str());
        match sm.text.to_lowercase().as_str() {
            "help" => {
                write_error_log!("HELP".to_string());
                send_message_to_channel(&workspace, &channel, "To start, just send a code snippet as a message. The bot will review it for you, and then you can ask follow-up questions. To stop a review session and start a new one, type \"restart\". Each session expires after 10 minutes of inactivity. Please send in a code snippet in Rust in your next message.".to_string());
            },

            "restart" => {
                let co = ChatOptions {
                    restart: true,
                    restarted_sentence: Some("You will act as a reviewer for Rust code.")
                };
                if let Some(r) = chat_completion(&openai_key_name, &format!("rust-code-review-{}", &channel), "", &co) {
                    write_error_log!(r.choice);
                    send_message_to_channel(&workspace, &channel, "Ok, let's start a new code review session. Please send in a code snippet in Rust in your next message.".to_string());
                }
            },

            _ => {
                let prompt = "I will send you a snippet of Rust source code. Please review it, describe what it does, and report whether there are errors or other issues in the code. Here is the Rust code snippet:\n\n".to_owned() + "'''\n" + &sm.text + "\n'''";
                let co = ChatOptions {
                    restart: false,
                    restarted_sentence: Some(&prompt)
                };
                if let Some(r) = chat_completion(&openai_key_name, &format!("rust-code-review-{}", &channel), &sm.text, &co) {
                    send_message_to_channel(&workspace, &channel, r.choice);
                }
            },

        };

    });
}
