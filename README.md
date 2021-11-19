# Pangxie Loader

A crabby modloader for Spelunky 2 on the Switch.

Latest downloads can be found in [Releases](https://github.com/jam1garner/pangxie-loader/releases).

## Build From Source

#### Build Dependencies

* Rust
* cargo-skyline
* DevkitPro aarch64 toolchain (with `aarch64-none-elf-gcc` in path)
* git

#### Runtime Dependencies

* [Skyline](https://github.com/skyline-dev/skyline)
* [Atmosphère](https://github.com/Atmosphere-NX/Atmosphere)

#### Build

```
cargo skyline build --release
```

(resulting skyline plugin will be in `target/aarch64-skyline-switch/release/libpangxie_loader.nro`)

## Install

1. Copy skyline (`subsdk9` and `main.npdm`) to `sd:/atmosphere/contents/01007EC013ABC000/exefs`
2. Copy `libpangxie_loader.nro` to `sd:/atmosphere/contents/01007EC013ABC000/romfs/skyline/plugins`

## Add Mods

Mods go into `sd:/spelunky2/mods`, with each subfolder representing a modpack, similar to modlunky.

For example:

```
spelunky2 
  L mods 
    L remix_mod 
      L Data 
        L Levels
        L Textures
```
