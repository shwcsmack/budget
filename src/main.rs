use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    print!("Welcome to ZBudget!");

    let selections = &[
        "New Budget",
        "Open Budget",
        "Exit",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("How would you like to start?")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    println!("Enjoy your {}!", selections[selection]);
}