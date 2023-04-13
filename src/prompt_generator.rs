pub fn generate_prompt(base_prompt: &str) -> String {
    format!(
        "Generate a bash command that will fulfil the following task: {}",
        base_prompt
    )
}
