#![allow(unused)]

use anyhow::{bail, Result};
use passwords::PasswordGenerator;
use thiserror::Error;
use zxcvbn::zxcvbn;

/// possible errors upon password generation.
#[derive(Debug, Error)]
pub enum PasswordGenerationError {
    #[error("Length must be at least 8.")]
    LengthTooLow,
    #[error("Password generation error: {0}")]
    PasswordGenerationError(String),
}

/// Generates a safe password with the given length.
///
/// the return will be a `PasswordGenerationError::LengthTooLow`
/// error if the length requested is under 8.
pub fn generate(len: usize) -> Result<String> {
    // valider que la length est supérieur à 8
    // ou renvoyer une erreur PasswordGenerationError::LengthTooLow
    // en utilisant bail!().

    // créer un générateur de mot de passe
    let pg = PasswordGenerator {
        length: len,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: true,
        spaces: false,
        exclude_similar_characters: false,
        strict: true,
    };

    todo!();

    // générer un mot de passe à l'aide de la struct
    // PasswordGenerator et de sa fonction generate_one.

    // traiter le cas où la génération aurait échoué
    // si c'est le cas, renvoyer une PasswordGenerationError::PasswordGenerationError
    // avec le texte de l'erreur dedans.
    //
    // /!\ generate_one() renvoie un Result<String, &str>
    // ce qui veut dire que l'erreur est une &str que vous pouvez passer
    // en String avec .into()

    // retourner le cas de succès avec le mot de passe généré dedans.
    // Ok(...)
    //
    // les retours de fonction en rust se font soit à l'aide du mot clé return,
    // soit en laissant la dernière ligne d'un bloc sans ;
}

/// Outputs a String representing a password's strength (measured by ZXCVBN).
///
/// This will return an error if the generated score is invalid.
pub fn format_password_strength(password: &str) -> Result<String> {
    let estimate = get_password_strength(password)?;

    // Les scores ZXCVBN sont de 0 à 4. En dessous de 3 le mot de passe
    // est considéré comme trop faible.
    //
    // utilisez la syntaxe match pour stocker, pour tout les cas de 0 à 4
    // un message différent en fonction de la force du mot de passe.
    //
    // vous devrez traiter un cas par défaut si le score est erroné (> 4) ;
    // on ne voudrait pas traiter tout les chiffres de 0 à 63.
    //
    // vous pouvez assigner le résultat d'une expression match à une variable ;)

    // renvoyer le résultat de succès avec le message à l'intérieur.
}

/// Generates a strengh score for a given password against ZXCVBN.
///
/// This will return an error in any case of issue with ZXCVBN.
pub fn get_password_strength(password: &str) -> Result<u8> {
    let estimate = zxcvbn(password, &[])?;

    Ok(estimate.score())
}
