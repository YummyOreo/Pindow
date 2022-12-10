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
### Fields:
`configs`:
    > Required

    The configs of the app.

    `name`:
        > Defaults to the index

        The name of the config.

    `apps`:
        > Defaults to empty list

        The apps

        `app_path`:
            > Required

            The path of the app

        `args`:
            > Defaults to empty list

            The Arguments/flags to start the app with

    `timeout`:
        > Defaults to 5
        > In seconds

        The timeout for number presses

    `keybindings`:
        > Defaults to the default keybindings

        `app_num`:
            > Defaults to: `LCtrl + ,`
            > Usage: `{num} + keybinding`

            The keybinding to spawn a new app.
            If no number is provided (or 0), it will spawn the current focused app (if in the list).

        `change_config`:
            > Defaults to: `LCtrl + \``
            > Usage: `{num} + keybinding`

            Sets the current config.
            If no number is provided (or 0), it will spawn the current focused app (if in the list).

        `debug_close`:
            > Defaults to: `RCtrl + RAlt`

            Stops the app if in debug mode.

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
