
all:
	rustc rust-sdl/src/sdl/lib.rs
	rustc --crate-type=lib graphics_sdl.rs -L.
	rm *.rlib

clean:
	rm *.rlib
