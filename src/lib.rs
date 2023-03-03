use slack_flows::{listen_to_channel, send_message_to_channel};
use openai_flows::{CompletionRequest, create_completion};
use flowsnet_platform_sdk::write_error_log;

#[no_mangle]
pub fn run() {
    listen_to_channel("secondstate", "rust-code-review", |sm| {
        write_error_log!("Received: ".to_owned() + &sm.text);
        let cr = CompletionRequest {
            prompt: sm.text + "\n\n\"\"\"\n\nWhat the above Rust source code is doing?\n1. ",
            max_tokens: 7000,
            ..Default::default()
        };

        let r = create_completion("Agent", cr);
        r.iter().for_each(|c| {
            write_error_log!("Generated: ".to_owned() + c);
            send_message_to_channel("secondstate", "rust-code-review", "1. ".to_string() + c);
        });
    });
}
