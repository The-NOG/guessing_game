use std::io;
use rand::Rng;
use std::cmp::Ordering;

//set const for RNG low limit
const RNG_LOW : i32 = 1;
//set const for RNG high limit
const RNG_HIGH : i32 = 101;
//set max amount of tries
const TRIES:i32 = 5;
fn main() {
    //Small little intro. States the high and low possibilities
    println!("Welcome to the guessing game!\nToday's high is {RNG_HIGH} with a low of {RNG_LOW}\nYou have {TRIES} attempts to guess the number!");
    //generate the secret number
    let secret_number = rand::thread_rng().gen_range(RNG_LOW,RNG_HIGH);
    //init tries counter as 0
    let mut tries:i32 = 0;
    //begin the loop
    loop{
        let remaining : i32 = TRIES - tries;
        println!("Tries remainging: {remaining}");
        println!("Please input your guess.");
        //make empty var for storing guess
        let mut guess = String::new();
        //read the line for input
        io::stdin().read_line(&mut guess)
            .expect("Failed to readline");
        //before checking if the input is a number, check to see if it is "quit"
        if guess.trim() == "quit"{
            //if so, bye and die
            println!("See you next time!");
            break;
        }
        //parse the guess as i32
        let guess: i32 = match guess.trim().parse(){
            Ok(num) => num,
            //if parse fails just go through the loop again
            Err(_) => continue,
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too High"),
            Ordering::Equal => {
                println!("You win!\nYou found it in {tries} guesses!");
                break;
            }
        }
        tries += 1;
        if tries == TRIES{
            println!("You ran out of guesses!\nThe number was {secret_number}!");
            break;
        }
    }
}
