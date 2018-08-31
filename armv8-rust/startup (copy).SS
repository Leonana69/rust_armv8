.weakref            Reset_Handler,defaultResetHandler

.section            isr_vector                  /* Put everything in a section called "isr_vector" from now on... */
.align              2                           /* Make sure the output goes on an address divisible by 4 (that's 1 << 2) */
/* Address:   Exception Vector Description: */
.long               0x0                           /* 0x00000000 The initial stack pointer (defined by the linker-script) */
.long               Reset_Handler               /* 0x00000004 The startup-code, the code that runs on power-on or RESET */

.macro             FUNCTION name                /* this macro makes life less tedious. =) */
.func              \name,\name                  /* this tells a debugger that the function starts here */
.type              \name,%function              /* when a function is pointed to from a table, this is mandatory */

.align                                          /* make sure the address is aligned for code output */
\name\():                                                           /* this defines the label. the \() is necessary to separate the colon from the label */
.endm

.macro             ENDFUNC name                 /* FUNCTION and ENDFUNC must always be paired */
.size              \name, . - \name             /* tells the linker how big the code block for the function is */
.pool                                           /* let the assembler place constants here */
.endfunc                                        /* mark the end of the function, so a debugger can display it better */
.endm

.text
.align
FUNCTION            defaultResetHandler
adrp x0, stacktop
mov sp, x0
b rust_main
ENDFUNC             defaultResetHandler

FUNCTION            defaultExceptionHandler
wfi                                             /* wait for an interrupt, in order to save power */
b                   defaultExceptionHandler     /* go round loop */
ENDFUNC             defaultExceptionHandler
