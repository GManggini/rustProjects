use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {    
    println!("Guess the number!");
    println!("Please input your guess");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    loop {

        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Please input a valid guess");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => { 
                println!("please input just N U M B E R S");
                continue;
            }
        };
        
        println!("You guessed: {guess}");  


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => { 
                println!("You Win!");
                break();
            }
        }
    }    
}

//feel free to chat, you will be welcome!
//bienvenido al chat, quedate tranquilo para hablar!
//quem vem chegando fica a vonts!
