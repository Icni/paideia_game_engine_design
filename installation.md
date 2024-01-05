# Installation

## Installing Rust

### Mac / Linux

Run the following in your command line:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Windows

Use one of the installers from [this website](https://forge.rust-lang.org/infra/other-installation-methods.html#other-ways-to-install-rustup).

## Installing SDL

### Linux

If you want to use Homebrew (a package manager), type

```sh
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

into your terminal and then run

```sh
brew install sdl2
```

If you already have a package management tool you like, stick with that. Otherwise, go to [this link](https://github.com/libsdl-org/SDL/releases/tag/release-2.28.5).

### Mac

Install Homebrew if you haven't already from [this link](https://github.com/Homebrew/brew/releases/latest).

Then run:

```sh
brew install sdl2
```
