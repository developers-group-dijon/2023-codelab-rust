#![allow(unused)]
use crate::{
    cli::{Cli, Command},
    console_utils::ConsoleIO,
    data_store::{DataStore, PasswordStore, Unlocked},
    passwords,
};
use anyhow::{bail, Result};
use chrono::{DateTime, Local, Utc};
use inquire::{required, PasswordDisplayMode};
use itertools::Itertools;
use thiserror::Error;

/// Possible errors upon handling passwords and datastore.
#[derive(Debug, Error)]
pub enum HandlingError {
    #[error("Datastore already initialized")]
    AlreadyInitialized,
    #[error("Datastore must be initialized before using. Use rpass init.")]
    NotInitialized,
    #[error("Datastore destroy aborted")]
    DestroyAborted,
    #[error("Password addition aborted")]
    AdditionAborted,
    #[error("Password deletion aborted")]
    DeleteAborted,
    #[error("Key \"{0}\" already exists in datastore")]
    KeyAlreadyExists(String),
    #[error("Key \"{0}\" is not in datastore")]
    KeyNotFound(String),
}

/// Entrypoint of the middleware.
///
/// Dispatch given CLI arguments to dedicated functions.
///
/// This will transmit any error in middleware to the `main()` function.
pub fn handle(cli: &Cli) -> Result<()> {
    todo!();

    // valider que le master password est bien dans les paramètres de la commande.
    // Attention, c'est une option, donc soit Ok(value) soit None (il y a des fonction is_...)
    //
    // Si ce n'est pas le cas, utiliser la fonction require_master_password().
    // Cette fonction renvoie un Result, mais on va transmettre ces erreurs à main ;)

    // construire une instance de DataStore (regarder les méthodes disponibles de DataStore)

    // Avant d'utiliser le DataStore, il faudrait valider qu'il est initialisé : regarder les méthodes
    // disponibles sur DataStore ;)
    //
    // Si le datastore n'est pas initialisé, et que l'argument commande de cli n'est pas de type Commande::Init
    // il faut renvoyer une erreur HandlingError::NotInitialized.
    //
    // Vous pouvez regarder si une valeur est du type d'un enum avec la macro matches!(val, type).

    // Utilisez la structure match pour couvrir tout les cas de cli.command, et appelez les bonnes fonctions
    // de ce module en conséquence (ex: Command::List => list(...)).
    //
    // Chacune de ces fonctions renvoie une instance du DataStore en état Unlocked, récupérer cette instance.

    // reverrouiller le datastore

    // retour d'un Result vide après exécution du code métier.
    Ok(())
}

/// Initializes the DataStore on the system.
///
/// This will return an error if :
/// - the initialization fails.
/// - the datastore cannot be unlocked.
fn init(data_store: DataStore, master_password: &str) -> Result<DataStore<Unlocked>> {
    todo!();
    // comme pour handle, nous ne souhaitons pas traiter les erreurs ici, il
    // faudrait les remonter dans main.

    // Si le DataStore est déjà initialisé, jeter une erreur HandlingError::AlreadyInitialized

    // Initialiser le DataStore.

    // Pour valider que cela fonctionne, déverrouiller le DataStore.

    // Imprimer un succès sur la sortie standard.
    // Vous pouvez vous aider de la struct console_utils::ConsoleIO.

    // retourner le DataStore unlocked.
}

/// Adds a password to the DataStore.
///
/// This will return an error if :
/// - the datastore cannot be unlocked.
/// - the password label cannot be read.
/// - the password label already exists in the DataStore.
/// - the password cannot be read.
/// - the password strength cannot be calculated.
/// - the "unsafe password addition" is not confirmed.
/// - the password data cannot be recorded in the DataStore.
fn add(data_store: DataStore, master_password: &str) -> Result<DataStore<Unlocked>> {
    ////////////////// PARTIE 1 //////////////////

    // créer une instance de ConsoleIO, ça va être utile :)

    // Déverrouiller le DataStore

    // Demander le label pour le mot de passe (son identifiant unique)
    // Ce label doit être entré obligatoirement.

    // Valider que le label n'existe pas déjà dans le DataStore
    // Si c'est le cas, renvoyer une erreur HandlingError::KeyAlreadyExists

    // Demander en suite l'URL et et login associé au mot de passe.
    // Ces données doivent être une chaîne vide par défaut.
    //
    // La fonction ask_question_default de ConsoleIO peut vous aider :)

    // Enfin, demander le mot de passe en lui-même
    // Il doit obligatoirement être saisi, et des caractères doivent
    // s'afficher à la saisie.

    //////////////////////////////////////////////

    ////////////////// PARTIE 2 //////////////////

    // récupérer une représentation de la force du mot de passe saisie avec
    // l'une des fonctions du module passwords.
    //
    // vous pouvez imprimer à l'aide de ConsoleIO et de la macro format!()
    // la force du mot de passe.

    // récupérer en suite la force directe du mot de passe (sa valeur).
    //
    // si la force du mot de passe est inférieure à 3
    // demander une confirmation à l'utilisateur si il veut utiliser
    // un mot de passe faible à l'aide de ConsoleIO
    //
    // Si il décline, renvoyez une erreur HandlingError::AdditionAborted

    //////////////////////////////////////////////

    ////////////////// PARTIE 3 //////////////////

    // Demander le commentaire lié au mot de passe (optionnel)

    // Créer une instance de PasswordStore avec toutes les données nécessaires.
    //
    // Vous pouvez vérifier qu'une chaine est vide avec .is_empty()
    //
    // Vous pouvez attribuer soit None, soit Some(value: T) à une Option<T>
    //
    // Pour créer une date en TZ UTC, vous pouvez utiliser Utc::now()

    // insérer la donnée dans le DataStore ouvert.

    // Signaler à l'utilisateur le succès de l'oprétation

    // retourner le DataStore unlocked.

    //////////////////////////////////////////////

    todo!();
}

