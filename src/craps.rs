
use crate::roll_die;
use std::{thread, time::Duration};

pub fn craps(bet: i32) -> i32 {
    let point = roll_die::roll_die(2, 6);
    println!("The point is: {point}");

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
            let mut roll = 0;
            while roll != point && roll != 7 {
                roll = roll_die::roll_die(2, 6);
                println!("The shooter rolled: {roll}");
                thread::sleep(Duration::from_secs(1));
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