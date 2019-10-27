# Rust8
A Rust CHIP-8 emulator

Yes, yet another CHIP-8 emulator, mostly written to learn RUST, SDL2 and how an emulator works.

# TODO
* Sound is missing, it's just a simple beep but seems hard to add in SDL2, I'll look at some tutorials later.
* Code is super messy, #wontfix lol.
* I'll add some screenshots later.

# Requirements
## SDL2
I included the standard build.rs scripts so it can work in Windows but you better 
follow the instructions in the [Rust-SDL2 repository](https://github.com/Rust-SDL2/rust-sdl2)

# Use
Are you really going to use this emulator? I mean there's like hundreds out there just google them ... seriously? Ok, well using cargo you can load a ch8 ROM using:
> cargo run pong.ch8

There are multiple ROM collections out there I added some in the references section.

# References
- http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/ Very useful and concise article
- http://devernay.free.fr/hacks/chip8/C8TECH10.HTM Nice in depth guide to know what all the opcodes do
- https://github.com/starrhorne/chip8-rust When all things failed I checked this other Rust emulator
- [BC Test ROM](https://slack-files.com/T3CH37TNX-F3RKEUKL4-b05ab4930d) and the [BC Test ROM Guide](https://slack-files.com/T3CH37TNX-F3RF5KT43-0fb93dbd1f) a really useful test ROM that tells you some opcode logic
- https://github.com/dmatlack/chip8/tree/master/roms A ROM Collection
