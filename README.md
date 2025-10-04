# hypr-stay

A simple tool for Hyprland that would make your mouse stay inside the active window if said window cannot grab your mouse by itself.

Every time the mouse tries to get out of the window, it gets moved to the opposite side, which idk if it's useful for LoL but it works a lot better with FPS games.

All you need to do is just:
```
cargo build --release
```
and then
```
sudo cp target/release/hypr-stay /usr/bin/
```
And add the binding into your config file:
```
bindd = SUPER ALT, G, exec, killall hypr-stay || hypr-stay
```

Based on [hyprland-cursor-lock](https://gitlab.com/mytdragon/hyprland-cursor-lock) which is in turn based on [hyprland-lol-silly-workaround](https://github.com/BKSalman/hyprland-lol-silly-workaround)
