# iolite

iolite is a [Final Fantasy XIV](https://www.finalfantasyxiv.com/) exploration tool. This is mostly a re-write of [Sapphire](https://github.com/SapphireServer/Sapphire) but in [Rust](https://www.rust-lang.org/), with the main goal of just being able to enter the world and not much else. A "lite" version, you could say.

Iolite **is not** a private server, it never will be. This project is research project out of interest to see how production servers work and everything will not work. Iolite is not intended to emulate the game.

**Note**: iolite is most definitely research code and is nowhere near optimised. It doesn't make sense, looks ugly and things will break each version update!

## Requirements

- [rustlang](https://www.rust-lang.org/tools/install) >1.77.2
- a C++ compiler available for your platform that can be picked up by "cc", see https://docs.rs/cc/1.0.98/cc/#compile-time-requirements
- Oodle `oo2net_9_win64.dll` (optionally `oo2net_9_win64.lib`)
- A legit copy of the game

### Supported game version: 7.18h

## Configuration

Please edit the `GAME_PATH` in `start_game_script.rs` to where iolite can find the `ffxiv_dx11` exe file, while excluding the `.exe`.

## Building

```shell
cargo build
```

## Starting

```shell
cargo run
```

### Starting the game client with local server parameters

This is helpful for running the game to auto connect to your local server.

```shell
cargo run-script start
```

## What does it do currently?

Currently iolite allows you to
 - View the lobby with a mock character and select a mock character (can't create a character or anything else)
 - Log in to a world and fly around

To change the world, edit `ZONE_ID` in `src\world\handle_zone_packets.rs` and restart the server.

Planned features
 - GM Commands
 - Change current world with a command

Contributions are always welcome.

## Credits

See [Contributors](https://github.com/0xbbadbeef/iolite/graphs/contributors).

This project is not affiliated with FINAL FANTASY or SQUARE ENIX CO., LTD. in any way.
