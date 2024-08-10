# tictactoe

TicTacToe TUI implementation in Rust 🦀

![image](https://github.com/user-attachments/assets/09a821d1-a0d9-4f5c-8d3e-cc62dce1cb13)

## Table of Contents
- [Installation](#installation)
- [Usage](#usage)
- [Detailed description](#detailed-description)
- [Links](#links)

## Installation

You have to compile it yourself, but that shouldn't be a problem. Only thing
you need is `cargo`. You need to go to the `tictactoe` project folder and run:

```
cargo build -r
```

After it's done compiling, you can start it in `./target/release/tictactoe`.

## Usage

You can start `tictactoe` game like this (automatically fills screen):

```
./tictactoe
```

If you want to set specific size and win length, you can do it like this:

```
./tictactoe -s <width> <height> -w <win_length>
```

All the usage and flags can be seen in the help:

```
./tictactoe -h
```

## Detailed description

When you start the game, you immediately see the board. Under the board,
there's current game state. It displays who's turn it is, who won or whether
game is a draw. There's one cell, which is selected (has bold border). You can
change selected cell using `Arrow` keys and using `hjkl` keys. Player on turn
can place its symbol by pressing `Enter`. The symbol will appear in the
selected cell.

When any player reaches set win length (by default 5), the winning sequence
gets crossed out. The game then can be restarted by pressing `r` key.

![image](https://github.com/user-attachments/assets/09a821d1-a0d9-4f5c-8d3e-cc62dce1cb13)

## Links

- **Author:** [Martan03](https://github.com/Martan03)
- **GitHub repository:** [tictactoe](https://github.com/Martan03/tictactoe)
- **Author website:** [martan03.github.io](https://martan03.github.io)
