# Pong CLI
A CLI pong clone.  
Uses the [`crossterm`][crossterm] and [`specs`][specs] crates.

## Description
Play __pong__ from your terminal!  
Play against a ball-chasing AI opponent,  locally against another player,  
or let _two_ AIs fight it out!  
Everything is configurable and commented within the [`settings.ron`][settings.ron] file.

## Installation
### From crates.io
__TODO:__ See [issue #2][issue-settings], then publish to crates.io

### Binaries
Binaries for Linux and Windows are available from the [releases] GitHub page.  
From there, simply download the `.zip` file for your platform,  
extract the archive and run the executable:  
- `pong-cli` for Linux
  __TODO:__ Add `.desktop` file for Linux file manager GUIs
- `pong-cli.exe` for Windows

### From source
To build the game from source, you'll need to clone the repo and compile the game using `cargo`.  
You'll need to have `rust` version __1.38.0__ or higher installed.  

Clone the repository with ...
```
git clone https://github.com/Noah2610/pong-cli
```
`cd` into the project's root with `cd pong-cli`,  
then you can build and run the game with ...
```
cargo run --release
```

## Features
| Feature   | Default? | Description |
| :-------- | :------: | :---------- |
| `random`  | __YES__  | Enables randomization for ball spawning directions.<br />See `settings.ron` file for configuration. |
| `color`   | __YES__  | Adds configurable color to the game.<br />See `settings.ron` file for configuration. |
| `nightly` | no       | Enables `specs/nightly` feature for debug/development purposes. |

Use `--no-default-features` to disable all features when building with `cargo build/run`.

## License
[MIT License][mit]

[mit]:            https://github.com/Noah2610/pong-cli/blob/master/LICENSE
[crossterm]:      https://github.com/crossterm-rs/crossterm
[specs]:          https://github.com/amethyst/specs
[settings.ron]:   https://github.com/Noah2610/pong-cli/blob/master/settings.ron
[issue-settings]: https://github.com/Noah2610/pong-cli/issues/2
