use std::cmp::Ordering;
use std::io;

fn main() {
    println!("== Le juste prix ==");

    let random_number = generate_random_number_between(1, 100);

    let mut found = false;

    while !found {
        println!("Quel est le juste prix ?");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Erreur: ligne non lue.");

        if let Ok(num) = guess.trim().parse::<u32>() {
            print!("Vous proposez : {num}");

            match num.cmp(&random_number) {
                Ordering::Less => println!(" -> C'est plus !"),
                Ordering::Greater => println!(" -> C'est moins !"),
                Ordering::Equal => {
                    println!(" -> Gagné !");
                    found = true;
                }
            }
        };
    };
}

/// Generates a random u32 number between min and max included.
fn generate_random_number_between(min: u32, max: u32) -> u32 {
    use rand::Rng;

    rand::thread_rng().gen_range(min..=max)
}