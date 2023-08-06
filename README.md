# RPG Maker 2 linux

Convert RPG Maker Games to linux Games with an extremly dificult process.

## How to

1. Download or [Create](#creating-the-configjson) `config.json` on the game folder.

2. Run the executable on game folder or pass the folder as a parameter `./rpg2linux [game_folder]`.

3. Answer the prompted questions.

4. Enjoy.

## Creating the `config.json`

In order to work, `rpg2linux` needs a json with the data necesary to make your game work on linux.

A normal `config.json` should look like this:

```json
{
  "file_asociations": [
    {
      "origin_file": "www/img/characters/mainCharacter.rpgmvp",
      "destination_files": ["www/img/characters/MainCharacter.rpgmvp"],
      "allows_symlink": false
    }
  ],
  "checked_nwjs_versions": [
    { "nwjs_version": "0.76.*", "especific_nwjs_commands": [""] },
    { "nwjs_version": "0.78.*", "especific_nwjs_commands": [""] }
  ],
  "pre_operation_commands": [""],
  "post_operation_commands": [""]
}
```

Here is an explanation

| Field                   | Description                                                                                                           |
| ----------------------- | --------------------------------------------------------------------------------------------------------------------- |
| pre_operation_commands  | this are commands that runs before anything                                                                           |
| file_asociations        | Windows is case insensitive, then sometimes is necesary to link the same file from lower_case to UPPER_CASE and so on |
| checked_nwjs_versions   | the nwjs version that is used to make your game playable on linux                                                     |
| post_operation_commands | finally commands that runs after the process finishes                                                                 |

## Compiling this tool

To compile this tool you need to:

1. Install [Rust](https://www.rust-lang.org/tools/install).

2. Copy this repository `git clone https://github.com/ThePixelCode/rpg2linux.git`

3. Run `cargo build --release`

4. Your executable is on `target/release/rpg2linux`
