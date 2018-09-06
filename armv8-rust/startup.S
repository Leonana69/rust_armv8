.section ".text.boot"

.global _boot_core

_boot_core:
    // read cpu id
    mrs     x1, mpidr_el1
    and     x1, x1, #3
    cbz     x1, 2f
    // cpu id > 0, stop
1:  wfe
    b       1b
2:  // cpu id == 0
    // set stack, this is essential for armv8
    ldr     x1, =stack_top
    add     x1, x1, #4
    mov     sp, x1

    
    // should not return
    bl      reset
    b       1b
