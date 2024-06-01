# iolite

iolite is a [Final Fantasy XIV](https://www.finalfantasyxiv.com/) exploration tool. This is mostly a re-write of [Sapphire](https://github.com/SapphireServer/Sapphire) but in [Rust](https://www.rust-lang.org/), with the main goal of just being able to enter the world and not much else. A "lite" version, you could say.

Iolite **is not** a private server, it never will be. This project is research project out of interest to see how production servers work and everything will not work. Iolite is not intended to emulate the game.

## Requirements

- [rustlang](https://www.rust-lang.org/tools/install) >1.77.2
- a C++ compiler available for your platform that can be picked up by "cc", see https://docs.rs/cc/1.0.98/cc/#compile-time-requirements
- A legit copy of the game

### Supported game version: 6.58

## Configuration

The configuration can be found in `/.cargo/config.toml`.

Please edit the `GAME_PATH` to where iolite can find the `ffxiv_dx11` exe file, while excluding the `.exe`.

## Building

```shell
cargo build
```

## Starting

```shell
cargo run
```

If your `GAME_PATH` is configured correctly the game will open when the iolite server has started (this is planned to be a separate script soon).

## What does it do currently?

Currently Iolite only allows you to log into the lobby and see a mock character.

Things that are currently being worked on:

- Ability to enter the world and walk around
- GM commands

Contributions are always welcome.

## Credits

See [Contributors](https://github.com/0xbbadbeef/iolite/graphs/contributors).

This project is not affiliated with FINAL FANTASY or SQUARE ENIX CO., LTD. in any way.
