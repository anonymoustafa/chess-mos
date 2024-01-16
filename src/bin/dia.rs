use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    let choices = vec!["Option 1", "Option 2", "Option 3"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option:")
        .items(&choices)
        .default(0) // Default selection (optional)
        .interact()
        .unwrap();

    println!("You chose: {}", choices[selection]);
}