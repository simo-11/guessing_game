use std::io::{self,Write};
use std::{thread, cmp::Ordering, time};
use rand::Rng;

fn main() {
    println!("Guess the number v1.0.4!");

    let secret_number= rand::thread_rng().gen_range(1, 101);
    let mut attempt =0;
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        attempt+=1;
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        let diff: i32=secret_number-guess;
        println!("Your guess #{} was {}", 
            attempt, guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => print!("Too small, add {} to get correct one# ",diff),
            Ordering::Greater => print!("Too big, subtract {} to get correct one# ",-diff),
            Ordering::Equal => {
                let dur=time::Duration::from_secs(5);
                println!("You win, this window will close in {} ms", dur.as_millis());
                for to_end in 0..100  {
                    thread::sleep(dur/100);
                    print!("{}\r",dur.as_millis()-to_end*dur.as_millis()/100);
                    io::stdout().flush().unwrap();
                }
                break;
            }
        }
    }
}
