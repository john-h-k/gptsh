use std::env;

mod key_selector;
mod openai_interface;
mod prompt_generator;

use openai_interface::{OpenAiContext, complete};

#[async_std::main]
async fn main() {
    let context = OpenAiContext {
        api_key: key_selector::select_default_key().await
    };

    let base_prompt = env::args().skip(1).collect::<Vec<String>>().join(" ");

    let prompt = prompt_generator::generate_prompt(&base_prompt);

    match complete(&context, &prompt).await {
        Ok(result) => {
            if let Some(choice) = result["choices"][0]["text"].as_str() {
                println!("{}", choice.trim());
            }
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
