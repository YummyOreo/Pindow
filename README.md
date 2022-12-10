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
### Entrys:
<details>
<summary>configs</summary>
This is were you put your configs.

> Required

<details>
<summary>name</summary>
The name of your configs

> Default: index of the config

</details>

    <details>
    <summary>apps</summary>
    A list of all your apps

    > Defaults to empty list

        <details>
        <summary>app_path</summary>
        The path of the app (can be a command)

        > Required
        </details>

        <details>
        <summary>args</summary>
        The arguments passed when spawing the app.

        > Defaults to empty list
        </details>

    </details>

    <details>
    <summary>timout</summary>
    The timeout for the numbers

    > In seconds
    > Default: 5 seconds
    </details>

    <details>
    <summary>keybindings</summary>
    Change the default keybindings

    > Defaults to the default keybindings

        <details>
        <summary>app_num</summary>
        The keybinding that you press to open a app.
        If there is no provided number (or 0), it will spawn a new app with the current focused app if it is in the list.

        > Usage: `{num} + keybinding`
        > Defaults to `LCtrl + ,`
        > See [keys](KEYS.md)
        </details>

        <details>
        <summary>change_config</summary>
        The keybind that you press to change your current config.
        If there is no number provided (or 0), it will increment through the configs.

        > Usage: `{num} + keybinding`
        > Defaults to `LCtrl + \``
        > See [keys](KEYS.md)
        </details>

        <details>
        <summary>debug_clone</summary>
        Closes the app if in debug mode.

        > Usage: `keybinding`
        > Defaults to RCtrl + RAlt`
        > See [keys](KEYS.md)
        </details>

    </details>

</details>

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
