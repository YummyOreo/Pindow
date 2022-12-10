# Pindow
**[About](#About)**

**[Installation](#Installation)**

**[Config](#Config)**

**[Config](#Contributing)**

# About
Pindow allows you to have a list of "pinned" apps, open each one with one keybind `{num} + keybind`.
> Currently this is only available on windows
## Preview

# Installation

# Config
Example:
```json
{
    "configs": [
        {
            "name": "Work"
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
### [Fields](FIELDS.md):

### Arguments/Flags
    `-d` or `--debug`: Starts the app in debug mode.
    `-c {num}` or `--config {num}`: The config that the app starts in. {num} is the index.

### Key
The keys are listed [here](KEYS.md).

# Contributing

## Building

# ToDo
- [ ] Help menu with `--help`
- [ ] Get path of config with flags
- [ ] Load different configs with flags
- [ ] Keybind to pin apps
- [ ] Add custom keybinds to a specific apps
- [ ] GUI
