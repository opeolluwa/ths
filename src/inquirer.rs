//utility class for parsing command line arguments
extern crate dialoguer;
use dialoguer::{theme::CustomPromptCharacterTheme, Confirmation, Input, Select as DialougerSelect};

// pub struct Inquirer {}
pub struct Prompt {}
pub struct Select {}
pub struct Confirm {}

//inquirer::Prompt used to prompt the user for input
impl Prompt {
    pub fn new(query: String) -> String {
        let theme = CustomPromptCharacterTheme::new('>');
        let answer: String = Input::with_theme(&theme)
            .with_prompt(&query)
            .interact()
            .unwrap();
        answer
    }
}

//Inquirer::Confirm used to confirm user selection cases of yes/no
impl Confirm {
    pub fn new(query: String) -> bool {
        let answer: bool = Confirmation::new().with_text(&query).interact().unwrap();
        answer
    }
}

impl Select {
    pub fn new(query: String, options: Vec<String>) -> String {
        let theme = CustomPromptCharacterTheme::new('>');
        let answer = DialougerSelect::with_theme(&theme)
            .with_prompt(&query)
            .default(0)
            .items(&options[..])
            .interact()
            .unwrap();
        answer.to_string()
    }
}

// impl Inquirer {}
