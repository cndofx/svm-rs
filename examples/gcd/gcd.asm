IN ; read integer "A" from standard input
IN ; read integer "B" from standard input

:GCD ; this is a label
DUP ; if B is 0 then A is the gcd
0 ; (immediate values get pushed on the stack)
@END ; (this is how you put the address of a label onto the stack)
JE ; (this will jump to the address at top of stack if the preceding two values are equal)
SWP ; if B is not 0 then the result is gcd(B, A modulo B)
OVR 
MOD
@GCD
JMP ; recursion!

:END
POP ; remove 0 from top of stack 
OUT ; now the result is at the top, print it.