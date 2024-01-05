# Installation

First, you need to figure out if your computer is a Linux, Mac, Windows, or
something else entirely.

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

On most Linux platforms, you can just use this to install SDL (if it's not
installed already):

```sh
sudo apt-get install libsdl2-dev
```

Otherwise, go to [this link](https://github.com/libsdl-org/SDL/releases/tag/release-2.28.5)
and download from there.

### Mac

Install Homebrew if you haven't already from [this link](https://github.com/Homebrew/brew/releases/latest).

Then run:

```sh
brew install sdl2
```

### Windows

Download from [this link](https://buildbot.libsdl.org/sdl-builds/sdl-visualstudio/?C=M;O=D).

## Installing Visual Studio Code

Just use [this link](https://code.visualstudio.com/download).

## VSCode Extensions

You only really to install `rust-analyzer`. In VSCode, go to the little grid-
shaped icon for the Extension Manager and search up `rust-analyzer`, then click `install`.