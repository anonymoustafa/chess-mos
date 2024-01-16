use dialoguer::{theme::ColorfulTheme, Select};
fn main() {
greetings();
    // while true {
    // println!("hello");
}


fn greetings() {
    // initiating a game
    let choices = vec!["Continue a previous game", "New game!", "From a snapshot"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option:")
        .items(&choices)
        .default(0) // Default selection (optional)
        .interact()
        .unwrap();
ret
    println!("You chose: {}", choices[selection]);
// end of initiating a game


}