/// Lists all the stored password in the DataStore.
///
/// This will return an error if the DataStore cannot be unlocked.
fn list(data_store: DataStore, master_password: &str) -> Result<DataStore<Unlocked>> {
    // créer une instance de ConsoleIO, ça va être utile :)

    // Déverrouiller le DataStore

    // Voici les headers du listing des mots de passe
    let headers: Vec<String> = vec![
        "Label".to_string(),
        "Url".to_string(),
        "Login".to_string(),
        "Comment".to_string(),
        "Creation date".to_string(),
        "Password strength".to_string(),
    ];

    // ...et le conteneur des lignes du listing.
    // C'est un tableau de tableau de chaînes
    let mut lines: Vec<Vec<String>> = vec![];

    // Parcourir les données du DataStore dans une boucle for ... in ...
    //
    //  - Dans cette boucle, utiliser la fonction sanitize_none_option_string()
    //    pour être sûr d'avoir une chaîne depuis une Option<String>
    //
    //  - Utiliser DateTime::from() pour transformer les dates en chaîne.
    //
    //  - Pousser, dans le tableau lines, un tableau avec toutes les valeurs nécessaires.

    // Utilisez la fonction string_table() de ConsoleIO pour afficher votre liste.

    // retourner le DataStore unlocked.

    todo!();
}

/// Deletes password data from the DataStore given a password label.
///
/// This will return an error if :
/// - the datastore cannot be unlocked.
/// - the label of the password to delete is not found in the DataStore.
/// - the deletion is not confirmed.
/// - the actual data deletion cannot be made.
fn delete(
    data_store: DataStore,
    label: &str,
    master_password: &str,
) -> Result<DataStore<Unlocked>> {
    todo!();

    // créer une instance de ConsoleIO, ça va être utile :)

    // Déverrouiller le DataStore

    // vérifier que le mot de passe donné par label existe bien dans le DataStore
    // si il n'existe pas, renvoyer une erreur HandlingError::KeyNotFound

    // Demander une confirmation à l'utilisateur si il veut vraiment effacer son mot de passe

    // Si ce n'est pas confirmé, sortir du programme avec l'erreur HandlingError::DeleteAborted

    // effacer l'entrée dans le DataStore

    // Signaler à l'utilisateur le succès de l'oprétation

    // retourner le DataStore unlocked.
}

/// Prints a password to the standard output given its label.
///
/// ⚠️ This should only be used in command chains. ⚠️
///
/// This will return an error if :
/// - the DataStore cannot be unlocked.
/// - the password label is not found in the DataStore.
fn dump(data_store: DataStore, label: &str, master_password: &str) -> Result<DataStore<Unlocked>> {
    todo!();

    // créer une instance de ConsoleIO, ça va être utile :)

    // Déverrouiller le DataStore

    // Si le mot de passe existe dans le DataStore, l'imprimer sur la sortie standard
    // Sinon, renvoyer une erreur HandlingError::KeyNotFound

    // retourner le DataStore unlocked.
}

/// Generates a strong random password of 24 chars.
///
/// This will return an error if :
/// - the DataStore cannot be unlocked.
/// - the password cannot be generated.
fn generate(data_store: DataStore, master_password: &str) -> Result<DataStore<Unlocked>> {
    todo!();
    // créer une instance de ConsoleIO, ça va être utile :)

    // Déverrouiller le DataStore

    // A l'aide d'une fonction créé précédement, générer un mot de passe
    // de 24 caractère.

    // Imprimer le mot de passe sur la sortie standard

    // retourner le DataStore unlocked.
}

///////////////////// UTILITY FUNCTIONS /////////////////////

/// Requests the master password to unlock a DataStore.
///
/// This will return an error if the password cannot be read.
fn require_master_password() -> Result<String> {
    let console_utils = ConsoleIO::new();

    let password_question = console_utils.input_password("Enter master password:");

    let master_password = password_question
        .without_confirmation()
        .with_display_mode(PasswordDisplayMode::Masked)
        .prompt()?;

    Ok(master_password)
}

/// Transforms an `Option<String>` into a `String` by filling a default empty string
/// in the value in place of a `None`.
fn sanitize_none_option_string(opt: Option<String>) -> String {
    if let Some(value) = opt {
        value
    } else {
        "".into()
    }
}
