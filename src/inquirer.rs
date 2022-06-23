//utility class for parsing command line arguments
extern crate dialoguer;
use dialoguer::{theme::CustomPromptCharacterTheme, Confirmation, Input, Select};

pub struct Inquirer {}
impl Inquirer {
    pub fn prompt(query: String) -> String {
        let theme = CustomPromptCharacterTheme::new('>');
        let answer: String = Input::with_theme(&theme)
            .with_prompt(&query)
            .interact()
            .unwrap();
        answer
    }

    pub fn confirm(query: String) -> bool {
        let answer: bool = Confirmation::new().with_text(&query).interact().unwrap();
        answer
    }

    pub fn select(query: String, options: Vec<String>) -> String {
        let theme = CustomPromptCharacterTheme::new('>');
        let answer = Select::with_theme(&theme)
            .with_prompt(&query)
            .default(0)
            .items(&options[..])
            .interact()
            .unwrap();
        answer.to_string()
    }
}
