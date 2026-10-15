# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2026-10-15

### Added
- ASCII-art die face rendering showing both dice side-by-side after each roll
- Interactive pause between rolls that waits for the user to press Enter
  (replacing the fixed 1-second delay)
- `roll_and_display()` function in `roll_die.rs` for reusable dice visualization
  across potential future dice games
- Fallback rendering for non-standard dice (d4, d8, d10, d20, etc.) that shows
  the numeric value inside the die box

### Changed
- Come-out roll now also displays die faces (previously only point-phase rolls
  showed dice)
- Removed `sleep(Duration::...)` delay from point phase in favor of
  user-controlled pause

### Fixed
- Come-out roll of 11 now correctly wins. Previously only 7 was treated as a
  natural, so rolling 11 incorrectly started a point round.

## [0.1.0] - 2026-10-10

### Added
- Initial release: text-based Craps game with come-out roll, point rounds,
  and payout logic
- Fixed $500 starting bankroll with $50 wager per round
- Configurable dice rolling (n dice, n sides)

[Unreleased]: https://github.com/Lawyneer/craps-rust/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/Lawyneer/craps-rust/releases/tag/v1.0.0
[0.1.0]: https://github.com/Lawyneer/craps-rust/releases/tag/v0.1.0