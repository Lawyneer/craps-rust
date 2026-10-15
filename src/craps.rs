
use crate::roll_die;
//use std::{thread, time::Duration};
use std::io;

pub fn craps(bet: i32) -> i32 {
    let point = roll_die::roll_and_display(2, 6);    

    match point {
        7 | 11 => {
            println!("Shooter rolled a {point} on the come-out and wins");
            bet
        }
        2 | 3 | 12 => {
            println!("Shooter rolled a {point} and lost");
            -bet
        }
        _ => {
            println!("The point is: {point}");           

            let mut roll = 0;
            while roll != point && roll != 7 {
                //thread::sleep(Duration::from_secs(2));
                // Pause the game so the play can see the results
                println!("Press Enter to keep rolling.");                
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                roll = roll_die::roll_and_display(2, 6);
                println!("The shooter rolled: {roll}");                                
            }
            if roll == point {
                println!("Shooter made the point and wins");
                bet
            } else {
                println!("Shooter crapped out");
                -bet
            }
        }
    }
}