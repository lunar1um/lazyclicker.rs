# lazyclicker.rs
a *lazy* autoclicker for unix systems that support multiple profiles and holding!!  
this is the second version of the autoclicker i made a while ago: [herehere](https://github.com/lunar1um/auto-clicker)  

#### expectations
i have only tested on arch linux, no idea about other os, but they should work!!  
also i tested global keybinds on hyprland, so it varies for your wm / compositor too!!!  

#### features
- multiple profiles (each with custom configuration and mode)
- working as a background process
- global keybinds (yippee)
- two modes: holding and clicking

#### setting up!
1. Head to [Releases](https://github.com/lunar1um/lazyclicker.rs/releases) and grab the latest Release!
2. Move the *binary file* (lazyclicker) to somewhere within your **PATH**
3. Run `lazyclicker init` for the first time and it will create a sample configuration file (often at `~/.config/lazyclicker`)

#### configurations
as this uses [toml](https://toml.io/) as the configuration language, so it's super easy to configure!!  

Example `profiles.toml`:
```toml
[[profile]]
name = "clicktest"
interval = 1000
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
- `interval`: time between clicks (in milliseconds)
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

#### troubleshooting
run `lazyclicker start [PROFILE_NAME] --run` and look for errors

`Error: NotFound`:
1. check if `/dev/uinput` exists: 

```sh
ls -l /dev/uinput
```

-> if it's missing: 
```sh
sudo modprobe uinput
```

-> make it permanent across reboots:
```sh
echo uinput | sudo tee /etc/modules-load.d/uinput.conf
```

2. fix perms

```sh
ls -l /dev/uinput
```

check the permissions of user, it should be something like `crw-rw-rw-`

if not,  
add user to the right group, or create a udev rule:

```sh
sudo groupadd input
sudo usermod -aG input $USER
```

then create `/etc/udev/rules.d/99-input.rules`:

```ini
KERNEL=="uinput", MODE="0660", GROUP="input"
```

and reload rules:

```sh
sudo udevadm control --reload-rules
sudo udevadm trigger
```
