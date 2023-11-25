use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Zgadnij liczbę!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Wpisz liczbę.");

        let mut guess= String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Nie udało się oczytać lini");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("Zgadujesz: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Za mało!"),
            Ordering::Greater => println!("Za dużo!"),
            Ordering::Equal => {
                println!("Wygrałeś!");
                break;
            }
        }
    }
}
