use std::io;                              // library-ability to accept user imput
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    println!("Guess a number");
    let secret_number = rand::thread_rng().gen_range(1, 101);               //Generating a random number
    println!("The secret number is: {}", secret_number); 
    println!("Please input your guess.");

    let mut guess = String::new();        // created a mutable var i.e bound to a new empty instance of a str.

    io::stdin()                           // stdin of io module is a type that represemts a handles to std input.  
    .read_line(&mut guess)                // read_line takes whatever the user input     
    .expect("Failed to read line");       // .expect is to hsndle err

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You Win!"),
    }
}