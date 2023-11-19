# passmr
A lightweight CLI tool for managing key-value.

# Install
1. Install binary from [v1.1.0 release](https://github.com/harperfu6/passmr/releases/tag/v1.1.0)
- Linux: passmr_v1.1.0_x86_64-unknown-linux-musl.tar.gz
- Mac OS: passmr_v1.1.0_x86_64-apple-darwin.zip
2. decompress file, then move binary to a $PATH dir.

e.g. (Linux ver)
```
$ tar -xzvf passmr_v1.1.0_x86_64-unknown-linux-musl.tar.gz
$ mv passmr $HOME/.local/bin/
```

NOTE: For Mac OS ver, you have to permit "Allow Anyway" from "Security & Privacy" Setting.

# How to use passmr

**launch passmr**
```
$ passmr
```

## Mode
- add mode: add key-value
- search mode: search key
	- select mode: select key, then copy/delete/edit value

press 'q' to quit.

## add mode
You can add new key-value. Press 'a' to enter add mode.

NOTE: All key-values are stored in `$HOME/.passmr/kvs` dir.
You can also create another KVS by (temporarily) moving the folder as `mv $HOME/.passmr/kvs $HOME/.passmr/kvs-old`.

## search mode
You can search key-value you added. Press 's' to enter search mode.

### select mode
In search window, press any word you want to search, then press 'Enter' to enter select mode.

You can select key by ↑/↓ or k/j.

**copy value**

You can copy value of key you select. Press 'Enter' to copy (to clipboard!).

**delete key-value**

You can delete key (and value) you select. Press 'd' to delete.

**edit key-value**

You can edit (only) value of key you select.  Press 'e' to edit.

NOTE: If you want to edit a key, you need to delete it and re-add it.

# Note
On Linux, you'll need to install xorg-dev and libxcb-composite0-dev to compile. 
```
sudo apt install xorg-dev libxcb-composite0-dev
```

# Todo
