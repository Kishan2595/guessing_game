use std::io;                                                                    // library-ability to accept user imput
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    println!("Guess a number");
    let secret_number = rand::thread_rng().gen_range(1, 101);                   // `GENERATING A NEW NUMBER`
    println!("The secret number is: {}", secret_number);
    loop {                                                                      // `CREATING A LOOP SO USER CAN GUESS THE NUMBER MULTIPLE TIMES`
        println!("Please input your guess.");

        let mut guess = String::new();                                          // created a mutable var i.e bound to a new empty instance of a str.

        io::stdin()                                                             // stdin of io module is a type that represemts a handles to std input.  
        .read_line(&mut guess)                                                  // read_line takes whatever the user input     
        .expect("Failed to read line");                                         // .expect is to hsndle err

        let guess: u32 = guess.trim().parse().expect("Please type a number!");  // guess redeclared using shadow method     // trim is to remove the whitespace at  
                                                                                // beginning and end of the int  // parse is to convert a string into int(u32)       
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {                                       // match is made up of arms(consisting patterns) 
            Ordering::Less => println!("Too small"),                            // as the pattern satisfies the code belonging to it runs
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}