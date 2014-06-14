
all:
	rustc core/rust-sdl/src/sdl/lib.rs
	rustc --crate-type=lib core/graphics_sdl.rs -L.
	rustc main.rs -L.
	rm *.rlib

clean:
	rm *.rlib
