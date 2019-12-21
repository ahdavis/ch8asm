# ch8asm

Open-source Chip-8 assembler

# Overview

`ch8asm`, pronounced like the word "chasm", is a simple-to-use and 
easy-to-learn assembler for the Chip-8 retro video game platform. It
uses simple syntax and can easily be picked up in an afternoon or two.

# Installation

Enter this command in your terminal to install `ch8asm`:

```
cargo install ch8asm
```

# Usage

To use `ch8asm`, type `ch8asm <source file>` at your terminal, replacing
`<source file>` with the path to your Chip-8 source code file. Assuming
nothing goes wrong (errors are described below), you will see a message
saying that your file was assembled correctly. Then, you can run the
resulting binary with your emulator of choice. If you don't have a
Chip-8 emulator already, you can use my emulator, Cookie, which can be
found [here](https://github.com/ahdavis/cookie).

# Something went wrong...

If you do not see the message `Successfully assembled <source file> into
<binary file>` when you try to assemble your game, `ch8asm` will display
an error message saying what went wrong when assembling your binary. Here
are the possible error messages and their most likely fixes:

## "Could not get the address of the label XXXX"

This message means that you attempted to use the label `XXXX` without
defining it elsewhere in your program. Be sure to put a colon after the
definition of your label.

## "Bad argument for XXXX instruction"

This message means that you used the wrong register 
(usually the `I` register) in an instruction that doesn't support that
register. Be sure you are using the proper instruction arguments, as 
detailed below.

## "Unknown character X"

This message means that you accidentally put a character in your
source code that `ch8asm` doesn't recognize. Go to the line number
mentioned in the error message and look for a stray character.

## "Unknown instruction XXXX"

This message means that you accidentally misspelled an instruction or tried
to invent a new one. Go to the line number mentioned in the error message
and check your code for proper spelling.

## "Expected XXXX, found XXXX"

This message means that you have an out-of-place symbol in your code. Go
to the line number mentioned in the error message and correct your code
according to the error message.

## "Bad skip type XXXX"

This message means that you accidentally misspelled a condition for the
`SKIP` instruction. Check the table of valid conditions and correct your
code to use a valid condition.

## "Binary is too large"

This simply means that your game is larger than the available memory
on the Chip-8. Unfortunately, the only fix for this is to rewrite your
game to generate a smaller binary. Instructions for this are beyond the
scope of this document and may be found online.

# Processor Information

The Chip-8 processor has sixteen general-purpose registers, numbered from
`V0` to `VF`. Each of these registers can hold one byte, and can be
manipulated at will. However, register `VF` is used by multiple
instructions as a flag register, and should only be used for that purpose.

The Chip-8 also has one "index" register, denoted by `I`. Its main use
is addressing memory for data storage and sprite drawing operations. It
can hold a value ranging from 0 to 4095 inclusive.

The Chip-8 also has 4096 (0x1000) bytes of memory. Out of these, addresses
0x050 through 0x0A0 are reserved for the built-in character set, and
binaries are loaded into memory starting at address 0x200. All other 
addresses are free for use by your games.

The Chip-8 also has two onboard timing devices, called the *delay timer*
and the *sound timer*. Both can be set through code, and will count down
to 0 at 60Hz when set to a non-zero value. The delay timer can be read 
from, whereas the sound timer cannot. While the sound timer is nonzero,
a tone will be played.

# Data Types

`ch8asm`'s assembly language has four major data types. They are:

* Decimal literals (preceded by a `#` sign)
* Hexadecimal literals (preceded by a `$` sign)
* Binary literals (preceded by a `%` sign)
* Labels (preceded by a `_` sign)

Decimal and hexadecimal literals can be any value from 0 to 4095
inclusive, while binary literals may only be one byte long. One use
for binary literals is to use them to draw out your sprites. Arrange your
binary literals so the 1s represent the desired on-pixels of your sprite,
attach them to a label definition, and you can use that label elsewhere
in your code to represent your sprite.

Labels come in two varieties: *definitions* and *references*. Definitions
provide `ch8asm` with a name for a given position in your game's code,
while references allow definitions to be used in code to reference that
position. Definitions are always followed by a colon, while references
are never followed by a colon.

This example snippet draws a smiley face in the upper left-hand corner
of the screen:

```
;this is a definition
_spr:
	%00000000
	%01000010
	%00000000
	%01000010
	%00100100
	%00011000

mov I, _spr ;this is a reference
mov V0, #0
mov V1, #0
draw V0, V1, #6

```

As a side note, you can comment your code by using a semicolon. Comments
start after a semicolon and continue to the end of the line.

# Instructions

Here is a list of all instructions that `ch8asm` understands. All
instructions and registers are case-insensitive, and commas must be
inserted between arguments. In this documentation, general purpose
registers are indicated by `VX` and `VY`, the index register is indicated
by `I`, constant values are denoted by `NN` or `NNN`, depending on
the size of the constant, and labels are denoted by `_lbl`, where `lbl` is
the actual name of the label.

## Draw Instructions

* CLS - Clears the screen.
* DRAW *VX*, *VY*, *H* - Draws a sprite at the coordinates (*VX*, *VY*)
with a height of *H* pixels. The sprite is read bit-coded from memory  
starting at the memory location pointed to by `I`. To facilitate collision
detection, `VF` is set to 1 if any pixels were flipped from on to off and
to 0 otherwise.
* SCH *VX* - Sets `I` to point to the hex character corresponding to the
value of *VX*, which must be within the range of 0 to 15, inclusive.

## Control Instructions

* JMP *_lbl* - Causes program flow to unconditionally jump to *_lbl*
* CALL *_lbl* - Calls a subroutine at *_lbl*.
* RET - Returns control from a subroutine to its calling code
* SKIP.*condition* *args* - Skips the next instruction if *condition* is
true for *args*. See the table below for a list of valid *condition*s.
* JPC (*_lbl* or *NNN*) - Unconditionally jumps to the argument plus `V0`

### Skip Conditions

| Condition | Arguments         | Truth condition         |
|-----------|-------------------|-------------------------|
| `EQ`      | `VX`,(`VY` or NN) | Arguments are equal     |
| `NE`      | `VX`,(`VY` or NN) | Arguments are not equal |
| `KD`      | `VX`              | Key `VX` is pressed     |
| `KU`      | `VX`              | Key `VX` is not pressed |


## Operation Instructions

* MOV *dest*, *src* - Stores the value of *src* in *dest*. If *dest* refers
to the `I` register, then *src* must be a constant or a label. 
*src* cannot be the`I` register.
* ADD *dest*, (*VX* or *NN*) - Adds the second argument to the first
and stores the result in the first. If the second argument is a constant,
*dest* cannot be the `I` register. Furthermore, if an overflow occurs when
adding, register `VF` is set to 1. If no overflow occurs, then `VF` will
be set to 0.
* OR *VX*, *VY* - Sets *VX* to itself bitwise ORed with *VY*.
* AND *VX*, *VY* - Sets *VX* to itself bitwise ANDed with *VY*.
* XOR *VX*, *VY* - Sets *VX* to itself bitwise XORed with *VY*.
* SUB *VX*, *VY* - Sets *VX* to itself minus *VY*. `VF` is set to 0 when
a borrow occurs and 1 otherwise.
* SUBN *VX*, *VY* - Sets *VX* to *VY* minus *VX*. `VF` is set to 0 when
a borrow occurs and 1 otherwise.
* SHR *VX* - Stores the least significant bit of *VX* in `VF` and bitwise
shifts *VX* to the right by 1.
* SHL *VX* - Stores the most significant bit of *VX* in `VF` and bitwise
shifts *VX* to the left by 1.

## Miscellaneous Instructions

* RAND *VX*, *NN* - Stores a random byte bitwise ANDed with *NN* in *VX*.
* GDL *VX* - Stores the current value of the delay timer in *VX*.
* KEY *VX* - Stops execution until a key is pressed and stores it in *VX*.
* SDL *VX* - Sets the delay timer to *VX*.
* SND *VX* - Sets the sound timer to *VX*
* BCD *VX* - Stores the binary-coded decimal representation of *VX* in
memory starting at location `I`.
* RDP *VX* - Stores the values of registers `V0` through *VX* in memory 
starting at location `I`.
* RLD *VX* - Reads data from memory starting at location `I` into registers
`V0` through *VX*.

# Further Reading

You may wish to read the 
[Wikipedia page](https://en.wikipedia.org/wiki/CHIP-8) on the Chip-8
to obtain further information about the processor and its capabilities.

# License

`ch8asm` is licensed under the GNU General Public License, version 3.

# Closing Remarks

Feedback and suggestions are always appreciated, so if you have an idea,
just open a pull request or issue on GitHub. Thanks for using `ch8asm`, 
and happy programming!
