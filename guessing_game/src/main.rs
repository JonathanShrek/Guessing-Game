use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    println!("Enter range length:");

    /*
     * Creates new mutable variable 'length' then uses the
     * 'io' module from the 'std' library to take user input.
     *
     * This allows the user to control how large of a range of
     * numbers the program will work with. The larger the range the
     * harder it is to guess the secret number.
     */
    let mut length = String::new();
    io::stdin().read_line(&mut length)
        .expect("Failed to read line");

    /*
     * Creates non-mutable variable 'length_int' that converts 
     * the user inputed string into 'u32' in order to
     * be used in 'gen_range'. This changes the range that will
     * be used in the program.
     */
    let length_int: u32 = length.trim().parse()
        .expect("empty value returned");

    /* 
     * Creates non-mutable variable 'secret_number'. We use the 'rand'
     * library to pick a random number from the range we established
     * above.
     */ 
    let secret_number = rand::thread_rng().gen_range(1,length_int + 1);

    /* 
     * Loop to keep iterating through user guesses until the user enters 
     * the correct number.
     */
    loop {
        println!("Please input your guess");

        // Creates new mutable variable 'guess'
        let mut guess = String::new();

        // Takes input for the user's guess and saves this in the 'guess' variable
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Match to allow program to continue if guess is not an int
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        /*
         * Match that compares guess with secret number and
         * output if the guess was too small, too big, or equal.
         * If guess is equal to the secret number the loop breaks and
         * the game is over.
         */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

