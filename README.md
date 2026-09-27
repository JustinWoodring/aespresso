# aespresso
A GTK4 frontend for Arch Linux's `archlinux-java` script.

### Installation:

Build from source:

```
cargo build --release
```

The AUR package is here: [aespresso](https://aur.archlinux.org/packages/aespresso/)

### Dependencies:

- `gtk4` (runtime libraries)
- `polkit` (for `pkexec`) or `lxqt-sudo`, used to run `archlinux-java` as root
- `archlinux-java`, provided by `java-runtime-common`

### Running:

Use the application menu. Wait, sorry you're an Arch user so you probably don't use those. ; )

For my fellow Archers faster with a terminal than an application menu: `aespresso`

![aespresso screenshot](https://github.com/JustinWoodring/aespresso/blob/master/aespresso-screenshot.png)
