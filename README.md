# Tauri/Yew Example

## Purpose

This is an example app demonstrating a `wasm` frontend built with [Yew](https://yew.rs) within the [Tauri](https://tauri.studio) framework.

## Build

```sh
cargo install wasm-pack 
cargo install --version 1.0.0-beta.5 tauri-cli
npm i --include=dev
make build
make package
make serve
```

### Chroot (WIP)

```sh
mkdir /mnt/aarch64-alpine
alpine-chroot-install -a aarch64 -d $HOME/aarch64-alpine -i /mnt/aarch64-alpine
$HOME/aarch64-alpine/enter-chroot
# as root in chroot
libs='webkit2gtk-dev libressl-dev'
dev_tools='git build-base rust cargo nodejs-current npm'
apk add $libs $dev_tools
useradd user
su - user
# as user in chroot
cargo install --version 1.0.0-beta.5 tauri-cli  # this will take a lot of time (62 mins on 9th gen intel i7)
git clone https://github.com/littlebenlittle/tauri-example.git
cd tauri-example
cargo tauri build  # this will also take a long time
```
