# Simply Script

This project provides an interpreter for "simply script", a _super-simple_ scripting language
(inspired by Assembly) that is mostly used for coding puzzles.

## Syntax

"simply script" supports the following commands:

### Set Command

_set |register| |value|_

Sets the value (32-bit integer) of the indicated register. Register names can be any
combination of letters, numbers, and underscore characters, but must start with a letter.
This command essentially serves the purpose of assigning a value to a variable.

### Copy Command

_cpy |register 1| |register 2|_

Copies the value from |register 1| to |register 2|.

### Add Command

_add |register 1| |register 2|_

Adds the value stored in |register 1| to the value stored in |register 2|. Stores the
final value in |register 2|.

### Sub Command

_sub |register 1| |register 2|_

Subtracts the value stored in |register 1| from the value stored in |register 2|. Stores
the final value in |register 2|.

### Jump Command

_jmp |register|_

The next line to execute will be the line whose value is stored in |register|. Attempting
to jump to a line number less than `1` will result in a `NegativeExecutionPointer` error.

### Jump When Zero Command

_jwz |register 1| |register 2|_

Check the value in |register 1|. If that value is `0`, continue execution at the line 
number stored in |register 2|. Otherwise, continue execution with the next line.

### Jump When Negative Command

_jwz |register 1| |register 2|_

Check the value in |register 1|. If that value is less than `0`, continue execution 
at the line number stored in |register 2|. Otherwise, continue execution with the next 
line.

### Jump When Positive Command

_jwz |register 1| |register 2|_

Check the value in |register 1|. If that value is greater than `0`, continue execution at 
the line number stored in |register 2|. Otherwise, continue execution with the next line.

### Jump Not Zero Command

_jnz |register 1| |register 2|_

Check the value in |register 1|. If that value is *not* `0`, continue execution at the 
line number stored in |register 2|. Otherwise, continue execution with the next line.

### Greater Than Command

_gth |register 1| |register 2|_

If the value stored in |register 1| is greater than |register 2|, store `1` in 
|register 2|, otherwise, store `-1` in |register 2|.

### Less Than Command

_lth |register 1| |register 2|_

If the value stored in |register 1| is less than |register 2|, store `1` in 
|register 2|, otherwise, store `-1` in |register 2|.

### Output Command

_out |register|_

Print the value stored in |register| to standard out.

## Usage

Just write your "simply script" into a text file (like "do_calculation.ok") and run the 
script with `simply do_calculation.ok`.

### Examples

#### Print Greater Number

Sets input values to `num1` and `num2`, then identifies and returns the greater number.
Just returns one of the numbers if they are equal. By convention, registers holding 
line numbers for jumps are named "Excel-style" using capital letters (A-Z, AA-ZZ, etc.).

```
set num1 18
set num2 15
set A 10
set B 11
cpy num2 compare
gth num1 compare
jwp compare A
out num2
jmp B
out num1
```