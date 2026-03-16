//! Diceware en français avec gestion des jets de dés
//!
//! Se base sur la liste alternative de [`ArthurPons`]:
//! - Les mots ne contiennent pas d'accents
//! - Pas d'insultes, de mots offensants, etc.
//!
//! Les jets de dés utilisent un CSPRNG vérifié ([`rand::StdRng`]) et zéroizent la mémoire après
//! chaque génération
//!
//! [`ArthurPons`]: https://github.com/ArthurPons/diceware-fr-alt/blob/master/diceware-fr-alt.txt
//! [`rand::StdRng`]: https://docs.rs/rand/latest/rand/rngs/struct.StdRng.html

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::OnceLock;

use rand::{
    RngExt, SeedableRng,
    rngs::{StdRng, SysRng},
};

static WORDTABLE: OnceLock<HashMap<u32, String>> = OnceLock::new();

// Tire un seul dé, retourne une valeur entre 1 et 6
fn roll_dice() -> Result<u32, anyhow::Error> {
    let mut rng = StdRng::try_from_rng(&mut SysRng)?;
    let val: u32 = rng.random_range(1..=6);
    Ok(val)
}

// Tire 5 dés et les combinent en une valeur unique de 5 digits (ex. 34152)
fn get_dice_round() -> Result<u32, anyhow::Error> {
    let mut val: u32 = 0;
    let base: u32 = 10;
    for i in (0..5).rev() {
        val += roll_dice()? * base.pow(i);
    }
    Ok(val)
}

// Recherche le mot correspondant au résultat du tir des 5 dés
fn lookup_word(dice_round: u32) -> Option<&'static str> {
    WORDTABLE.get()?.get(&dice_round).map(|s| s.as_str())
}

/// Génère une passphrase de `num_of_words` séparés par un tiret '-'
///
/// # Erreurs
/// Retourne une erreur si un jet de dé ne peu être mappé à un mot
pub fn get_n_words(num_of_words: u32) -> Result<String, anyhow::Error> {
    let mut passphrase = String::new();

    for i in 0..num_of_words {
        let dice_round = get_dice_round()?;
        if i > 0 {
            passphrase.push('-');
        }

        let word = lookup_word(dice_round)
            .ok_or_else(|| anyhow::anyhow!("No word for roll: {dice_round}"))?;

        passphrase.push_str(word)
    }

    Ok(passphrase)
}

/// Importe la wordlist à partir d'un fichier
///
/// # Arguments
/// * `path` - Chemin vers le fichier de la wordlist
///
/// # Erreurs
/// Retourne une erreur si le fichier ne peut pas être ouvert ou une ligne est mal formatée
pub fn import_table(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut map: HashMap<u32, String> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        };
        let (key_str, value) = line
            .split_once('\t')
            .ok_or_else(|| format!("Invalid line format: '{line}'"))?;
        map.insert(key_str.parse()?, value.to_string());
    }

    WORDTABLE
        .set(map)
        .map_err(|_| "Table was already initialized".into())
}
