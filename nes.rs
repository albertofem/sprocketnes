//
// sprocketnes/nes.rs
//
// Author: Patrick Walton
//

#![feature(link_args, macro_rules, globs)]
#![no_main]

extern crate native;
extern crate sdl;
extern crate libc;

// NB: This must be first to pick up the macro definitions. What a botch.
#[macro_escape]
pub mod util;

pub mod apu;
pub mod audio;
#[macro_escape]
pub mod cpu;
pub mod disasm;
pub mod gfx;
pub mod input;
pub mod main;
pub mod mapper;
pub mod mem;
pub mod ppu;
pub mod rom;

// C library support
pub mod speex;

#[no_mangle]
pub extern "C" fn SDL_main(argc: i32, argv: **u8) -> i32 {
    native::start(argc as int, argv, proc() main::start(argc, argv)) as i32
}

