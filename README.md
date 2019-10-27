# Pong CLI
A CLI pong clone.  
Uses the [`crossterm`][crossterm] and [`specs`][specs] crates.

## Asciicasts
<p align="center">
  <a href="https://asciinema.org/a/277218" target="_blank">
    <img
     title="player vs AI"
     src="https://asciinema.org/a/277218.svg"
     width="300px" />
  </a>
  <a href="https://asciinema.org/a/277219" target="_blank">
    <img
    title="player vs AI with 3 balls"
    src="https://asciinema.org/a/277219.svg"
    width="300px" />
  </a>
</p>

## Description
Play __pong__ from your terminal!  
Play against a ball-chasing AI opponent,  locally against another player,  
or let _two_ AIs fight it out!  
Everything is configurable, see the [Configuration][readme-configuration] section.

## Installation
### From [crates.io][crates.io]
You'll need to have `cargo` and `rust` version __1.38.0__ or higher installed.  
Download, build, and install from [crates.io][crates.io] with ...
```
cargo install pong-cli
```
Then you should be able to run the game from anywhere with ...
```
pong-cli
```

### Binaries
Binaries for Linux and Windows are available from the [releases] GitHub page.  
From there, simply download the `.zip` file for your platform,  
extract the archive and run the executable:  
- `pong-cli` for Linux (from terminal),  
  or double-click the `PongCLI.desktop` file from your GUI file-manager
- `pong-cli.exe` for Windows

### From source
To build the game from source, you'll need to clone the repo and compile the game using `cargo`.  
You'll need to have `rust` version __1.38.0__ or higher installed.  

Clone the repository with ...
```
git clone https://github.com/Noah2610/pong-cli
```
`cd` into the project's root with `cd pong-cli`,  
Then you can build and run the game with ...
```
cargo run --release
```
You can also install it with `cargo`, so it is available everywhere from the command-line ...
```
cargo install --path .
```

## Configuration
Configurations are read from a `ron` file at one of the following locations (in order):
- `./settings.ron`
- `./pong-cli.ron`
- `$HOME/.pong-cli.ron`
- `$HOME/.config/pong-cli/settings.ron` (for Linux),  
  `$HOME/AppData/Roaming/pong-cli/settings.ron` (for Windows)  
  `$HOME/Library/Preferences/pong-cli/settings.ron` (for MacOS)

To start, copy the default [`settings.ron`][settings.ron] file to one of these locations.  
On Linux, you can enter the following (if you have `curl` installed), to  
automatically create the proper directory/directories and download the latest `settings.ron` file:
```
mkdir -p ~/.config/pong-cli
curl https://raw.githubusercontent.com/Noah2610/pong-cli/master/settings.ron > ~/.config/pong-cli/settings.ron
```

Once the `settings.ron` file is in place, you can edit it as you wish.  
The file itself is commented, so just open it to see all available settings.

## Features
| Feature   | Default? | Description |
| :-------- | :------: | :---------- |
| `random`  | __YES__  | Enables randomization for ball spawning directions.<br />See `settings.ron` file for configuration. |
| `style`   | __YES__  | Adds configurable color and text styling (bold, underline, etc.) to the game.<br />See `settings.ron` file for configuration. |
| `nightly` | no       | Enables `specs/nightly` feature for debug/development purposes. |

Use `--no-default-features` to disable all features when building with `cargo build/run`.

## License
[MIT License][mit]

[mit]:                  https://github.com/Noah2610/pong-cli/blob/master/LICENSE
[crossterm]:            https://github.com/crossterm-rs/crossterm
[specs]:                https://github.com/amethyst/specs
[settings.ron]:         https://github.com/Noah2610/pong-cli/blob/master/settings.ron
[issue-settings]:       https://github.com/Noah2610/pong-cli/issues/2
[releases]:             https://github.com/Noah2610/pong-cli/releases
[readme-configuration]: #configuration
[crates.io]:            https://crates.io/crates/pong-cli
