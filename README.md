# Pindow
**[About](#About)**
**[Installation](#Installation)**
**[Config](#Config)**
**[Config](#Contributing)**

---
Pindow as a windows app that allows you to have a list of "pinned" apps and open each one with one keybind (`{num} + keybind`).

# About
---
## Preview

# Installation
---

# Config
---
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
### Key
The keys are listed [here](KEYS.md).

# Contributing
---

## Building

# ToDo
---
- [ ] Help menu with `--help`
- [ ] Get path of config with arguments
- [ ] Load different configs with arguments
- [ ] Keybind to pin apps
- [ ] Add custom keybinds to a specific apps
- [ ] GUI
