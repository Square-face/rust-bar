# Hybrid Bar
A status bar focused on wl-roots Wayland compositors

## Preview
With blur through Hyprland.
![image](https://user-images.githubusercontent.com/54314240/185250795-b5c1b948-ef69-4293-bd1b-4deedbbaa82d.png)

## What does it support?
It supports:
- Labels (static and inherit text from a command);
- Buttons with `on click` actions via a bash command;
- Transparency (+ blur if your compositor supports it);
- Custom CSS

A.K.A It's a simple status bar I put together with gtk-layer-shell and GTK 3 because I couldn't be bothered with more weird, half-broken bars.
## I have no config
You have to make one yourself for now, until Hybrid has a default example config available.

```sh
mkdir -p ~/.config/HybridBar
cd ~/.config/HybridBar
touch config.json
```

Now edit the config in your favorite text-editor (better be NeoVim) and begin configuring it.
# Config Layout
I'm assuming you are familiar with JSON, if you aren't, well too bad.
## Base
Before you can use the bar, you have to adjust the color and alpha.

Here's an example:

```json
{
    "background": {
        "r": 10,
        "g": 10,
        "b": 10,
        "a": 0.5
    }
}
```
## Widgets
Available widgets:

Labels:
- left-label: Left-aligned label;
- centered-label: Centered label
- right-label: Right-aligned label

Optional Keys Supported:
- text: String
- command: String

Buttons:
- left-button: Left-aligned button
- centered-button: Centered button
- right-button: Right-aligned button

Optional Keys Supported:
- text: String
- command: String

Spacing:
- left-spacing: Left-focused spacing
- centered-spacing: Centered-focused spacing
- right-spacing: Right-focused spacing

Optional Keys Supported:
- spacing_start: i32
- spacing_end: i32

To actually use a widget, here's an example:

```json
"left-label_UNIQUE_NAME": {
        "text": "whomai stdout ",
        "command": "whoami"
    }
```

Every widget **has** to end with `_`, then you add the unique name for that widget.

The `text` and `command` nested JSON keys are simply described as:
- text: Raw Label Text
- command: Optional bash command

**All** keys are optional, if you skip `text` for example, it'll be using an empty value.

No, the unique name isn't actually displayed anywhere, it's just to be able to differ each component from another.
## Video Tutorial
You can watch a video tutorial made by Foren [here](https://www.youtube.com/watch?v=5g7MX3jgv8A)
## Example Config
Made for [Hyprland](https://github.com/hyprwm/Hyprland).

```json
{
    "background": {
        "r": 10,
        "g": 10,
        "b": 10,
        "a": 0.5
    },
    "left-label_username": {
        "text": "user: ",
        "command": "whoami"
    },
    "left-label_current_workspace": {
        "text": " | workspace: ",
        "command": "hyprctl monitors -j | jq -r \".[].activeWorkspace.id\""
    },
    "left-label_max_workspaces": {
        "text": "/10"
    },
    "centered-label_active_window": {
        "command": "hyprctl activewindow -j | jq -r \".title\""
    },
    "right-label_volume": {
        "text": " | vol: ",
        "command": "echo $(pactl get-sink-volume @DEFAULT_SINK@ | rg -o '[0-9]{1,3}%' | head -n 1 | cut -d '%' -f 1)%"
    },
    "right-label_time": {
        "text": " | time: ",
        "command": "date +%H:%M" 
    },
    "right-spacing_end": {
        "spacing_end": 5
    }
}
```
## CSS Support
Starting from `0.1.3`, CSS is now supported and you can make it auto-load on startup by making a `style.css` file next to your `config.json` at the same path.

If you want a sample CSS which has been used up until now, check `examples/style.css`.

Since `0.1.4`, you can now also style separate labels and buttons.

For example: `left-label_username_stuff` can be styled using CSS via `#username_stuff { /* Code */ }`.
## Environment Variables
`HYBRID_LOG` = `0 OR 1` : Logs debug output to stdout.

`HYBRID_POS` = `TOP OR BOTTOM` : Tells the bar where to position itself, TOP or BOTTOM.

`HYBRID_CONFIG` = `name.json` : Locates the config inside the HybridBar config path, then uses it for the rest of the bars session.
# Installation
Dependencies:

1. gtk-layer-shell
2. gtk3
3. bash

## Arch Linux
AUR: `paru -S hybrid-bar-git`

**NOTE**: This builds the bar, so you'll need Rust installed.
## Building
1. `git clone https://github.com/vars1ty/HybridBar`
2. `cd HybridBar`
3. `cargo build --release`
4. `cd target/release`
5. Run the `hybrid-bar` executable.
