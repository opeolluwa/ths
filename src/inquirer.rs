//utility class for parsing command line arguments
extern crate console;
extern crate dialoguer;

use console::Style;
use dialoguer::{theme::ColorfulTheme, Confirmation, Input, Select as DialougerSelect};

//define the structs, structs maps to  classes in Typescript
pub struct Prompt {}
pub struct Select {}
pub struct Confirm {}

//inquirer::Prompt used to prompt the user for input
impl Prompt {
    pub fn new(query: String) -> String {
        let theme = ColorfulTheme {
            values_style: Style::new().yellow().bright(),
            indicator_style: Style::new().yellow().bold(),
            yes_style: Style::new().yellow().bright(),
            no_style: Style::new().yellow().bright(),
            ..ColorfulTheme::default()
        };
        let answer: String = Input::with_theme(&theme)
            .with_prompt(&query)
            .interact()
            .unwrap();
        answer
    }
}

//Inquirer::Confirm used to confirm user selection cases of yes/no
impl Confirm {
    pub fn new(mut query: String) -> bool {
        query = query.to_string();
        let theme = ColorfulTheme {
            values_style: Style::new().yellow().bright(),
            indicator_style: Style::new().yellow().bold(),
            yes_style: Style::new().yellow().bright(),
            no_style: Style::new().yellow().bright(),
            ..ColorfulTheme::default()
        };

        let answer: bool = Confirmation::with_theme(&theme)
            .with_text(&query)
            .interact()
            .unwrap();
        answer
    }
}

//Inquirer :: Select use to select option from command line
impl Select {
    pub fn new(query: String, options: Vec<String>) -> String {
        let theme = ColorfulTheme {
            values_style: Style::new().yellow().bright(),
            indicator_style: Style::new().yellow().bold(),
            yes_style: Style::new().yellow().bright(),
            no_style: Style::new().yellow().bright()
            // picked_item_prefix: style("❯".to_string()).for_stderr().green(),
            // active_style: style("❯".to_string()).for_stderr().green(),
            ..ColorfulTheme::default()
        };
        let index = DialougerSelect::with_theme(&theme)
            .with_prompt(&query)
            .default(0)
            .items(&options[..])
            .interact()
            .unwrap();

        options[index].to_string()
    }
}

