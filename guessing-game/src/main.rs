use std::io;
use std::cmp::Ordering;
use rand::Rng; 

fn main() {
    println!("Guess the number!");

    //Generates a random number between 1 and 101
    let secret_number = rand::thread_rng().gen_range(1, 101);

    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        //Read user input and prevent error with expect
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        //Converting string to unsigned 32-bit number so we can compare with the generated one since rust cannot compare string and number
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");

        //Handling error if user inputs an invalid character
        //If parse is able to successfully turn the string into a number, it will return an Ok value that contains the resulting number.
        //If parse is not able to turn the string into a number, it will return an Err value that contains more information about the error.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {}", guess);
        
        //Compare user imput with Ordering enumerators
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                //Exit the loop if the user input is the same as the random number
                break;
            }
        }
    }

}
