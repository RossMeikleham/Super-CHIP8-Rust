[![Build Status](https://travis-ci.org/RossMeikleham/Super-CHIP8-Rust.svg?branch=master)](https://travis-ci.org/RossMeikleham/Super-CHIP8-Rust)
# SCHIP-Rust

An implementation of a CHIP 8 emulator in Rust with extended Super CHIP 8 functionality. 
Created as an exercise to help me learn the Rust programming language.


# Dependencies

* SDL 1.2 development libraries
* Latest Rust version from [master branch](https://github.com/rust-lang/rust)
* Cargo (Rust Package Manager)



# Instructions

To download and build:
```
git clone https://github.com/RossMeikleham/Super-CHIP8-Rust.git
cd Super-CHIP8-Rust
cargo build
```

Running the emulator:
```
./schip8 [game]
```

# Keys


The following tables show the mappings from the hexidecimal CHIP key layout to an ordinary keyboard layout:

CHIP Keys

|C|D|E|F|                      
|---|---|---|---|
|**8**|**9**|**A**|**B**|
|**4**|**5**|**6**|**7**| 
|**0**|**1**|**2**|**3**| 

Keyboard Keys:

|1|2|3|4|                      
|---|---|---|---|
|**q**|**w**|**e**|**r**|
|**a**|**s**|**d**|**f**| 
|**z**|**x**|**c**|**v**| 

# Tests

To run unit tests:
```
cargo test
```
> ***Note*** If running tests ensure environment variable RUST_TEST_TASKS is set to 1 as tests need to be run sequentially.


# Screenshots

CHIP 8:

![Invaders](/images/invaders.png?raw=true)      ![Pong](/images/pong.png?raw=true)

Super CHIP 8:

![Car](/images/car.png?raw=true)      ![ANT](/images/ant.png?raw=true)


# Resources

http://www.chip8.com/?page=109 for a collection of free demos and games.
