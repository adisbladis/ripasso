# ripasso
[![Build Status](https://travis-ci.org/cortex/ripasso.svg?branch=master)](https://travis-ci.org/cortex/ripasso)


A simple password manager written in Rust

This is a reimplementation of https://github.com/cortex/gopass in Rust. I started it mainly because since https://github.com/go-qml/qml
is unmaintaned. Also, using a safe language for you passwords seems like a good idea. 

It has not yet reached feature-parity, but the basic functionality works. If this plays out well, it will replace gopass.

PRs are very welcome!

## Build instructions

### Mac OS X

```
$ brew update
$ brew install automake cmake qt5 gtk+3
$ export PATH="/usr/local/opt/qt/bin:$PATH"
$ git clone https://github.com/cortex/ripasso.git
$ cd ripasso
$ cargo run
```

### Ubuntu
```
$ apt install cargo libgtk-3-dev qtdeclarative5-dev libqt5svg5-dev cmake
$ cargo build
```

### Arch
```
$ pacman -S qt5-base qt5-svg qt5-declarative
```
