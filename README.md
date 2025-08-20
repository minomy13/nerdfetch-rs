<p align="center">
  <img src="icon.svg" alt="nerdfetch-rs icon" width="160" style="display: block; margin-left: auto; margin-right: auto">
  
  <br/>
  
  <img src="https://img.shields.io/github/license/minomy13/nerdfetch-rs?style=for-the-badge" alt="license">
  <img src="https://img.shields.io/github/v/tag/minomy13/nerdfetch-rs?style=for-the-badge&label=version" alt="version">
  
  <br/>

  <img alt="AUR Version" src="https://img.shields.io/aur/version/nerdfetch-rs?style=for-the-badge&link=https%3A%2F%2Faur.archlinux.org%2Fpackages%2Fnerdfetch-rs&logo=archlinux">
  <img alt="Crates.io Version" src="https://img.shields.io/crates/v/nerdfetch-rs?style=for-the-badge&link=https%3A%2F%2Fcrates.io%2Fcrates%2Fnerdfetch-rs">

  <br/>
</p>

# nerdfetch-rs

<br/>

> A minimal, fast system fetch tool — rewritten in Rust for speed, clarity, and configurability.

<br/>

![Alt](https://repobeats.axiom.co/api/embed/2e201bc93ef94da91317609669ec0b33a688d6a9.svg "Repobeats analytics image")

**Acknowledgements:** Heavily inspired by [NerdFetch](https://github.com/ThatOneCalculator/nerdfetch) by ThatOneCalculator.
This project is a Rust-based reimagining with a similar aesthetic and minimalistic goals.

## Config

You can find the config file at `$HOME/.config/nerdfetch-rs.toml`. As the suffix might suggest, it is in TOML format.

### Modules

Every module has its own table named like `[modules.module_name]`. Every module can be enabled or disabled
with the `active` key, which is a boolean. Some default modules are enabled by
default and must be disabled explicitely.

- **Default:** `color_palette` `kernel` `memory` `os` `shell` `uptime` `user`
- **Others:** `host`

#### Module-Specifics

##### Kernel

| Key       | Value                                                 |
| --------- | ----------------------------------------------------- |
| show_name | Show name of kernel. Otherwise just version is shown. |

##### Host

📝 **Note:** Activating this module may have a minor performance impact on macOS, as it spawns a child process.

### Theme

The following keys can be set in the `[theme]` table.

| Key        | Value                                                                  |
| ---------- | ---------------------------------------------------------------------- |
| ascii_art  | (Multiline-) String of ASCII art displayed to the left of modules.     |
| art_color  | Color of whole ASCII art.                                              |
| icon_color | Color of module icons.                                                 |
| info_color | Color of module text.                                                  |

### Colors

`white` `black` `red` `green` `yellow` `blue` `magenta` `cyan` `bright_red` `bright_green` `bright_yellow` `bright_blue` `bright_magenta` `bright_cyan`

## Installation

This software should work for most Linux distros and macOS. You **need** to have a [Nerd Font](https://www.nerdfonts.com)
installed and enabled in your terminal.

### Package manager

- **AUR.** `yay -S nerdfetch-rs`
- **Homebrew.** We have to get a lot more popular to make this possible. Please use *crates.io* in the meantime.
- **NIX.** Coming soon.
- **crates.io.** `cargo install nerdfetch-rs`. Rust toolchain required. (Debian, Red Hat, macOS, etc.)
- **Build from source.** Clone this repository and run `cargo build --release`. You'll find your binary in the `target/release` directory. Rust toolchain required.
