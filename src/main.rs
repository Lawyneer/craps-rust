// A program to play a simple texted based game of craps

// Dependencies
use std::io;
mod roll_die;
mod craps;

// main
fn main() {

    // Initialize variables
    let mut bank_roll = 500;
    let keep_playing = true;
    let mut bet: i32 = 50;  // will update to allow player to set bet later
    

    while bank_roll > 0 && keep_playing == true {

        clearscreen::clear().expect("failed to clear screen");
        
        // use the absolute value of bet for when it is negative after a loss
        bet = craps::craps(bet.abs());

        bank_roll += bet;

        println!("The players bankroll is {bank_roll}");

        if bank_roll != 0 {
            println!("Press Enter to continue or ctrl + C to terminate the program ...");
        } else {
            println!("Player bust.  Game over.");        
        }

        // Pause the game so the play can see the results
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();      
        
    } // end while

} // end main