// Roll n dice, optionally displaying each die face as ASCII art.

use rand::prelude::*;

// ASCII art rows for each standard six-sided die face.
// Index 0 corresponds to face 1, etc. Rows are printed top to bottom.
const DIE_ROWS: [[&str; 3]; 6] = [
    ["|       |", "|   o   |", "|       |"], // 1
    ["| o     |", "|       |", "|     o |"], // 2
    ["| o     |", "|   o   |", "|     o |"], // 3
    ["| o   o |", "|       |", "| o   o |"], // 4
    ["| o   o |", "|   o   |", "| o   o |"], // 5
    ["| o   o |", "| o   o |", "| o   o |"], // 6
];

// Renders one row (0..3) of the face of a die, supporting d1–d6.
// Non-six-sided dice fall back to showing the numeric value.
fn die_row(face: i32, row: usize) -> String {
    if (1..=6).contains(&face) {
        DIE_ROWS[(face - 1) as usize][row].to_string()
    } else if row == 1 {
        format!("| {:^5} |", face)
    } else {
        "|       |".to_string()
    }
}

// Prints a set of dice side by side, one line of art per console row.
fn print_dice(faces: &[i32]) {
    let top_bottom = "+-------+";

    println!();
    let line: Vec<String> = faces.iter().map(|_| top_bottom.to_string()).collect();
    println!("{}", line.join(" "));
    for row in 0..3 {
        let line: Vec<String> = faces
            .iter()
            .map(|&face| die_row(face, row))
            .collect();
        println!("{}", line.join(" "));
    }
    let line: Vec<String> = faces.iter().map(|_| top_bottom.to_string()).collect();
    println!("{}", line.join(" "));
    println!();
}

// Rolls a single die and records its face (shared by both public functions).
fn roll_one(sided_die: i32) -> i32 {
    rand::rng().random_range(1..=sided_die)
}

// Original behavior preserved: rolls n dice and returns the sum, silently.
#[allow(dead_code)]  //for games that don't show die faces.
pub fn roll_die(num_dice: i32, sided_die: i32) -> i32 {
    let mut sum = 0;
    for _ in 0..num_dice {
        sum += roll_one(sided_die);
    }
    sum
}

// Rolls n dice, displays each die face, and returns the sum.
// Use this anywhere you want the player to see what was thrown.
pub fn roll_and_display(num_dice: i32, sized_die: i32) -> i32 {
    let faces = roll_faces(num_dice, sized_die);
    print_dice(&faces);
    faces.iter().sum()
}

fn roll_faces(num_dice: i32, sided_die: i32) -> Vec<i32> {
    (0..num_dice).map(|_| rand::rng().random_range(1..=sided_die)).collect()
}