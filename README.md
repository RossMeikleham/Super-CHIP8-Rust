CHIP-Rust
==========

An implementation of a CHIP 8 emulator in Rust, created as an exercise for learning the Rust language.


Dependencies
===========
* SDL 1.2 development libraries
* Latest Rust version from [master branch](https://github.com/rust-lang/rust)



Instructions
============
If using Windows you will require Cygwin or another tool
which can execute makefiles.

To download and build:
```
git clone https://github.com/RossMeikleham/SCHIP-Rust.git
cd SCHIP-Rust
git submodule update --init
make
```

Running the emulator:
```
./schip [game]
```

Keys
====

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

Tests
=====
To run unit tests:
```
$./core_tests
```
> ***Note*** If running tests ensure environment variable RUST_TEST_TASKS is set to 1 as tests need to be run sequentially.


Screenshots
==========
CHIP 8 :
![Invaders](/images/invaders.png?raw=true) ![Pong](/images/pong.png?raw=true)
Super CHIP 8:
![Car](/images/car.png?raw=true) ![ANT](/images/ant.png?raw=true)

Resources
========
http://www.chip8.com/?page=109 for a collection of free demos and games.
