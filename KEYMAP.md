# Example:
```json
{
    "keymaps": {
        {
            "keys": ["'"],
            "modifiers": ["LCtrl"],
            "event": "AddApp"
        }
    }
}
```

## Modifiers:
- R/LCtrl
- R/LAlt
- Shift
- R/LShift

## Events:
### "OpenApp":

The event to spawn a new app to given a number.
If no number is provided (or 0), it will spawn the current focused app (if in the list).

### "OpenApp{number}"

Opens the app at the given index.

### "AddApp"

Adds the current focesed app to the current config's app list.
If a number is provided (not 0), then it will insert the app into that index and shift all others to the right.

### "IncrementConfig":

Increments the current config

### "DecrementConfig":

Increments the current config

### "IncrementSetConfig":

Sets the current config to givent a number.
If no number is provided (or 0), it will incement the current config.

### "DecrementSetConfig":

Sets the current config to givent a number.
If no number is provided (or 0), it will decement the current config.

### "SetConfig{number}":

Sets the config to the given index.

### "DebugClose":

Closes the app if in debug mode.

# Defaults:
### "OpenApp":
`{number} LCtrl + ,`

### "OpenApp{number}":
`None`

### "AddApp":
`{number} LCtrl + '`

### "IncrementConfig":
`None`

### "DecrementConfig":
`None`

### "IncrementSetConfig":
`{number} LCtrl + ``

### DecrementSetConfig":
`None`

### "SetConfig{number}":
`None`

### "DebugClose":
`RCtrl + RAlt`

