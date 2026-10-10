# Craps — A Text-Based Dice Game in Rust

A terminal-based version of the casino dice game Craps written in Rust. This project is a port of my existing [Python text-based Craps game](https://github.com/Lawyneer/Craps).  I'm using it as a hands-on/practical exercise for learning Rust.

## About the Game

For those that don't know, Craps is a dice game played with two six-sided dice. The basic flow implemented is:

1. **Come-out roll**:  The shooter rolls two dice:
   - Rolling a **7** wins.
   - Rolling a **2, 3, or 12** loses.
   - Any other total becomes the **point**.
2. **Point phase**:  The shooter keeps rolling:
   - Rolling the **point** before rolling a 7 wins.
   - Rolling a **7** before the point loses ("sevening out").

The player starts with a $500 bankroll and wagers $50 per round. Winnings are added to the bankroll and losses are subtracted. The game continues until the player busts or just cancels the program.

## Features

- Full come-out roll and point-round game logic
- Bankroll tracking with automatic game-over on bust
- Screen clearing between rounds for clean terminal display
- Pause between rounds so the player can review results

## Getting Started

### Prerequisites

- [Rust](https://github.com/Lawyneer/craps-python) (stable toolchain, includes Cargo)

### Building and Running

```bash
git clone https://github.com/Lawyneer/craps-rust.git
cd craps
cargo run
```

## Project Structure

```
src/
├── main.rs      # Game loop, bankroll management, user interaction
├── craps.rs     # Craps game logic (come-out roll, point rounds, payouts)
└── roll_die.rs  # Dice-rolling utility (n dice, n sides, returns sum)
```

## Roadmap / Future Developments

- [ ] Player-selected bet amounts
- [ ] Pass Line / Don't Pass Line betting
- [ ] Taking and laying odds

## Learning Rust Through Porting

This program is a Rust rewrite of my [original Python version](https://github.com/Lawyneer/Craps).  I am hoping porting a familiar program to a new language will help me learn its idioms.  In particular, Rust introduces concepts like ownership, borrowing, explicit mutability, and module visibility that differ significantly from Python.

## License

This project is licensed under the [MIT License](LICENSE).
