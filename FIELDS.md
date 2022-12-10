```
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
```
