# Pangxie Loader

A crabby modloader for Spelunky 2 on the Switch.

Only supports Switch version 1.25, but significantly motivated and skilled parties could quite easily port
to new versions by updating the offset on `src/lib.rs` line 65.

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

(Alternatively use `cargo skyline install` to install from source)

#### Run With Logging

```
cargo skyline run
```

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
