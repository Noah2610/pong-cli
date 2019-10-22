# Pong CLI
A CLI pong clone.  
Uses the [`crossterm`][crossterm] and [`specs`][specs] crates.

## Description
Play __pong__ from your terminal!  
Play against a ball-chasing AI opponent,  locally against another player,  
or let _two_ AIs fight it out!  
Everything is configurable and commented within the [`settings.ron`][settings.ron] file.

## Installation
__TODO:__ See [issue #2][issue-settings], then publish to crates.io

For now you'll need to clone the whole repo and build the crate yourself.  
You'll need to have `rust` version __1.38.0__ or higher installed.  

Clone the repository with ...
```
git clone https://github.com/Noah2610/pong-cli
```
`cd` into the project's root with `cd pong-cli`,  
then you can build and run the game with ...
```
cargo run
```

## License
[MIT License][mit]

[mit]:            https://github.com/Noah2610/pong-cli/blob/master/LICENSE
[crossterm]:      https://github.com/crossterm-rs/crossterm
[specs]:          https://github.com/amethyst/specs
[settings.ron]:   https://github.com/Noah2610/pong-cli/blob/master/settings.ron
[issue-settings]: https://github.com/Noah2610/pong-cli/issues/2
