# Base Directory

See [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir/latest/)

Regardless of the platform, check the environment variable `XDG_XYZ_ABC`,
If it exists, use the folder defined by the environment variable,
Otherwise, use the platform's default folder

All paths set in these environment variables must be absolute. If an implementation encounters a relative path in any of
these variables it should consider the path invalid and ignore it.

> Variables that may be used:
>
> + `{USER}`: Username (`Unix-like`: environment variable `USER`,
    `Windows`: environment variable `USER_NAME`)
> + `{UID}`: User ID (Only in `Unix-like`)
> + `{APP_NAME}`: Application Name
> + `{APPDATA}`: environment variable `APPDATA` (Only in `Windows`)
> + `{LOCALAPPDATA}`: environment variable `LOCALAPPDATA` (Only in `Windows`)

## User-specific Data

XDG: `XDG_DATA_HOME`:

+ `Unix-like`: `~/.local/share/{APP_NAME}`
+ `Windows`: `{APPDATA}/{APP_NAME}/data`
+ `MacOS`: `~/Library/Application Support/`

## User-specific Configuration

XDG: `XDG_CONFIG_HOME`

+ `Unix-like`: `~/.config/{APP_NAME}`
+ `Windows`: `{APPDATA}/{APP_NAME}/config`
+ `MacOS`: `~/Library/Preferences/`

## User-specific State Data

XDG: `XDG_STATE_HOME`:

+ `Unix-like`: `~/.local/state/{APP_NAME}`
+ `Windows`: `{LOCALAPPDATA}/{APP_NAME}/state`
+ `MacOS`: `~/Library/Application Support/`

## User-specific Cached Data

XDG: `XDG_CACHE_HOME`:

+ `Unix-like`: `~/.cache/{APP_NAME}`
+ `Windows`: `{LOCALAPPDATA}/{APP_NAME}/cache`
+ `MacOS`: `~/Library/Caches/`

## User-specific Runtime Files

XDG: `XDG_RUNTIME_DIR`:

+ `Linux`: `/run/user/{UID}`
+ `Windows`: `%TEMP%`
+ `MacOS`: `$TMPDIR`
+ `Unix-like`: `/var/run/user/{UID}`

## System Runtime Files

> Similar to this `rust`: [`std::env::temp_dir()`](https://doc.rust-lang.org/std/env/fn.temp_dir.html)

+ `Linux`: `/tmp`
  + `Android`: Decided by the system
+ `Windows`: `GetTempPath2`/`GetTempPath`
+ `MacOS`: `confstr(_CS_DARWIN_USER_TEMP_DIR, ...)`
+ `Unix-like`: `/tmp`