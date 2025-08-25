# lazyclicker.rs
a *lazy* autoclicker for unix systems that support multiple profiles and holding!!  
this is the second version of the autoclicker i made a while ago: [herehere](https://github.com/lunar1um/autoclicker)  

#### expectations
i have only tested on arch linux, no idea about other os, but they should work!!  
also i tested global keybinds on hyprland, so it varies for your wm / compositor too!!!  

#### features
- multiple profiles (each with custom configuration and mode)
- working as a background process
- global keybinds (yippee)
- two modes: holding and clicking

#### setting up!
1. Head to [Releases](https://github.com/lunar1um/autoclicker.rs) and grab the latest Release!
2. Move the *binary file* (lazyclicker) to somewhere within your **PATH**
3. Run `lazyclicker init` for the first time and it will create a sample configuration file (often at `~/.config/lazyclicker`)

#### configurations
as this uses [toml](https://toml.io/) as the configuration language, so it's super easy to configure!!  

Example `profiles.toml`:
```toml
[[profile]]
name = "clicktest"
interval = 1
button = "Left"
repeat = 1
mode = "Click"

[[profile]]
name = "holdtest"
button = "Left"
mode = "Hold"
```

- `name`: the name of the profile
- `mode`: clicking mode (can be either `Click` or `Hold`)
- `button`: mouse button to click (can be either `Left` or `Right`)
- `interval`: time between clicks (in seconds)
- `repeat`: numbers of repeat each click

> [!NOTE]
> it only accepts the uppercased Left, Right, Click, and Hold

> [!NOTE]
> `interval` and `repeat` is optional for `Hold` mode, and compulsory for `Click` mode

#### commands
1. `lazyclicker init`: Initialize the sample configuration file and path
2. `lazyclicker list`: List all available profiles
3. `lazyclicker start [PROFILE_NAME]`: Start / Run a profile
4. `lazyclicker stop [PROFILE_NAME]`: Stop a running profile