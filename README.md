# Pindow
**[About](#About)**

**[Installation](#Installation)**

**[Config](#Config)**

**[Contributing](#Contributing)**

# About
Pindow allows you to have a list of "pinned" apps, open each one with one keybind `{num} + keybind`.
> Currently this is only available on windows
## Preview
https://user-images.githubusercontent.com/76080854/206830997-5ecc5d55-8219-4fff-8c81-84c90ac9c71b.mp4


# Installation
[Download the binary here](https://github.com/YummyOreo/Pindow/releases) or [build it](#Building). Then put it somewere.

> You can put it in your path or have it start on start up. Whatever suits you!

# Config
Example:
```json
{
    "configs": [
        {
            "name": "Work",
            "apps": [
                {
                    "app_path": "alacritty",
                    "args": [
                        "--working-directory", "D:/Desktop 2", "--hold"
                    ]
                }
            ],
            "timeout": 10,
            "keybindings": {
                "debug_close": ["Key1", "RControl"]
            }
        },
        {
            "name": "Nothing"
        }
    ]
}
```
Your config is located in `App Data/Roaming/Pindow/config.json`.

### Fields
The fields of the config are listed [here](FIELDS.md)

### Arguments/Flags
    `-d` or `--debug`: Starts the app in debug mode.
    `-c {num}` or `--config {num}`: The config that the app starts in. {num} is the index.
    `-p {path} or --path {path}`: Load a custom config path. If it can't find the path, it will try to create it in the path.
    `--get-path `: Prints the path of the config
    `-h or --help `: Prints the help menue

### Key
The keys are listed [here](KEYS.md).

# Contributing
Just open a issue or pr to a existing issue and i'll see if I want it.
To run you should do: `cargo run -- -d`.

There are no dependencies that will not be installed by cargo!
> Bar windows requirement as it uses the windows api

## Building
Run `cargo build --release`!

# ToDo
#### v1.0.0
- [x] Help menu with `--help` (Will not start the app)
- [x] Get path of config with flags (Will not start the app)
- [x] Load different configs files with flags
- [ ] Keybind to pin apps
#### v1.1.0
- [ ] Add custom keybinds to a specific apps
- [ ] GUI to view the current list/config
