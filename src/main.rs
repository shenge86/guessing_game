use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Being bored at home, the computer decided to play a game against you.");
    println!("Human, guess the number between 1 and 100!");
    let secret_number = rand::thread_rng().gen_range(1,101);

    let mut i = 1;
    let imax = 5;
    while i <= imax {
        println!("Human, you have {} guesses left. Please input guess:", imax+1-i);

        let mut guess = String::new(); // mutable

        io::stdin().read_line(&mut guess) // &mut makes it a mutable reference
            .expect("Human, please try another input.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
            // .expect("Please type a number!");

        println!("Guess # {}", i);
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("The secret number is: {}", secret_number);
                println!("You win!");
                break;
            }
        }
        i+=1;
    }
    // println!("{}",i);

    if i > imax {
        println!("The secret number is: {}", secret_number);
        println!("After {} guesses, you still didn't get it? You lose!", imax)
    } 
}
