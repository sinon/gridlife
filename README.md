# Game of Life

[![codecov](https://codecov.io/github/sinon/game-of-life/graph/badge.svg?token=5Y7PP6AC3K)](https://codecov.io/github/sinon/game-of-life)

Implementation of [Conway's Game of Life].

Generate 2d `Grid` of given size that are empty or contain a random distribution of dead or alive populations.

`update_states` is then called on the `Grid` to generate the next grid state based on the rules of [Conway's Game of Life].

Package also contains an example Text User Interface (TUI) leveraging `gridlife` with `ratatui`, which can be used to run random simulations.

## Run TUI

`cargo run --features="build-binary"`

<img width="1200" alt="image" src="https://github.com/user-attachments/assets/63ff7fc7-5d7f-447a-a9de-496dbe611fcd" />

<!--Links -->
[Conway's Game of Life]: https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life
