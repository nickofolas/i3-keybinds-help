# i3-keybinds-help
I originally wrote a script for this functionality in Python, but decided to rewrite it in Rust for the sake of learning the language.<br>
I have no expectation of this really being useful to anyone, but in the off chance that it is, I may as well provide a *bit* of explanation.

## What it is
The program parses an i3 config file (which it tries to auto-detect), and then spits out a list of any keybindings that have associated documentation. The list is formatted such that it can be piped to dmenu (that is, it's joined by newlines) - I use it with rofi.

A standard "documented keybinding" looks like:<br>
```
# ## Documentation goes here ##
bindsym $mod+Shift+s *some exec call...*
```
<br>
The program will only extract keybinds with that exact format, so leave any "undocumented" bindings the way they are.

## Installation
`cargo install --git https://github.com/nickofolas/i3-keybinds-help --branch main i3-keybinds-help`
