# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- 1-second pause between rolls during point rounds so the user can see the
  progression of the game

### Fixed
- Come-out roll of 11 now correctly wins. Previously only 7 was treated as a
  natural, so rolling 11 incorrectly started a point round.

## [0.1.0] - 2026-10-10

### Added
- Initial release: text-based Craps game with come-out roll, point rounds,
  and payout logic
- Fixed $500 starting bankroll with $50 wager per round
- Configurable dice rolling (n dice, n sides)

[Unreleased]: https://github.com/Lawyneer/craps-rust/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Lawyneer/craps-rust/releases/tag/v0.1.0