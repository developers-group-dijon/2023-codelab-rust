#[allow(unused_imports)]
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("== Le juste prix ==");

    todo!();

    // récupérer un nombre aléatoir enetre 1 & 100

    // créer un mutex pour sortir de la boucle de jeu

    // tant que le mutex est à false
    //      demander une entrée à l'utilisateur
    //      trimmer puis transformer l'entrée utilisateur en entier non signé (u32)
    //      comparer l'entrée utilisateur au nombre aléatoire
    //      donner le résultat à l'utilisateur
    //      si l'entrée est égale au nombre aléatoire, la boucle de jeu s'arrête
}

/// Generates a random u32 number between min and max included.
#[allow(dead_code)]
fn generate_random_number_between(min: u32, max: u32) -> u32 {
    use rand::Rng;

    rand::thread_rng().gen_range(min..=max)
}

/// lit l'entrée standard
#[allow(dead_code)]
fn get_input_from_user() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Erreur: ligne non lue.");

    input
}
