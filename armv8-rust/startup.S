
.section            isr_vector                  /* Put everything in a section called "isr_vector" from now on... */
.align              2                           /* Make sure the output goes on an address divisible by 4 (that's 1 << 2) */
/* Address:   Exception Vector Description: */
; .long               0x0                           /* 0x00000000 The initial stack pointer (defined by the linker-script) */
; .long               _start               /* 0x00000004 The startup-code, the code that runs on power-on or RESET */


.text
.align
_start:
adrp x0, stacktop
mov sp, x0
b rust_main
b .

