// Function to roll n number of dice and return the sum.

// pa
use rand::prelude::*;

pub fn roll_die(num_dice: i32, sided_die: i32) -> i32 {
    let mut die = 1;
    let mut dice_sum = 0;

    while die <= num_dice {

        dice_sum = dice_sum + rand::rng().random_range(1..=sided_die);

        die = die + 1;
    }
    
    return dice_sum
}
