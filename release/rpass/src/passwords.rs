use anyhow::{bail, Result};
use passwords::PasswordGenerator;
use thiserror::Error;
use zxcvbn::zxcvbn;

#[derive(Debug, Error)]
pub enum PasswordGenerationError {
    #[error("Length must be at least 8.")]
    LengthTooLow,
    #[error("Password generation error: {0}")]
    PasswordGenerationError(String),
}

pub fn generate(len: usize) -> Result<String> {
    if len < 8 {
        bail!(PasswordGenerationError::LengthTooLow);
    }

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

    let generated = pg.generate_one();

    if let Err(error) = generated {
        bail!(PasswordGenerationError::PasswordGenerationError(
            error.into()
        ))
    }

    Ok(generated.unwrap())
}

pub fn format_password_strength(password: &str) -> Result<String> {
    let estimate = get_password_strength(password)?;

    Ok(match estimate {
        0 => "🚮 0/4 - You must change it !".to_string(),
        1 => "❌ 1/4 - Nowhere near safe !".to_string(),
        2 => "⚠️ 2/4 - Not safe !".to_string(),
        3 => "✅ 3/4 - Safe".to_string(),
        4 => "🔥 4/4 - Ultra safe".to_string(),
        _ => bail!("Error while parsing password score !"),
    })
}

pub fn get_password_strength(password: &str) -> Result<u8> {
    let estimate = zxcvbn(password, &[])?;

    Ok(estimate.score())
}