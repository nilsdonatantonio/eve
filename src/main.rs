extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use eve::{get_n_words, import_table};
use inquire::CustomType;

fn main() {
    if let Err(e) = run() {
        eprintln!("Erreur: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    import_table("/home/enigma/Projects/passphrase_generator/wordlist-fr.txt")?;
    let num_of_words = CustomType::<u32>::new("Combien de mots doivent composer la passphrase ?")
        .with_help_message("8 mots ~= 90 bits d'entropie")
        .prompt()?;

    let passphrase = get_n_words(num_of_words)?;

    println!("La passphrase est: {passphrase}");

    Ok(())
}
