# Rusty Moves

A simple Rust server which plays various games with multiple clients simultaneously through UDP.

## Features

- Tic-Tac-Toe random move player (extensible to nxn board, m as win condition)

## To-Do

- [x] Fix Stockfish integration for chess (see feature/chess branch)
- [ ] Full chess integration (WIP, see feature/chess branch)
  - [x] Random top Stockfish move (see feature/chess branch)
  - [ ] Re-design interaction protocal to fully support chess features
- [ ] Finish some ideas for a smarter Tic-Tac-Toe AI
- [ ] Client and server code cleanup - reuse components
- [ ] Re-design protocols over UDP
  - [ ] Fix lost packet issues
  - [ ] Distinguish between chess/tic-tac-toe messages
    - [ ] Remove overly-verbose enums used for the two games, players (and "GameAndPlayer")

