pub fn generate_prompt(base_prompt: &str) -> String {
    format!(
        "Generate a zsh command for macOS that will fulfil the following task: {}",
        base_prompt
    )
}
