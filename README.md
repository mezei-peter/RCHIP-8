# RCHIP-8: A CHIP-8 emulator/interpreter written in Rust

## Introduction
The aim of this personal project is to create and **interpreter and an emulated system** so that **binary .ch8 files turn into runnable programs**.

CHIP-8 is an interpreted programming language. It is compiled into binary .ch8 files containing 2-byte long instructions that get interpreted by the CHIP-8 interpreter which gives instructions to the computer's processor. The instructions are in big-endian byte order. Some examples:
- 00E0 - Clear screen
- 1NNN - Set the program counter(16-bit register) to NNN(12-bit memory address)
- 6XNN - Set the *x*th variable register(an 8-bit register) to NN(8-bit value)
- 7XNN - Add to the *x*th variable register(an 8-bit register) the value NN(8-bit value)
- ANNN - Set the index register(16-bit register) to NNN(12-bit memory address)
- DXYN - Draw to the screen an N byte long(N pixel tall) sprite starting from the memory address stored in the index register(16-bit register) at coordinates stored by the *x*th and *y*th variable registers(8-bit registers). The screen is 64\*32 pixels, so the coordinates fit in 1 byte each.

Currently, the main part of the emulator is finished and it runs programs. However, there are some breaking bugs that need to be fixed (To-do list at the bottom of this document).

## Demo
corax89's test ROM remade by Timendus.

![screenshot of running a test rom](https://i.imgur.com/c7TDeSP.png)

Pong.

![screenshot of RCHIP-8 running Pong](https://i.imgur.com/3VLwf3B.png)

## Requirements

### vcpkg
The project uses vcpkg to link SDL2 statically. Install and build vcpkg with: 
```
cargo install cargo-vcpkg
cargo vcpkg build
cargo build
```
**The vcpkg build procedure may take around 5 minutes.**

## Usage
Running RCHIP-8 requires an input file. I provided some ROMs in the *roms* folder:
```
cargo run -- roms/IBM_Logo.ch8
```
There are differences between the original *COSMAC VIP* interpreter and modern interpreters. You can configure these options with the **--config** argument after the file path argument.
```
cargo run -- roms/IBM_Logo.ch8 --config
```
## Implemented features
- Reading input file
- Emulating a guest system
- Fetch-Decode-Execute cycle
- Display window with SDL2
- Taking input with SDL2
- Configurable old vs modern behaviour

## To-do list
- Adding sound
- The display often times gets broken and needs to be fixed.
- Key input regarding the *FX0A* instruction is broken and needs to be fixed.
- Code refactoring
